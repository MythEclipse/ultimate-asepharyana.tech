import WebSocket from 'ws';
import { ChatMessage } from '../models/chatModel';
import { ChatService } from '../services/chatService';
import logger from '../utils/logger';

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
    let parsedData;
    try {
      parsedData = JSON.parse(data.toString());
    } catch (error) {
      logger.error('Failed to parse message data', error);
      return;
    }

    const message: ChatMessage = {
      user: parsedData.user || `${Math.floor(Math.random() * 1000)}`,
      text: parsedData.text,
      email: parsedData.email || '',
      imageProfile: parsedData.imageProfile || '',
      imageMessage: parsedData.imageMessage || '',
      role: parsedData.role || 'guest',
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
