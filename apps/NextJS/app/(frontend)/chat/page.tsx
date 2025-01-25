'use client';

import { useState, useEffect, useRef, useCallback } from 'react';
import { useSession } from 'next-auth/react';
import { format } from 'date-fns';
import ReconnectingWebSocket from 'reconnecting-websocket';
import Card from '@/components/card/ThemedCard';
import { Textarea } from '@/components/text/textarea';
import Button from '@/components/button/NormalButton';
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
  const [userId, setUserId] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [isSending, setIsSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const socketRef = useRef<ReconnectingWebSocket | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const textAreaRef = useRef<HTMLTextAreaElement>(null);

  // Auto-resize textarea
  useEffect(() => {
    if (textAreaRef.current) {
      textAreaRef.current.style.height = 'auto';
      textAreaRef.current.style.height = `${textAreaRef.current.scrollHeight}px`;
    }
  }, [inputValue]);

  // Auto-scroll to bottom
  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [messages, scrollToBottom]);

  // User ID management
  useEffect(() => {
    const storedUserId = localStorage.getItem('chatUserId');
    if (session?.user?.id) {
      setUserId(session.user.id);
      if (storedUserId) localStorage.removeItem('chatUserId');
    } else if (storedUserId) {
      setUserId(storedUserId);
    } else {
      const newUserId = crypto.randomUUID();
      localStorage.setItem('chatUserId', newUserId);
      setUserId(newUserId);
    }
  }, [session]);

  // WebSocket connection
  useEffect(() => {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const socket = new ReconnectingWebSocket(
      `${protocol}//ws.asepharyana.cloud`
    );

    socketRef.current = socket;

    socket.addEventListener('open', () => {
      setIsConnected(true);
      setError(null);
    });

    socket.addEventListener('close', () => setIsConnected(false));

    socket.addEventListener('message', (event) => {
      const message: Message = JSON.parse(event.data);
      setMessages((prev) => [...prev, message]);
    });

    socket.addEventListener('error', () => {
      setError('Connection error. Trying to reconnect...');
    });

    return () => {
      socket.close();
      socketRef.current = null;
    };
  }, [userId]);

  // Message sending
  const handleSendMessage = useCallback(async () => {
    if (!inputValue.trim() || !socketRef.current || isSending) return;

    setIsSending(true);
    setError(null);

    try {
      const newMessage = {
        text: inputValue,
        userId,
        user: session?.user?.name || 'Anonymous',
        timestamp: Date.now(),
      };

      socketRef.current.send(JSON.stringify(newMessage));
      setInputValue('');
    } catch (err) {
      setError('Failed to send message. Please try again.');
    } finally {
      setIsSending(false);
    }
  }, [inputValue, userId, session, isSending]);

  // Keyboard handling
  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
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
            <div
              className={`w-2 h-2 rounded-full ${
                isConnected ? 'bg-green-500' : 'bg-red-500'
              }`}
            />
            <span className={isConnected ? 'text-green-500' : 'text-red-500'}>
              {isConnected ? `Connected as ${userId}` : 'Connecting...'}
            </span>
          </div>
        </div>

        <div className="h-96 overflow-y-auto p-4 space-y-3">
          {messages.map((message) => {
            const isSent = message.userId === userId;
            return (
              <div
                key={message.id}
                className={`flex ${isSent ? 'justify-end' : 'justify-start'}`}
              >
                <div
                  className={`flex items-start gap-3 max-w-[85%] ${
                    isSent ? 'flex-row-reverse' : 'flex-row'
                  }`}
                >
                  <div className="flex-shrink-0 w-8 h-8 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center">
                    {message.user[0].toUpperCase()}
                  </div>
                  <div
                    className={`p-3 rounded-lg ${
                      isSent
                        ? 'bg-blue-500 text-white'
                        : 'bg-gray-100 dark:bg-gray-700'
                    }`}
                  >
                    <div className="text-xs font-medium mb-1">
                      {isSent ? 'You' : message.user}
                    </div>
                    <div className="whitespace-pre-wrap break-words">
                      {message.text}
                    </div>
                    <div className="text-xs mt-1 opacity-70">
                      {format(new Date(message.timestamp), 'HH:mm')}
                    </div>
                  </div>
                </div>
              </div>
            );
          })}
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
              {isSending ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                'Send'
              )}
            </Button>
          </div>
          {error && (
            <div className="text-red-500 text-sm mt-2 text-center">{error}</div>
          )}
        </div>
      </Card>
    </div>
  );
}