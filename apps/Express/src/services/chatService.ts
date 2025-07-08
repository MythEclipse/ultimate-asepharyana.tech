import { prisma, ChatMessage } from '@asepharyana/database';
import logger from '../utils/logger';
import { handleServiceError } from '../utils/errorUtils';

export class ChatService {
  async saveMessage(message: ChatMessage): Promise<ChatMessage> {
    try {
      const savedMessage = await prisma.chatMessage.create({
        data: message,
      });
      return savedMessage;
    } catch (error) {
      throw handleServiceError(error, 'ChatService', 'save message');
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
      throw handleServiceError(error, 'ChatService', 'load messages');
    }
  }

  async closeDatabase(): Promise<void> {
    try {
      await prisma.$disconnect();
      logger.info('Chat database connection closed');
    } catch (error) {
      throw handleServiceError(error, 'ChatService', 'close database connection');
    }
  }
}
