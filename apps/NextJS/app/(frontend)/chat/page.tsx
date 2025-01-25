'use client';

import { useState, useEffect, useRef, useCallback } from 'react';
import { useSession } from 'next-auth/react';
import { io, Socket } from 'socket.io-client';
import Card from '@/components/card/ThemedCard';
import { Textarea } from '@/components/text/textarea';

type Message = {
  id: string;
  text: string;
  userId: string;
  user: string;
  timestamp: number;
  isSent: boolean;
};

const generateId = () =>
  Date.now().toString(36) + Math.random().toString(36).substr(2, 5);

export default function ChatClient() {
  const { data: session } = useSession();
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [userId, setUserId] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const socketRef = useRef<Socket | null>(null);

  // Generate or retrieve user ID
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

  // Setup Socket.IO connection
  useEffect(() => {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const socket = io(`${protocol}//ws.asepharyana.cloud`, {
      reconnection: true,
      reconnectionAttempts: 5,
      reconnectionDelay: 3000,
      reconnectionDelayMax: 10000,
      autoConnect: true,
    });

    socketRef.current = socket;

    // Event listeners
    socket.on('connect', () => {
      setIsConnected(true);
      console.log('Socket.IO Connected');
    });

    socket.on('disconnect', () => {
      setIsConnected(false);
      console.log('Socket.IO Disconnected');
    });

    socket.on('chat_message', (message: Message) => {
      setMessages((prev) => [
        ...prev,
        {
          ...message,
          isSent: message.userId === userId,
        },
      ]);
    });

    socket.on('connect_error', (error) => {
      console.error('Socket.IO Connection Error:', error);
    });

    // Cleanup on unmount
    return () => {
      socket.disconnect();
      socketRef.current = null;
    };
  }, [userId]);

  // Handle sending messages
  const handleSendMessage = useCallback(() => {
    if (!inputValue.trim() || !socketRef.current) return;

    const newMessage: Message = {
      id: generateId(),
      text: inputValue,
      userId,
      user: session?.user?.name || 'Anonymous',
      timestamp: Date.now(),
      isSent: true,
    };

    // Optimistic update
    setMessages((prev) => [...prev, newMessage]);

    // Send message to server
    socketRef.current.emit('chat_message', newMessage, (ack: { success: boolean }) => {
      if (!ack.success) {
        console.error('Failed to send message');
        // Rollback if failed
        setMessages((prev) => prev.filter((msg) => msg.id !== newMessage.id));
      }
    });

    setInputValue('');
  }, [inputValue, userId, session]);

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
        <div className='flex items-center justify-between text-sm p-4 border-b'>
          <span>Status:</span>
          <span className={isConnected ? 'text-green-500' : 'text-red-500'}>
            {isConnected ? `Connected as ${userId}` : 'Disconnected'}
          </span>
        </div>

        <div className='h-96 overflow-y-auto p-4 space-y-3'>
          {messages.map((message) => (
            <div
              key={message.id}
              className={`flex flex-col ${
                message.isSent ? 'items-end' : 'items-start'
              }`}
            >
              <div
                className={`p-3 rounded-lg max-w-[85%] ${
                  message.isSent
                    ? 'bg-blue-500 text-white'
                    : 'bg-gray-100 dark:bg-gray-700'
                }`}
              >
                <div className='text-xs opacity-75 mb-1'>
                  {message.isSent ? 'You' : message.user}
                </div>
                <div className='whitespace-pre-wrap break-words'>
                  {message.text}
                </div>
                <div className='text-xs mt-1 opacity-50'>
                  {new Date(message.timestamp).toLocaleTimeString()}
                </div>
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
              disabled={!isConnected}
            />
            <button
              onClick={handleSendMessage}
              disabled={!isConnected}
              className='absolute right-3 bottom-3 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg disabled:opacity-50 disabled:cursor-not-allowed transition-colors'
            >
              Send
            </button>
          </div>
        </div>
      </Card>
    </div>
  );
}
