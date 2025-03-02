import { prisma, ChatMessage } from '@asepharyana/database';
import logger from '../utils/logger';

export class ChatService {
  async saveMessage(
    message: Omit<ChatMessage, 'id' | 'timestamp'>
  ): Promise<ChatMessage> {
    try {
      const savedMessage = await prisma.chatMessage.create({
        data: message,
      });
      logger.info(`Message saved: ${JSON.stringify(savedMessage)}`);
      return savedMessage;
    } catch (error) {
      if (error instanceof Error) {
        throw new Error(`Failed to save message: ${error.message}`);
      } else {
        throw new Error('Failed to save message: Unknown error');
      }
    }
  }

  async loadMessages(limit: number = 50): Promise<ChatMessage[]> {
    try {
      const messages = await prisma.chatMessage.findMany({
        orderBy: { timestamp: 'desc' },
        take: limit,
      });
      return messages;
    } catch (error) {
      if (error instanceof Error) {
        throw new Error(`Failed to load messages: ${error.message}`);
      } else {
        throw new Error('Failed to load messages: Unknown error');
      }
    }
  }

  async closeDatabase(): Promise<void> {
    try {
      await prisma.$disconnect();
      logger.info('Chat database connection closed');
    } catch (error) {
      if (error instanceof Error) {
        throw new Error(`Failed to close database: ${error.message}`);
      } else {
        throw new Error('Failed to close database: Unknown error');
      }
    }
  }
}
