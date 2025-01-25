'use client';

import { useState, useEffect, useRef, useCallback } from 'react';
import { useSession } from 'next-auth/react';
import ReconnectingWebSocket from 'reconnecting-websocket';
import { format } from 'date-fns';
import Card from '@/components/card/ThemedCard';
import { Textarea } from '@/components/text/textarea';
import Button  from '@/components/button/NormalButton';
import { Loader2 } from 'lucide-react';

type Message = {
  id: string;
  text: string;
  userId: string;
  user: string;
  timestamp: number;
};

export default function ChatClient() {
  const { data: session } = useSession();
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [isSending, setIsSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const wsRef = useRef<ReconnectingWebSocket | null>(null);
  const userIdRef = useRef<string>('');
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const textAreaRef = useRef<HTMLTextAreaElement>(null);

  // Stabilkan user ID
  useEffect(() => {
    const getUserId = () => {
      if (session?.user?.id) return session.user.id;
      const storedId = localStorage.getItem('chatUserId') || crypto.randomUUID();
      if (!localStorage.getItem('chatUserId')) localStorage.setItem('chatUserId', storedId);
      return storedId;
    };
    
    userIdRef.current = getUserId();
  }, [session]);

  // Setup WebSocket dengan cleanup
  useEffect(() => {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const ws = new ReconnectingWebSocket(`${protocol}//ws.asepharyana.cloud`);

    wsRef.current = ws;

    const handleMessage = (event: MessageEvent) => {
      const message: Message = JSON.parse(event.data);
      setMessages(prev => [...prev, message]);
    };

    const handleOpen = () => {
      setIsConnected(true);
      setError(null);
      console.log('WS Connected - ID:', userIdRef.current);
    };

    const handleClose = () => setIsConnected(false);
    const handleError = () => setError('Connection error. Reconnecting...');

    ws.addEventListener('open', handleOpen);
    ws.addEventListener('close', handleClose);
    ws.addEventListener('message', handleMessage);
    ws.addEventListener('error', handleError);

    return () => {
      console.log('Cleaning up WebSocket');
      ws.removeEventListener('open', handleOpen);
      ws.removeEventListener('close', handleClose);
      ws.removeEventListener('message', handleMessage);
      ws.removeEventListener('error', handleError);
      ws.close();
      wsRef.current = null;
    };
  }, []);

  // Auto-scroll dan resize
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

  // Handle pengiriman pesan
  const handleSendMessage = useCallback(async () => {
    if (!inputValue.trim() || isSending) return;

    setIsSending(true);
    setError(null);

    try {
      const newMessage = {
        text: inputValue,
        userId: userIdRef.current,
        user: session?.user?.name || 'Anonymous',
        timestamp: Date.now(),
      };

      wsRef.current?.send(JSON.stringify(newMessage));
      setInputValue('');
    } catch (err) {
      setError('Failed to send message');
    } finally {
      setIsSending(false);
    }
  }, [inputValue, session, isSending]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  return (
    <div className="container mx-auto py-8 px-4 max-w-2xl">
      <h1 className="text-4xl font-extrabold text-gray-800 dark:text-gray-100 mb-8 text-center">
        Chat Room
      </h1>

      <Card>
        <div className="flex items-center justify-between text-sm p-4 border-b">
          <span>Status:</span>
          <div className="flex items-center gap-2">
            <div className={`w-2 h-2 rounded-full ${isConnected ? 'bg-green-500' : 'bg-red-500'}`} />
            <span className={isConnected ? 'text-green-500' : 'text-red-500'}>
              {isConnected ? `Connected as ${userIdRef.current}` : 'Connecting...'}
            </span>
          </div>
        </div>

        <div className="h-96 overflow-y-auto p-4 space-y-3">
          {messages.map((message) => (
            <MessageBubble
              key={`${message.id}-${message.timestamp}`}
              message={message}
              isOwn={message.userId === userIdRef.current}
            />
          ))}
          <div ref={messagesEndRef} />
        </div>

        <div className="p-4 border-t">
          <div className="relative">
            <Textarea
              ref={textAreaRef}
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Type a message..."
              className="pr-20 resize-none"
              rows={1}
              disabled={!isConnected}
            />
            <Button
              onClick={handleSendMessage}
              disabled={!isConnected || isSending}
              className="absolute right-3 bottom-3 gap-1.5"
            >
              {isSending ? <Loader2 className="w-4 h-4 animate-spin" /> : 'Send'}
            </Button>
          </div>
          {error && <div className="text-red-500 text-sm mt-2 text-center">{error}</div>}
        </div>
      </Card>
    </div>
  );
}

function MessageBubble({ message, isOwn }: { message: Message; isOwn: boolean }) {
  return (
    <div className={`flex ${isOwn ? 'justify-end' : 'justify-start'}`}>
      <div className={`flex items-start gap-3 max-w-[85%] ${isOwn ? 'flex-row-reverse' : 'flex-row'}`}>
        <div className="flex-shrink-0 w-8 h-8 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center">
          {message.user[0].toUpperCase()}
        </div>
        <div
          className={`p-3 rounded-lg ${
            isOwn ? 'bg-blue-500 text-white' : 'bg-gray-100 dark:bg-gray-700'
          }`}
        >
          <div className="text-xs font-medium mb-1">{isOwn ? 'You' : message.user}</div>
          <div className="whitespace-pre-wrap break-words">{message.text}</div>
          <div className="text-xs mt-1 opacity-70">
            {format(new Date(message.timestamp), 'HH:mm')}
          </div>
        </div>
      </div>
    </div>
  );
}