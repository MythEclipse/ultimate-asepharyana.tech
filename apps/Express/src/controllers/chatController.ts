// controllers/chatController.ts
import { Socket } from 'socket.io';
import { ChatService } from '@/services/chatService';
import { MessageCreateInput, ChatMessage } from '@/type/chat';
import logger from '@/utils/logger';

interface MessageResponse {
  status: 'success' | 'error';
  message?: ChatMessage;
  error?: string;
  data?: any;
}

export default function handleConnection(socket: Socket) {
  const chatService = new ChatService();
  logger.info(`Client connected: ${socket.id}`);

  const handleDatabaseError = (error: unknown, context: string) => {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Database error in ${context}:`, {
      error: errorMessage,
      socketId: socket.id,
      stack: new Error().stack
    });
    return errorMessage;
  };

  const validateMessageInput = (data: MessageCreateInput) => {
    const trimmedData = {
      text: data.text?.trim() || '',
      user: data.user?.trim() || '',
      userId: data.userId?.trim() || ''
    };

    if (!trimmedData.userId) throw new Error('User ID is required');
    if (!trimmedData.text) throw new Error('Message text is required');
    if (!trimmedData.user) throw new Error('Username is required');

    return trimmedData;
  };

  const handleMessage = async (data: MessageCreateInput, callback: (res: MessageResponse) => void) => {
    try {
      const validatedData = validateMessageInput(data);
      const message = await chatService.createMessage(validatedData);
      
      socket.broadcast.emit('new_message', message);
      
      callback({
        status: 'success',
        message
      });
    } catch (error) {
      const errorMessage = handleDatabaseError(error, 'message handler');
      callback({
        status: 'error',
        error: errorMessage
      });
    }
  };

  const handleHistoryRequest = async (page: number, callback: (res: MessageResponse) => void) => {
    try {
      const history = await chatService.getMessagesPaginated(page);
      callback({
        status: 'success',
        data: history
      });
    } catch (error) {
      const errorMessage = handleDatabaseError(error, 'history request');
      callback({
        status: 'error',
        error: errorMessage,
        data: []
      });
    }
  };

  const handleDisconnect = async () => {
    try {
      await chatService.close();
      logger.info(`Client disconnected: ${socket.id}`);
    } catch (error) {
      handleDatabaseError(error, 'disconnect handler');
    }
  };

  socket.on('message', handleMessage);
  socket.on('request_history', ({ page }, callback) => handleHistoryRequest(page, callback));
  socket.on('disconnect', handleDisconnect);
}