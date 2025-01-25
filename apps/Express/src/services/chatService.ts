import { PrismaClient, ChatMessage } from '@prisma/client';
import logger from '@/utils/logger';

const prisma = new PrismaClient();

export class ChatService {
  async saveMessage(
    message: Omit<ChatMessage, 'id' | 'timestamp'>
  ): Promise<ChatMessage> {
    try {
      return await prisma.chatMessage.create({ data: message });
    } catch (error) {
      logger.error('Save message error:', error);
      throw error;
    }
  }

  async loadMessages(limit = 50): Promise<ChatMessage[]> {
    return prisma.chatMessage.findMany({
      orderBy: { timestamp: 'asc' },
      take: limit,
    });
  }

  async close(): Promise<void> {
    await prisma.$disconnect();
  }
}
