'use client';

import useSWR from 'swr';
import Card from '@/components/card/ThemedCard';
import { Textarea } from '@/components/text/textarea';
import { useSession } from 'next-auth/react';
import React, { useState, useEffect, useRef, useCallback } from 'react';

type Message = {
  id: string;
  text: string;
  userId: string;
  timestamp: number;
};

const generateId = () =>
  Date.now().toString(36) + Math.random().toString(36).substr(2, 5);

export default function ChatClient() {
  const { data: session } = useSession();
  const [inputValue, setInputValue] = useState('');
  const [userId, setUserId] = useState('');
  const [isConnected, setIsConnected] = useState(false);

  const wsRef = useRef<WebSocket | null>(null);
  const isMounted = useRef(false);

  // SWR untuk manajemen pesan
  const { data: messages = [], mutate } = useSWR<Message[]>(
    'chat-messages',
    null,
    {
      revalidateOnFocus: false,
      revalidateOnReconnect: false,
    }
  );

  // Inisialisasi user ID
  useEffect(() => {
    const storedUserId = localStorage.getItem('chatUserId');
    const newUserId = session?.user?.id || storedUserId || generateId();

    if (!storedUserId && !session?.user?.id) {
      localStorage.setItem('chatUserId', newUserId);
    }
    setUserId(newUserId);
  }, [session]);

  // WebSocket setup dengan SWR integration
  useEffect(() => {
    isMounted.current = true;

    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    wsRef.current = new WebSocket(`${protocol}//ws.asepharyana.cloud`);

    wsRef.current.onopen = () => {
      if (isMounted.current) setIsConnected(true);

      // Fetch initial messages
      wsRef.current?.send(
        JSON.stringify({
          type: 'GET_HISTORY',
          userId,
        })
      );
    };

    wsRef.current.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);

        if (data.type === 'HISTORY') {
          // Update cache dengan histori pesan
          mutate(data.messages, false);
        } else if (data.type === 'NEW_MESSAGE') {
          // Optimistic update dengan SWR
          mutate((prev = []) => [...prev, data.message], false);
        }
      } catch (error) {
        console.error('Message parsing error:', error);
      }
    };

    return () => {
      isMounted.current = false;
      wsRef.current?.close();
    };
  }, [userId, mutate]);

  // Pengiriman pesan dengan optimistic update
  const sendMessage = useCallback(async () => {
    if (!inputValue.trim() || !wsRef.current) return;

    const newMessage: Message = {
      id: generateId(),
      text: inputValue,
      userId,
      timestamp: Date.now(),
    };

    // Optimistic update
    mutate((prev = []) => [...prev, newMessage], false);

    try {
      await new Promise((resolve, reject) => {
        if (wsRef.current?.readyState === WebSocket.OPEN) {
          wsRef.current.send(
            JSON.stringify({
              type: 'NEW_MESSAGE',
              message: newMessage,
            })
          );
          resolve(true);
        } else {
          reject('Connection not ready');
        }
      });

      setInputValue('');
    } catch (error) {
      console.error('Failed to send message:', error);
      // Rollback jika gagal
      mutate(
        (prev = []) => prev.filter((msg) => msg.id !== newMessage.id),
        false
      );
    }
  }, [inputValue, userId, mutate]);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  return (
    <div className='container mx-auto py-8 px-4 max-w-2xl'>
      <h1 className='text-4xl font-extrabold text-gray-800 dark:text-gray-100 mb-8 text-center'>
        Chat Room
      </h1>

      <Card>
        <div className='p-4 border-b'>
          <div className='flex items-center justify-between'>
            <span className='font-medium'>Status:</span>
            <span
              className={`badge ${isConnected ? 'badge-success' : 'badge-error'}`}
            >
              {isConnected ? 'Connected' : 'Disconnected'}
            </span>
          </div>
          <div className='text-sm opacity-75 mt-1'>
            User ID: {userId.slice(0, 8)}
          </div>
        </div>

        <div className='h-96 overflow-y-auto p-4 space-y-3'>
          {messages.map((message) => (
            <div
              key={message.id}
              className={`chat ${message.userId === userId ? 'chat-end' : 'chat-start'}`}
            >
              <div className='chat-header'>
                {message.userId === userId
                  ? 'You'
                  : `User ${message.userId.slice(0, 6)}`}
                <time className='text-xs opacity-50 ml-2'>
                  {new Date(message.timestamp).toLocaleTimeString()}
                </time>
              </div>
              <div
                className={`chat-bubble ${
                  message.userId === userId
                    ? 'chat-bubble-primary'
                    : 'chat-bubble-secondary'
                }`}
              >
                {message.text}
              </div>
            </div>
          ))}
        </div>

        <div className='p-4 border-t'>
          <div className='relative'>
            <Textarea
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder='Type a message...'
              className='pr-20 resize-none'
              rows={3}
            />
            <button
              onClick={sendMessage}
              className='absolute right-3 bottom-3 btn btn-primary btn-sm'
              disabled={!isConnected}
            >
              Send
            </button>
          </div>
        </div>
      </Card>
    </div>
  );
}
