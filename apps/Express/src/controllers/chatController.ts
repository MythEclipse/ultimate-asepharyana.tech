import { Socket } from 'socket.io';
import { ChatMessage as PrismaChatMessage } from '@prisma/client';

interface ChatMessage extends PrismaChatMessage {
  user: string;
  userId: string; // Add userId
}
import { ChatService } from '@/services/chatService';
import logger from '@/utils/logger';

const clients: Set<Socket> = new Set();
const chatService = new ChatService();

export default function handleConnection(socket: Socket) {
  clients.add(socket);
  logger.info('New client connected');

  // Load recent messages and send to the new client
  chatService
    .loadMessages()
    .then((messages) => {
      messages.reverse().forEach((message) => {
        socket.emit('message', message);
      });
    })
    .catch((error) => {
      logger.error('Failed to load messages', error);
    });

  socket.on('message', async (data) => {
    const message: ChatMessage = {
      id: '', // Prisma will auto-generate the ID
      user: data.user, // Use the user field from the parsed data
      text: data.text,
      timestamp: new Date(),
      userId: data.userId, // Include userId
    };

    logger.info(`Message received: ${JSON.stringify(message)}`);

    // Save message to database
    try {
      await chatService.saveMessage(message);
      logger.info('Message saved to database');
    } catch (error) {
      logger.error('Failed to save message to database', error);
    }

    // Broadcast message to all clients
    clients.forEach((client) => {
      client.emit('message', message);
    });
  });

  socket.on('disconnect', () => {
    clients.delete(socket);
    logger.info('Client disconnected');
  });
}
