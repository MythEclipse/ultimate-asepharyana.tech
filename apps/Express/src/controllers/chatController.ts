import WebSocket from 'ws';
import { ChatMessage as PrismaChatMessage } from '@prisma/client';

interface ChatMessage extends PrismaChatMessage {
  user: string;
  userId: string; // Add userId
}
import { ChatService } from '@/services/chatService';
import logger from '@/utils/logger';

const clients: Set<WebSocket> = new Set();
const chatService = new ChatService();

export default function handleConnection(ws: WebSocket) {
  clients.add(ws);
  logger.info('New client connected');

  // Load recent messages and send to the new client
  chatService
    .loadMessages()
    .then((messages) => {
      messages.reverse().forEach((message) => {
        ws.send(JSON.stringify(message));
      });
    })
    .catch((error) => {
      logger.error('Failed to load messages', error);
    });

  ws.on('message', async (data) => {
    const parsedData = JSON.parse(data.toString());
    const message: ChatMessage = {
      id: '', // Prisma will auto-generate the ID
      user: parsedData.user, // Use the user field from the parsed data
      text: parsedData.text,
      timestamp: new Date(),
      userId: parsedData.userId, // Include userId
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
      if (client.readyState === WebSocket.OPEN) {
        client.send(JSON.stringify(message));
      }
    });
  });

  ws.on('close', () => {
    clients.delete(ws);
    logger.info('Client disconnected');
  });
}
