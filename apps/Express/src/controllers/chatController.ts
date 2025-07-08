import WebSocket from 'ws';
import { ChatService } from '../services/chatService';
import logger from '../utils/logger';
import { v4 as uuidv4 } from 'uuid';
import { validateChatMessage } from '../utils/messageValidator';
import { broadcastMessage, sendMessageToClient } from '../utils/websocketUtils';

const clients: Set<WebSocket> = new Set();
const chatService = new ChatService();

export default function handleConnection(ws: WebSocket) {
  clients.add(ws);
  logger.info('New client connected');

  // Load recent messages and send to the new client
  chatService
    .loadMessages()
    .then((messages) => {
      sendMessageToClient(
        ws,
        {
          type: 'history',
          messages: messages.slice().reverse(),
        }
      );
    })
    .catch((error) => {
      logger.error('Failed to load messages', error);
    });

  ws.on('message', async (data) => {
    let rawMessage: string;
    if (typeof data === 'string') {
      rawMessage = data;
    } else if (Buffer.isBuffer(data)) {
      rawMessage = data.toString('utf8');
    } else {
      rawMessage = JSON.stringify(data);
    }
    logger.debug('Raw message received:', rawMessage);

    let parsedData;
    try {
      let jsonString: string;
      if (typeof data === 'string') {
        jsonString = data;
      } else if (Buffer.isBuffer(data)) {
        jsonString = data.toString('utf8');
      } else {
        jsonString = '';
      }
      parsedData = JSON.parse(jsonString);
    } catch (error) {
      logger.error('Failed to parse message data', error);
      sendMessageToClient(ws, {
        type: 'error',
        message: 'Invalid message format',
      });
      return;
    }

    const { isValid, errors, validatedMessage } = validateChatMessage(parsedData);

    if (!isValid) {
      logger.error('Message validation failed:', errors);
      sendMessageToClient(ws, {
        type: 'error',
        message: 'Message validation failed',
        errors: errors,
      });
      return;
    }

    const message = {
      ...validatedMessage!,
      id: uuidv4(),
      timestamp: new Date(),
    };

    logger.info(`Message received: ${JSON.stringify(message)}`);

    try {
      const savedMessage = await chatService.saveMessage(message);

      broadcastMessage(clients, {
        type: 'new_message',
        message: { ...savedMessage, id: savedMessage.id },
      });
    } catch (error) {
      logger.error('Failed to save message to database', error);
      sendMessageToClient(ws, {
        type: 'error',
        message: 'Failed to save message',
        userId: message.userId,
      });
    }
  });

  ws.on('close', () => {
    clients.delete(ws);
    logger.info('Client disconnected');
  });
}
