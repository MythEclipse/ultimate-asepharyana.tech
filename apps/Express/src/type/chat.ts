// types/chat.ts
import { z } from 'zod';
import { MessageValidator } from '@/validators/messageValidator';

export type MessageCreateInput = z.infer<typeof MessageValidator>;

export type ChatMessage = {
  id: string;
  text: string;
  userId: string;
  user: string;
  timestamp: Date;
};

export type PaginatedMessages = {
  messages: ChatMessage[];
  currentPage: number;
  totalPages: number;
  totalMessages: number;
};
