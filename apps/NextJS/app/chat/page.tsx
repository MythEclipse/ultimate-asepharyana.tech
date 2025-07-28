'use client';
export const dynamic = 'force-dynamic';

import { useState, useEffect, useRef, useCallback } from 'react';
import { useSession } from 'next-auth/react'; // Import useSession hook
import ReconnectingWebSocket from 'reconnecting-websocket';
import { format } from 'date-fns';
import Card from '@core/ui/ThemedCard';
import { Textarea } from '@/components/text/textarea';
import Button from '@core/ui/BaseButton';
import { AlertCircle, Loader2, Paperclip, Wifi, WifiOff } from 'lucide-react';
import Image from 'next/image';

type RawChatMessage = {
  id: string; // Required in RustExpress
  user_id: string; // Required
  text: string; // Required
  email?: string; // Optional
  image_profile?: string; // Optional
  image_message?: string; // Optional
  role: string; // Required
  timestamp: string; // Required
};

type NormalizedChatMessage = {
  id: string; // String ID from RustExpress
  user: string; // Will map from user_id
  text: string;
  email: string;
  imageProfile: string;
  imageMessage: string;
  role: string;
  timestamp: number; // Convert from string to number for display
};

const normalizeChatMessage = (
  message: RawChatMessage
): NormalizedChatMessage => {
  const timestamp = message.timestamp
    ? Date.parse(message.timestamp)
    : Date.now();

  return {
    id: message.id || crypto.randomUUID(), // Use string ID as is
    user: message.user_id,
    text: message.text,
    email: message.email || '',
    imageProfile: message.image_profile || '/profile-circle-svgrepo-com.svg',
    imageMessage: message.image_message || '',
    role: message.role,
    timestamp: isNaN(timestamp) ? Date.now() : timestamp,
  };
};

export default function ChatClient() {
  const { data: session, status: sessionStatus } = useSession();
  const user = session?.user;
  const isLoading = sessionStatus === 'loading';

  const [messages, setMessages] = useState<NormalizedChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [file, setFile] = useState<File | null>(null);
  const [status, setStatus] = useState({
    connected: false,
    sending: false,
    uploading: false,
  });
  const [error, setError] = useState<string | null>(null);

  const ws = useRef<ReconnectingWebSocket | null>(null);
  const endRef = useRef<HTMLDivElement>(null);
  const userRef = useRef(user); // Use userRef for WebSocket context

  useEffect(() => {
    userRef.current = user;
  }, [user]);

  useEffect(() => {
    // Wait for loading to finish before checking user
    if (isLoading) return;

    // Only connect if user is authenticated
    if (!userRef.current) {
      setError('Please log in to join the chat.');
      setStatus((prev) => ({ ...prev, connected: false }));
      return;
    }

    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const host =
      process.env.NODE_ENV === 'development'
        ? 'localhost:4091'
        : 'ws.asepharyana.tech';
    ws.current = new ReconnectingWebSocket(`${protocol}//${host}/ws`);

    const handleHistory = (data: { messages: RawChatMessage[] }) => {
      if (Array.isArray(data.messages)) {
        const normalizedMessages = data.messages.map(normalizeChatMessage);
        setMessages(normalizedMessages);
      }
    };

    const handleMessage = (raw: string) => {
      try {
        const wsMessage = JSON.parse(raw);
        if (wsMessage.type === 'history') {
          handleHistory(wsMessage);
        } else if (wsMessage.type === 'error') {
          if (wsMessage.user === userRef.current?.email) {
            setError(wsMessage.message || 'An error occurred');
          }
        } else if (wsMessage.type === 'new_message') {
          const message = wsMessage.message;
          const normalizedMessage: NormalizedChatMessage =
            normalizeChatMessage(message);
          setMessages((prev) => {
            const exists = prev.some((m) => m.id === normalizedMessage.id);
            return exists ? prev : [...prev, normalizedMessage];
          });
        }
      } catch {
        console.error('Failed to parse message:', raw);
      }
    };

    ws.current.onmessage = (e) => handleMessage(e.data);
    ws.current.onopen = () => {
      setStatus((prev) => ({ ...prev, connected: true }));
      ws.current?.send(JSON.stringify({ type: 'requestHistory' }));
    };
    ws.current.onclose = () =>
      setStatus((prev) => ({ ...prev, connected: false }));
    ws.current.onerror = () => {
      if (userRef.current?.email) setError('Connection error');
    };

    return () => {
      ws.current?.close();
    };
  }, [userRef.current?.email, isLoading]);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const sendMessage = useCallback(async () => {
    if ((!input.trim() && !file) || status.sending) return;
    if (!user) {
      setError('You must be logged in to send messages.');
      return;
    }

    const newMessage: RawChatMessage = {
      id: crypto.randomUUID(),
      user_id: user.id || 'Anonymous',
      text: input,
      email: user.email || '',
      image_profile: user.image || '/profile-circle-svgrepo-com.svg',
      image_message: '',
      role: 'user',
      timestamp: new Date().toISOString(),
    };

    try {
      setStatus((prev) => ({ ...prev, sending: true }));
      setError(null);

      if (file) {
        setStatus((prev) => ({ ...prev, uploading: true }));
        const formData = new FormData();
        formData.append('file', file);
        const response = await fetch('/api/uploader', {
          method: 'POST',
          body: formData,
        });
        const { url } = await response.json();
        newMessage.image_message = url;
        setFile(null);
        setStatus((prev) => ({ ...prev, uploading: false }));
      }

      ws.current?.send(JSON.stringify(newMessage));
      setInput('');
    } catch {
      if (userRef.current?.email) setError('Failed to send message');
    } finally {
      setStatus((prev) => ({ ...prev, sending: false }));
    }
  }, [input, file, status.sending, user]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-screen">
        <Loader2 className="w-8 h-8 animate-spin mr-2" />
        <span className="text-lg">Checking authentication...</span>
      </div>
    );
  }

  return (
    <div className='mx-auto p-4 max-w-3xl h-screen flex flex-col'>
      <div className='flex flex-col gap-4 flex-1'>
        {/* Header */}
        <div className='text-center space-y-2'>
          <h1 className='text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
            Community Chat
          </h1>

          {/* Status Connection */}
          <div className='flex items-center justify-center gap-2 px-4 py-2 rounded-full bg-gradient-to-r from-blue-50 to-purple-50 dark:from-gray-800 dark:to-gray-800'>
            <div
              className={`p-1.5 rounded-full ${status.connected ? 'bg-green-400' : 'bg-red-400'}`}
            >
              {status.connected ? (
                <Wifi className='w-4 h-4 text-white' />
              ) : (
                <WifiOff className='w-4 h-4 text-white' />
              )}
            </div>
            <span className='text-sm font-medium text-gray-600 dark:text-gray-300'>
              {status.connected ? 'Connected to chat' : 'Connecting...'}
            </span>
          </div>
        </div>

        {/* Chat Messages */}
        <Card>
          <div className='flex-1 overflow-y-auto p-4 space-y-4'>
            {messages.map((message) => (
              <MessageBubble
                key={message.id}
                message={message}
                isOwn={message.email === user?.email}
              />
            ))}
            <div ref={endRef} />
          </div>

          {/* Input Area */}
          <div className='p-4 border-t border-gray-200 dark:border-gray-700 space-y-2'>
            {error && (
              <div className='flex items-center gap-2 px-4 py-2 bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400 rounded-lg'>
                <AlertCircle className='w-5 h-5' />
                <span className='text-sm'>{error}</span>
              </div>
            )}

            <div className='flex flex-col md:flex-row gap-2 items-stretch'>
              <div className='relative flex-1'>
                <Textarea
                  value={input}
                  onChange={(e) => setInput(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter' && !e.shiftKey) {
                      e.preventDefault();
                      sendMessage();
                    }
                  }}
                  placeholder='Type your message...'
                  className='min-h-[100px] md:min-h-[60px] pr-16 resize-none'
                  rows={1}
                  disabled={!status.connected || !user}
                />
                <div className='absolute right-2 bottom-2 flex items-center gap-1.5'>
                  <input
                    type='file'
                    onChange={(e) => setFile(e.target.files?.[0] || null)}
                    className='hidden'
                    id='file-input'
                    disabled={!status.connected || status.uploading || !user}
                  />
                  <label
                    htmlFor='file-input'
                    className={`p-1.5 rounded-md cursor-pointer ${
                      status.uploading
                        ? 'text-gray-400'
                        : 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700'
                    }`}
                  >
                    {status.uploading ? (
                      <Loader2 className='w-5 h-5 animate-spin' />
                    ) : (
                      <Paperclip className='w-5 h-5' />
                    )}
                  </label>
                </div>
              </div>

              <Button
                onClick={sendMessage}
                disabled={
                  !status.connected || status.sending || status.uploading || !user
                }
                className='h-auto bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white shadow-lg'
              >
                {status.sending ? (
                  <Loader2 className='w-5 h-5 animate-spin' />
                ) : (
                  'Send'
                )}
              </Button>
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}

function MessageBubble({
  message,
  isOwn,
}: {
  message: NormalizedChatMessage;
  isOwn: boolean;
}) {
  const safeTimestamp = message.timestamp;

  return (
    <div className={`flex ${isOwn ? 'justify-end' : 'justify-start'}`}>
      <div
        className={`flex items-start gap-3 max-w-[90%] ${
          isOwn ? 'flex-row-reverse' : 'flex-row'
        }`}
      >
        <div className='relative'>
          <Image
            src={message.imageProfile || '/profile-circle-svgrepo-com.svg'}
            alt={message.user}
            width={40}
            height={40}
            className='rounded-full border-2 border-white dark:border-gray-800 shadow-sm'
          />
          {message.role === 'admin' && (
            <div className='absolute -bottom-1 -right-1 bg-blue-500 text-white p-0.5 rounded-full'>
              <svg
                className='w-4 h-4'
                fill='none'
                stroke='currentColor'
                viewBox='0 0 24 24'
              >
                <path
                  strokeLinecap='round'
                  strokeLinejoin='round'
                  strokeWidth={2}
                  d='M5 13l4 4L19 7'
                />
              </svg>
            </div>
          )}
        </div>

        <div
          className={`p-4 rounded-2xl shadow-sm transition-all ${
            isOwn
              ? 'bg-gradient-to-br from-blue-600 to-purple-600 text-white'
              : 'bg-gray-100 dark:bg-gray-800'
          }`}
        >
          <div className='flex items-center gap-3 mb-2'>
            <span className='font-medium text-sm'>
              {isOwn ? 'You' : message.user}
            </span>
            <span
              className={`text-xs ${
                isOwn ? 'text-blue-100' : 'text-gray-500 dark:text-gray-400'
              }`}
            >
              {format(new Date(safeTimestamp), 'HH:mm')}
            </span>
          </div>

          {message.text && (
            <p className='text-sm leading-relaxed break-words'>
              {message.text}
            </p>
          )}

          {message.imageMessage && (
            <div className='mt-3 rounded-lg overflow-hidden border border-gray-200 dark:border-gray-700'>
              <Image
                src={message.imageMessage}
                alt='Attachment'
                width={240}
                height={135}
                className='w-full h-auto object-cover'
              />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
