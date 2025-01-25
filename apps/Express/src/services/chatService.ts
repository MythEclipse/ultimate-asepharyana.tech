import { PrismaClient, ChatMessage } from '@prisma/client';
import logger from '@/utils/logger';

const prisma = new PrismaClient();

export class ChatService {
  async saveMessage(
    message: Omit<ChatMessage, 'id' | 'timestamp'>
  ): Promise<ChatMessage> {
    try {
      return await prisma.chatMessage.create({
        data: {
          text: message.text,
          userId: message.userId,
          user: message.user,
        },
      });
    } catch (error) {
      logger.error('Failed to save message:', error);
      throw error;
    }
  }

  async loadMessages(limit = 100): Promise<ChatMessage[]> {
    return prisma.chatMessage.findMany({
      orderBy: { timestamp: 'asc' },
      take: limit,
    });
  }

  async close() {
    await prisma.$disconnect();
  }
}
