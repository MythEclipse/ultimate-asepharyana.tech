'use client';

import { useState, useEffect, useRef, useCallback } from 'react';
import { useSession } from 'next-auth/react';
import useSWR from 'swr';
import ReconnectingWebSocket from 'reconnecting-websocket';
import { format } from 'date-fns';
import Card from '@/components/card/ThemedCard';
import { Textarea } from '@/components/text/textarea';
import Button from '@/components/button/NormalButton';
import { Loader2 } from 'lucide-react';
import Image from 'next/image';

type ChatMessage = {
  id: string;
  user: string;
  text?: string;
  email: string;
  imageProfile: string;
  role: string;
  timestamp: number;
  imageMessage?: string;
};

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function ChatClient() {
  const { data: session } = useSession();
  const { data: messages = [], mutate } = useSWR<ChatMessage[]>('/api/messages', fetcher, {
    refreshInterval: 30000,
    revalidateOnFocus: false,
  });
  const [inputValue, setInputValue] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [isSending, setIsSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [imageFile, setImageFile] = useState<File | null>(null);
  const [isUploading, setIsUploading] = useState(false);

  const wsRef = useRef<ReconnectingWebSocket | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const textAreaRef = useRef<HTMLTextAreaElement>(null);

  // WebSocket Management
  useEffect(() => {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const ws = new ReconnectingWebSocket(`${protocol}//ws.asepharyana.cloud`);

    wsRef.current = ws;

    const handleMessage = (event: MessageEvent) => {
      const processMessage = async () => {
        const parsedData = JSON.parse(event.data);
        const newMessage: ChatMessage = {
          id: parsedData.id,
          user: parsedData.user,
          text: parsedData.text,
          email: parsedData.email,
          imageProfile: parsedData.imageProfile,
          imageMessage: parsedData.imageMessage,
          role: parsedData.role,
          timestamp: parsedData.timestamp,
        };

        mutate((prev = []) => {
          if (prev.some((msg) => msg.id === newMessage.id)) return prev;
          return [...prev, newMessage];
        }, false);
      };

      processMessage();
    };

    const handleError = () => setError('Connection error. Reconnecting...');
    const handleClose = () => setIsConnected(false);
    const handleOpen = () => setIsConnected(true);

    ws.addEventListener('open', handleOpen);
    ws.addEventListener('message', handleMessage);
    ws.addEventListener('error', handleError);
    ws.addEventListener('close', handleClose);

    return () => {
      ws.removeEventListener('open', handleOpen);
      ws.removeEventListener('message', handleMessage);
      ws.removeEventListener('error', handleError);
      ws.removeEventListener('close', handleClose);
      ws.close();
    };
  }, [mutate]);

  // Auto-scroll dan textarea resize
  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  useEffect(() => {
    scrollToBottom();
    if (textAreaRef.current) {
      textAreaRef.current.style.height = 'auto';
      textAreaRef.current.style.height = `${textAreaRef.current.scrollHeight}px`;
    }
  }, [messages, inputValue, scrollToBottom]);

  // Handle file change
  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) setImageFile(file);
  };

  // Upload image
  const uploadImage = async (file: File) => {
    const formData = new FormData();
    formData.append('file', file);

    try {
      setIsUploading(true);
      const response = await fetch('/api/uploader', {
        method: 'POST',
        body: formData,
      });
      return await response.json();
    } finally {
      setIsUploading(false);
    }
  };

  // Handle send message
  const handleSend = useCallback(async () => {
    if ((!inputValue.trim() && !imageFile) || isSending) return;

    setIsSending(true);
    setError(null);

    try {
      let imageUrl = '';
      if (imageFile) {
        const { url } = await uploadImage(imageFile);
        imageUrl = url;
        setImageFile(null);
      }

      const tempId = Date.now().toString();
      const optimisticMessage: ChatMessage = {
        id: tempId,
        user: session?.user?.name || 'Anonymous',
        text: inputValue,
        email: session?.user?.email || 'anonymous@example.com',
        imageProfile: session?.user?.image || '/profile-circle-svgrepo-com.svg',
        role: 'user',
        imageMessage: imageUrl,
        timestamp: Date.now(),
      };

      // Optimistic update
      mutate((prev = []) => [...prev, optimisticMessage], false);

      // Send via WebSocket
      wsRef.current?.send(JSON.stringify({
        ...optimisticMessage,
        id: undefined,
      }));

      setInputValue('');
    } catch (err) {
      setError('Failed to send message');
      const tempId = Date.now().toString();
      mutate((prev = []) => prev.filter(msg => msg.id !== tempId), false);
    } finally {
      setIsSending(false);
    }
  }, [inputValue, session, isSending, imageFile, mutate]);

  // Keyboard handling
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className='container mx-auto py-8 px-4 max-w-2xl'>
      <h1 className='text-4xl font-extrabold text-gray-800 dark:text-gray-100 mb-8 text-center'>
        Chat Room
      </h1>

      <Card>
        <div className='flex items-center justify-between text-sm p-4 border-b border-blue-500 dark:border-blue-500'>
          <span className='text-blue-500'>Status:</span>
          <div className='flex items-center gap-2'>
            <div
              className={`w-2 h-2 rounded-full ${isConnected ? 'bg-green-500' : 'bg-red-500'}`}
            />
            <span className={isConnected ? 'text-green-500' : 'text-red-500'}>
              {isConnected ? 'Connected' : 'Connecting...'}
            </span>
          </div>
        </div>

        <div className='h-96 overflow-y-auto p-4 space-y-3'>
          {messages.map((message) => (
            <MessageBubble
              key={message.id}
              message={message}
              isOwn={message.email === session?.user?.email}
            />
          ))}
          <div ref={messagesEndRef} />
        </div>

        <div className="p-4 border-t border-blue-500 dark:border-blue-500">
          <div className="flex items-start gap-2">
            <Textarea
              ref={textAreaRef}
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Type a message..."
              className="flex-1 resize-none min-h-[40px]"
              rows={1}
              disabled={!isConnected}
            />
            <div className="flex items-center gap-2 h-full">
              <input
                type="file"
                onChange={handleFileChange}
                className="hidden"
                id="file-input"
                disabled={!isConnected || isUploading}
              />
              <label
                htmlFor="file-input"
                className={`h-10 px-3 py-2 flex items-center justify-center rounded-md text-sm border ${
                  isUploading
                    ? 'text-gray-400 border-gray-400'
                    : 'text-blue-500 border-blue-500 hover:bg-blue-50 dark:hover:bg-blue-900/20'
                } cursor-pointer transition-colors`}
              >
                {isUploading ? 'Uploading...' : '📎'}
              </label>
              <Button
                onClick={handleSend}
                disabled={!isConnected || isSending || isUploading}
                className="h-10 gap-1.5"
              >
                {isSending || isUploading ? (
                  <Loader2 className="w-4 h-4 animate-spin" />
                ) : (
                  'Send'
                )}
              </Button>
            </div>
          </div>
          {error && (
            <div className="text-red-500 text-sm mt-2 text-center">{error}</div>
          )}
        </div>
      </Card>
    </div>
  );
}

function MessageBubble({
  message,
  isOwn,
}: {
  message: ChatMessage;
  isOwn: boolean;
}) {
  return (
    <div className={`flex ${isOwn ? 'justify-end' : 'justify-start'}`}>
      <div
        className={`flex items-start gap-3 max-w-[85%] ${
          isOwn ? 'flex-row-reverse' : 'flex-row'
        }`}
      >
        <Image
          src={message.imageProfile}
          alt={message.user}
          width={32}
          height={32}
          className="rounded-full object-cover"
        />
        <div
          className={`p-3 rounded-lg ${
            isOwn ? 'bg-blue-500 text-white' : 'bg-gray-100 dark:bg-gray-700'
          }`}
        >
          <div className="flex items-center gap-2 mb-1">
            <span className="text-xs font-medium">
              {isOwn ? 'You' : message.user}
            </span>
            <span className="text-xs px-2 text-gray-500 dark:text-gray-400">
              {format(message.timestamp, 'HH:mm')}
            </span>
          </div>
          <p>{message.text}</p>
          {message.imageMessage && (
            <Image
              src={message.imageMessage}
              alt="Attachment"
              width={160}
              height={90}
              className="rounded-lg mt-2"
            />
          )}
        </div>
      </div>
    </div>
  );
}