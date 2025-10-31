'use client';

import React, { useEffect, useRef } from 'react';
import { format } from 'date-fns';
import { ChatMessage } from '../../lib/chat-api';

interface MessageListProps {
  messages: ChatMessage[];
  currentUserId?: string;
}

export default function MessageList({ messages, currentUserId }: MessageListProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  if (messages.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground">
        <p>No messages yet. Start the conversation!</p>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto p-4 space-y-4">
      {messages.map((message) => {
        const isOwnMessage = message.user_id === currentUserId;

        return (
          <div
            key={message.id}
            className={`flex ${isOwnMessage ? 'justify-end' : 'justify-start'}`}
          >
            <div
              className={`max-w-[70%] rounded-lg px-4 py-2 ${
                isOwnMessage
                  ? 'bg-primary text-primary-foreground'
                  : 'bg-muted text-foreground'
              }`}
            >
              {!isOwnMessage && (
                <p className="text-xs font-semibold mb-1 opacity-70">
                  {message.user_name}
                </p>
              )}
              <p className="text-sm break-words whitespace-pre-wrap">
                {message.content}
              </p>
              <p className="text-xs mt-1 opacity-70">
                {format(new Date(message.created_at), 'HH:mm')}
              </p>
            </div>
          </div>
        );
      })}
      <div ref={messagesEndRef} />
    </div>
  );
}
