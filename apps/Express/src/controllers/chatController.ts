import WebSocket from 'ws';
import { ChatService } from '@/services/chatService';
import logger from '@/utils/logger';

const clients = new Set<WebSocket>();
const chatService = new ChatService();

export default function handleConnection(ws: WebSocket) {
  clients.add(ws);
  logger.info('New client connected');

  chatService
    .loadMessages()
    .then((messages) => {
      messages.reverse().forEach((msg) => {
        ws.send(JSON.stringify(msg));
      });
    })
    .catch((error) => {
      logger.error('Failed to load messages', error);
    });

  ws.on('message', async (data) => {
    try {
      const parsedData = JSON.parse(data.toString());
      const messageData = {
        id: parsedData.id || '', // or generate a unique id
        user: parsedData.user,
        userId: parsedData.userId,
        text: parsedData.text,
        timestamp: '', // or use current date
      };

      logger.info(`Received: ${JSON.stringify(messageData)}`);

      const savedMessage = await chatService.saveMessage(messageData);
      logger.info('Saved message:', savedMessage);

      clients.forEach((client) => {
        if (client !== ws && client.readyState === WebSocket.OPEN) {
          client.send(JSON.stringify(savedMessage));
        }
      });
    } catch (error) {
      logger.error('Message processing failed:', error);
    }
  });

  ws.on('close', () => {
    clients.delete(ws);
    logger.info('Client disconnected');
  });
}