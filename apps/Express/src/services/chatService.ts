import { PrismaClient, ChatMessage } from '@asepharyana/database';
import logger from '@/utils/logger';

export class ChatService {
  private prisma: PrismaClient;

  constructor() {
    this.prisma = new PrismaClient();
  }

  async saveMessage(message: ChatMessage): Promise<void> {
    try {
      await this.prisma.chatMessage.create({
        data: {
          text: message.text,
          userId: message.userId, // Include userId
        },
      });
    } catch (error) {
      logger.error('Failed to save message to database', error);
      throw error;
    }
  }

  async loadMessages(limit: number = 50): Promise<ChatMessage[]> {
    try {
      return await this.prisma.chatMessage.findMany({
        orderBy: { timestamp: 'desc' },
        take: limit,
      });
    } catch (error) {
      logger.error('Failed to load messages from database', error);
      throw error;
    }
  }

  async closeDatabase(): Promise<void> {
    try {
      await this.prisma.$disconnect();
      logger.info('Prisma database connection closed');
    } catch (error) {
      logger.error('Failed to close Prisma database connection', error);
      throw error;
    }
  }
}
