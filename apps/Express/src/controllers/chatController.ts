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
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(
          JSON.stringify({
            type: 'history',
            messages: messages.reverse(), // Kirim sebagai satu payload untuk mengurangi pengulangan
          })
        );
      }
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
      user: parsedData.user || `User${Math.floor(Math.random() * 1000)}`,
      text: parsedData.text,
      email: parsedData.email || '',
      imageProfile: parsedData.imageProfile || '',
      imageMessage: parsedData.imageMessage || '',
      role: parsedData.role || 'guest',
      timestamp: Date.now(),
      id: undefined, // ID akan diisi oleh database
    };

    logger.info(`Message received: ${JSON.stringify(message)}`);

    try {
      // Simpan pesan ke database
      const savedMessage = await chatService.saveMessage(message);

      // Broadcast pesan ke semua client
      clients.forEach((client) => {
        if (client.readyState === WebSocket.OPEN) {
          client.send(
            JSON.stringify({
              type: 'new_message',
              message: { ...savedMessage, id: savedMessage.id },
            })
          );
        }
      });
    } catch (error) {
      logger.error('Failed to save message to database', error);
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(
          JSON.stringify({
            type: 'error',
            message: 'Failed to save message',
          })
        );
      }
    }
  });

  ws.on('close', () => {
    clients.delete(ws);
    logger.info('Client disconnected');
  });
}
