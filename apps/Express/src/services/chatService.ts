// services/chatService.ts
import { PrismaClient } from '@prisma/client';
import {
  MessageCreateInput,
  ChatMessage,
  PaginatedMessages,
} from '@/type/chat';
import logger from '@/utils/logger';

const MAX_MESSAGES_PER_PAGE = 50;

export class ChatService {
  private prisma: PrismaClient;

  constructor() {
    this.prisma = new PrismaClient();
  }

  async createMessage(data: MessageCreateInput): Promise<ChatMessage> {
    try {
      const message = await this.prisma.chatMessage.create({
        data: {
          text: data.text,
          userId: data.userId,
          user: data.user,
        },
      });

      return {
        id: message.id,
        text: message.text,
        userId: message.userId,
        user: message.user,
        timestamp: message.timestamp,
      };
    } catch (error) {
      logger.error('Failed to create message:', error);
      throw error;
    }
  }

  async getMessagesPaginated(page: number = 1): Promise<PaginatedMessages> {
    try {
      const totalMessages = await this.prisma.chatMessage.count();
      const messages = await this.prisma.chatMessage.findMany({
        skip: (page - 1) * MAX_MESSAGES_PER_PAGE,
        take: MAX_MESSAGES_PER_PAGE,
        orderBy: { timestamp: 'desc' },
      });

      return {
        messages: messages.map((m) => ({
          id: m.id,
          text: m.text,
          userId: m.userId,
          user: m.user,
          timestamp: m.timestamp,
        })),
        currentPage: page,
        totalPages: Math.ceil(totalMessages / MAX_MESSAGES_PER_PAGE),
        totalMessages,
      };
    } catch (error) {
      logger.error('Failed to get messages:', error);
      throw error;
    }
  }

  async close() {
    await this.prisma.$disconnect();
  }
}
