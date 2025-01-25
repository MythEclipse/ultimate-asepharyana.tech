'use client';

import Card from '@/components/card/ThemedCard';
import { Textarea } from '@/components/text/textarea';
import { useSession } from 'next-auth/react';
import React, { useState, useEffect, useRef, useCallback } from 'react';

// Type untuk pesan
type Message = {
  id: string;
  text: string;
  userId: string;
  timestamp: number;
  isSent: boolean;
};

// Fungsi pembuat ID unik
const generateId = () =>
  Date.now().toString(36) + Math.random().toString(36).substr(2, 5);

export default function ChatClient() {
  const { data: session } = useSession();
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [userId, setUserId] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [isSending, setIsSending] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectAttempts = useRef(0);

  // Generate atau ambil user ID
  useEffect(() => {
    const storedUserId = localStorage.getItem('chatUserId');
    if (session?.user?.id) {
      setUserId(session.user.id);
    } else if (storedUserId) {
      setUserId(storedUserId);
    } else {
      const newUserId = generateId();
      localStorage.setItem('chatUserId', newUserId);
      setUserId(newUserId);
    }
  }, [session]);

  // Handle WebSocket connection dengan reconnect exponential backoff
  const connectWebSocket = useCallback(() => {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    wsRef.current = new WebSocket(`${protocol}//ws.asepharyana.cloud`);

    wsRef.current.onopen = () => {
      setIsConnected(true);
      reconnectAttempts.current = 0;
      console.log('WebSocket Connected');
    };

    wsRef.current.onmessage = (event) => {
      try {
        const message: Message = JSON.parse(event.data);
        setMessages((prev) => [
          ...prev,
          {
            ...message,
            isSent: message.userId === userId,
          },
        ]);
      } catch (error) {
        console.error('Invalid message format:', error);
      }
    };

    wsRef.current.onclose = () => {
      setIsConnected(false);
      const delay = Math.min(3000 * 2 ** reconnectAttempts.current, 30000);
      setTimeout(() => {
        reconnectAttempts.current += 1;
        connectWebSocket();
      }, delay);
    };

    wsRef.current.onerror = (error) => {
      console.error('WebSocket Error:', error);
    };
  }, [userId]);

  useEffect(() => {
    if (userId) {
      connectWebSocket();
    }

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
        wsRef.current = null;
      }
    };
  }, [userId, connectWebSocket]);

  // Handle pengiriman pesan dengan optimistik update
  const handleSendMessage = useCallback(async () => {
    if (!inputValue.trim() || !wsRef.current || isSending) return;

    const tempId = generateId();
    const newMessage: Message = {
      id: tempId,
      text: inputValue,
      userId,
      timestamp: Date.now(),
      isSent: true,
    };

    try {
      setIsSending(true);
      setMessages((prev) => [...prev, newMessage]);

      await new Promise((resolve, reject) => {
        if (wsRef.current?.readyState === WebSocket.OPEN) {
          wsRef.current.send(JSON.stringify(newMessage));
          resolve(true);
        } else {
          reject('Connection not ready');
        }
      });

      setInputValue('');
    } catch (error) {
      console.error('Failed to send message:', error);
      setMessages((prev) => prev.filter((msg) => msg.id !== tempId));
    } finally {
      setIsSending(false);
    }
  }, [inputValue, userId, isSending]);

  // Handle keyboard events
  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  return (
    <div className='container mx-auto py-8 px-4 max-w-2xl'>
      <h1 className='text-4xl font-extrabold text-gray-800 dark:text-gray-100 mb-8 text-center'>
        Chat Room
      </h1>

      <Card>
        <div className='flex items-center justify-between text-sm'>
          <span>Status: </span>
          <span className={isConnected ? 'text-green-500' : 'text-red-500'}>
            {isConnected ? `Connected as ${userId}` : 'Disconnected'}
          </span>
        </div>

        <div className='h-96 overflow-y-auto space-y-2 p-2 bg-gray-50 dark:bg-gray-800 rounded-lg'>
          {messages.map((message) => (
            <div
              key={message.id}
              className={`p-3 rounded-lg max-w-[80%] ${
                message.isSent
                  ? 'ml-auto bg-blue-500 text-white'
                  : 'mr-auto bg-gray-200 dark:bg-gray-700'
              }`}
            >
              <div className='text-xs opacity-75 mb-1'>
                {message.isSent ? 'You' : `User ${message.userId.slice(0, 6)}`}
              </div>
              <div className='whitespace-pre-wrap break-words'>
                {message.text}
              </div>
            </div>
          ))}
        </div>

        <div className='relative'>
          <Textarea
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder='Type a message...'
            className='pr-16 resize-none'
            disabled={!isConnected || isSending}
          />
          <button
            onClick={handleSendMessage}
            disabled={!isConnected || isSending}
            className='absolute right-2 bottom-2 px-4 py-1 bg-blue-500 text-white rounded-md hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed'
          >
            {isSending ? 'Sending...' : 'Send'}
          </button>
        </div>
      </Card>
    </div>
  );
}
