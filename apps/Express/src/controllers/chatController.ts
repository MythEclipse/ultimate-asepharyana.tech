import WebSocket from 'ws';
import { ChatService } from '../services/chatService';
import { ChatMessage } from '@asepharyana/database';
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
            messages: messages.reverse(), // Send as a single payload to reduce repetition
          })
        );
      }
    })
    .catch((error) => {
      logger.error('Failed to load messages', error);
    });

  ws.on('message', async (data) => {
    // Log raw message for debugging
    logger.debug('Raw message received:', data.toString());

    let parsedData;
    try {
      parsedData = JSON.parse(data.toString());
    } catch (error) {
      logger.error('Failed to parse message data', error);
      // Send error response to client
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(
          JSON.stringify({
            type: 'error',
            message: 'Invalid message format',
          })
        );
      }
      return;
    }

    // Validate required fields
    if (!parsedData.text) {
      logger.error('Message missing required field: text');
      // if (ws.readyState === WebSocket.OPEN) {
      //   ws.send(
      //     JSON.stringify({
      //       type: 'error',
      //       message: 'Message text is required',
      //     })
      //   );
      // }
      return;
    }

    // Construct message object with validation
    const message: ChatMessage = {
      userId: parsedData.userId || `User${Math.floor(Math.random() * 1000)}`,
      text: parsedData.text, // Guaranteed to exist due to validation
      email: parsedData.email || '',
      imageProfile: parsedData.imageProfile || '',
      imageMessage: parsedData.imageMessage || '',
      role: parsedData.role || 'guest',
      timestamp: new Date(),
      id: '',
    };

    logger.info(`Message received: ${JSON.stringify(message)}`);

    try {
      // Save message to database
      const savedMessage = await chatService.saveMessage(message);

      // Broadcast message to all clients
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
            userId: message.userId,
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
