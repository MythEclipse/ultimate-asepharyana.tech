'use client';

import Card from '@/components/card/ThemedCard';
import { Textarea } from '@/components/text/textarea';
import { useSession } from 'next-auth/react';
import React, { useState, useEffect, useRef } from 'react';

export default function ChatClient() {
  const { data: session } = useSession();
  const [messages, setMessages] = useState<
    Array<{ user: string; text: string }>
  >([]);
  const [inputValue, setInputValue] = useState('');
  const [userId, setUserId] = useState('');
  const wsRef = useRef<WebSocket | null>(null);
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    const generateRandomId = () => {
      return 'user_' + Math.random().toString(36).substr(2, 9);
    };

    if (session?.user?.id) {
      setUserId(session.user.id);
    } else {
      setUserId(generateRandomId());
    }

    // Create WebSocket connection
    const connectWebSocket = () => {
      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      wsRef.current = new WebSocket(`${protocol}//ws.asepharyana.cloud`);

      wsRef.current.onopen = () => {
        setIsConnected(true);
        console.log('WebSocket Connected');
      };

      // Handle incoming messages
      wsRef.current.onmessage = (event) => {
        const message = JSON.parse(event.data);
        setMessages((prevMessages) => [...prevMessages, message]);
      };

      wsRef.current.onclose = () => {
        setIsConnected(false);
        console.log('WebSocket Disconnected');
        // Attempt to reconnect after 3 seconds
        setTimeout(connectWebSocket, 3000);
      };
    };

    connectWebSocket();

    // Clean up WebSocket connection on component unmount
    return () => {
      if (wsRef.current) {
        wsRef.current.onclose = null; // Prevent reconnection on intentional close
        wsRef.current.close();
      }
    };
  }, [session]);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && inputValue.trim() && userId.trim()) {
      if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
        const message = {
          text: inputValue,
          userId: userId,
        };
        wsRef.current.send(JSON.stringify(message));
        setInputValue('');
      }
    }
  };

  return (
    <div className='container mx-auto py-8 px-4'>
      <h1 className='text-4xl font-extrabold text-gray-800 dark:text-gray-100 mb-8 text-center'>
        Chat Room
      </h1>
      <div className='mb-8'>
        <Card>
          <div className='mb-2'>
            Connection Status:{' '}
            {isConnected ? (
              <span className='text-green-500'>Connected as {userId}</span>
            ) : (
              <span className='text-red-500'>Disconnected</span>
            )}
          </div>
          <div className='mb-4 space-y-2'>
            {messages.map((message, index) => (
              <div
                key={index}
                className='bg-blue-100 dark:bg-gray-700 p-2 rounded border border-blue-500 dark:border-blue-500'
              >
                <span className='font-bold text-blue-700 dark:text-blue-300'>
                  {message.user}:{' '}
                </span>
                <span className='text-blue-600 dark:text-blue-200'>
                  {message.text}
                </span>
              </div>
            ))}
          </div>
          <Textarea
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder='Type a message...'
            className='w-full p-2 border border-blue-500 dark:border-blue-500 rounded-lg focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-500 dark:bg-gray-700 dark:text-gray-100'
          />
        </Card>
      </div>
    </div>
  );
}
