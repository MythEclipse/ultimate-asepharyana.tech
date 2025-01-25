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
    this.prisma = new PrismaClient({
      log: ['query', 'info', 'warn', 'error'], // Aktifkan logging query
    });
  }

  async createMessage(data: MessageCreateInput): Promise<ChatMessage> {
    try {
      logger.info('Attempting to save message:', { data });

      const message = await this.prisma.chatMessage.create({
        data: {
          text: data.text,
          userId: data.userId,
          user: data.user,
        },
      });

      logger.info('Message successfully saved:', { messageId: message.id });
      return {
        id: message.id,
        text: message.text,
        userId: message.userId,
        user: message.user,
        timestamp: message.timestamp,
      };
    } catch (error) {
      logger.error('Failed to create message:', {
        error,
        inputData: data,
        stack: new Error().stack,
      });
      throw new Error('Failed to save message to database');
    }
  }

  async getMessagesPaginated(page: number = 1): Promise<PaginatedMessages> {
    try {
      const [totalMessages, messages] = await Promise.all([
        this.prisma.chatMessage.count(),
        this.prisma.chatMessage.findMany({
          skip: (page - 1) * MAX_MESSAGES_PER_PAGE,
          take: MAX_MESSAGES_PER_PAGE,
          orderBy: { timestamp: 'desc' },
          select: {
            id: true,
            text: true,
            userId: true,
            user: true,
            timestamp: true,
          },
        }),
      ]);

      return {
        messages,
        currentPage: page,
        totalPages: Math.ceil(totalMessages / MAX_MESSAGES_PER_PAGE),
        totalMessages,
      };
    } catch (error) {
      logger.error('Failed to get messages:', {
        error,
        page,
        stack: new Error().stack,
      });
      throw new Error('Failed to load chat history');
    }
  }

  async close() {
    try {
      await this.prisma.$disconnect();
      logger.info('Database connection closed');
    } catch (error) {
      logger.error('Error closing database connection:', error);
    }
  }
}
