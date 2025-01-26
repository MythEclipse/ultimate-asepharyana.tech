'use client';

import { useState, useEffect, useRef, useCallback } from 'react';
import { useSession } from 'next-auth/react';
import ReconnectingWebSocket from 'reconnecting-websocket';
import { format } from 'date-fns';
import Card from '@/components/card/ThemedCard';
import { Textarea } from '@/components/text/textarea';
import Button from '@/components/button/NormalButton';
import { Loader2 } from 'lucide-react';
import Image from 'next/image';

type ChatMessage = {
  id?: number;
  user: string;
  text: string;
  email?: string;
  imageProfile?: string;
  imageMessage?: string;
  role: string;
  timestamp?: number;
};

const validateTimestamp = (ts?: number) => {
  return ts && typeof ts === 'number' && !isNaN(ts) && ts > 0 ? ts : Date.now();
};

export default function ChatClient() {
  const { data: session } = useSession();
  const [messages, setMessages] = useState<ChatMessage[]>([]);
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
  const sessionRef = useRef(session);

  useEffect(() => {
    sessionRef.current = session;
  }, [session]);

  useEffect(() => {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    ws.current = new ReconnectingWebSocket(`${protocol}//ws.asepharyana.cloud`);

    const handleHistory = (data: { messages: ChatMessage[] }) => {
      if (Array.isArray(data.messages)) {
        const normalizedMessages = data.messages.map(
          (message: ChatMessage) => ({
            id: message.id,
            user: message.user,
            text: message.text,
            email: message.email || '',
            imageProfile:
              message.imageProfile || '/profile-circle-svgrepo-com.svg',
            role: message.role,
            timestamp: validateTimestamp(message.timestamp),
            imageMessage: message.imageMessage || '',
          })
        );
        setMessages(normalizedMessages);
      }
    };

    const handleMessage = (raw: string) => {
      try {
        const wsMessage = JSON.parse(raw);

        if (wsMessage.type === 'history') {
          handleHistory(wsMessage);
        } else if (wsMessage.type === 'error') {
          if (wsMessage.user === sessionRef.current?.user?.name) {
            setError(wsMessage.message || 'An error occurred');
          }
        } else if (wsMessage.type === 'new_message') {
          const message = wsMessage.message;

          const normalizedMessage: ChatMessage = {
            id: message.id,
            user: message.user,
            text: message.text,
            email: message.email || '',
            imageProfile:
              message.imageProfile || '/profile-circle-svgrepo-com.svg',
            role: message.role,
            timestamp: validateTimestamp(message.timestamp),
            imageMessage: message.imageMessage,
          };

          setMessages((prev) => {
            const exists = prev.some((m) => m.id === normalizedMessage.id);
            return exists ? prev : [...prev, normalizedMessage];
          });
        }
      } catch (err) {
        console.error('Failed to parse message:', raw);
      }
    };

    ws.current.onmessage = (e) => handleMessage(e.data);
    ws.current.onopen = () => {
      setStatus((p) => ({ ...p, connected: true }));
      ws.current?.send(JSON.stringify({ type: 'requestHistory' }));
    };
    ws.current.onclose = () => setStatus((p) => ({ ...p, connected: false }));
    ws.current.onerror = () => {
      if (sessionRef.current?.user?.name) {
        setError('Connection error');
      }
    };

    return () => {
      ws.current?.close();
    };
  }, []);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const sendMessage = useCallback(async () => {
    if ((!input.trim() && !file) || status.sending) return;

    const newMessage: ChatMessage = {
      user: session?.user?.name || 'Anonymous',
      text: input,
      email: session?.user?.email || '',
      imageProfile: session?.user?.image || '/profile-circle-svgrepo-com.svg',
      imageMessage: '',
      role: 'user',
    };

    try {
      setStatus((p) => ({ ...p, sending: true }));
      setError(null);

      if (file) {
        setStatus((p) => ({ ...p, uploading: true }));
        const formData = new FormData();
        formData.append('file', file);
        const response = await fetch('/api/uploader', {
          method: 'POST',
          body: formData,
        });
        const { url } = await response.json();
        newMessage.imageMessage = url;
        setFile(null);
        setStatus((p) => ({ ...p, uploading: false }));
      }

      ws.current?.send(JSON.stringify(newMessage));
      setInput('');
    } catch (err) {
      if (sessionRef.current?.user?.name) {
        setError('Failed to send message');
      }
    } finally {
      setStatus((p) => ({ ...p, sending: false }));
    }
  }, [input, file, status.sending, session]);

  return (
    <div className='container mx-auto py-8 px-4 max-w-2xl'>
      <h1 className='text-4xl font-extrabold text-gray-800 dark:text-gray-100 mb-8 text-center'>
        Chat Room
      </h1>

      <Card>
        <div className='flex items-center justify-between text-sm p-4 border-b border-blue-500'>
          <span className='text-blue-500'>Status:</span>
          <div className='flex items-center gap-2'>
            <div
              className={`w-2 h-2 rounded-full ${status.connected ? 'bg-green-500' : 'bg-red-500'}`}
            />
            <span
              className={status.connected ? 'text-green-500' : 'text-red-500'}
            >
              {status.connected ? 'Connected' : 'Connecting...'}
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
          <div ref={endRef} />
        </div>

        <div className='p-4 border-t border-blue-500'>
          <div className='flex items-start gap-2'>
            <Textarea
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) =>
                e.key === 'Enter' &&
                !e.shiftKey &&
                (e.preventDefault(), sendMessage())
              }
              placeholder='Type a message...'
              className='flex-1 resize-none min-h-[40px] border-blue-500 top-2'
              rows={1}
              disabled={!status.connected}
            />
            <div className='flex items-center gap-2 h-full'>
              <input
                type='file'
                onChange={(e) => setFile(e.target.files?.[0] || null)}
                className='hidden'
                id='file-input'
                disabled={!status.connected || status.uploading}
              />
              <label
                htmlFor='file-input'
                className={`h-10 px-3 py-2 flex items-center justify-center rounded-md text-sm border ${
                  status.uploading
                    ? 'text-gray-400 border-gray-400'
                    : 'text-blue-500 border-blue-500 hover:bg-blue-50'
                } cursor-pointer`}
              >
                {status.uploading ? 'Uploading...' : '📎'}
              </label>
              <Button
                className='!px-3 !py-1'
                onClick={sendMessage}
                disabled={
                  !status.connected || status.sending || status.uploading
                }
              >
                {status.sending ? (
                  <Loader2 className='w-4 h-4 animate-spin' />
                ) : (
                  'Send'
                )}
              </Button>
            </div>
          </div>
          {error && (
            <div className='text-red-500 text-sm mt-2 text-center'>{error}</div>
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
  const safeTimestamp = validateTimestamp(message.timestamp);

  return (
    <div className={`flex ${isOwn ? 'justify-end' : 'justify-start'}`}>
      <div
        className={`flex items-start gap-3 max-w-[85%] ${isOwn ? 'flex-row-reverse' : 'flex-row'}`}
      >
        <Image
          src={message.imageProfile || '/profile-circle-svgrepo-com.svg'}
          alt={message.user}
          width={32}
          height={32}
          className='rounded-full object-cover'
        />

        <div
          className={`p-3 rounded-lg ${isOwn ? 'bg-blue-500 text-white' : 'bg-gray-100 dark:bg-gray-700'}`}
        >
          <div className='flex items-center gap-2 mb-1'>
            <span className='text-xs font-medium'>
              {isOwn ? 'You' : message.user}
            </span>
            <span className='text-xs px-2 text-gray-500 dark:text-gray-400'>
              {format(new Date(safeTimestamp), 'HH:mm')}
            </span>
          </div>
          {message.text && <p>{message.text}</p>}
          {message.imageMessage && (
            <Image
              src={message.imageMessage}
              alt='Attachment'
              width={160}
              height={90}
              className='rounded-lg mt-2'
            />
          )}
        </div>
      </div>
    </div>
  );
}
