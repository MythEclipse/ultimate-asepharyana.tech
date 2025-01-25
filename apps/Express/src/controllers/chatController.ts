// controllers/chatController.ts
import { Socket } from 'socket.io';
import { ChatService } from '@/services/chatService';
import { MessageCreateInput, ChatMessage } from '@/type/chat'; // Pastikan path benar
import logger from '@/utils/logger';

// Type untuk response callback
interface MessageResponse {
  status: 'success' | 'error';
  message?: ChatMessage;
  error?: string;
  data?: any;
}

export default function handleConnection(socket: Socket) {
  const chatService = new ChatService();

  const handleError = (error: unknown, defaultMessage: string) => {
    const message = error instanceof Error ? error.message : defaultMessage;
    logger.error(message);
    return message;
  };

  // Event handler untuk pesan baru
  const handleMessage = async (
    data: MessageCreateInput,
    callback: (res: MessageResponse) => void
  ) => {
    try {
      const validatedData = {
        text: data.text.trim(),
        user: data.user.trim(),
        userId: data.userId.trim(),
      };

      const message = await chatService.createMessage(validatedData);

      // Broadcast ke semua client kecuali pengirim
      socket.broadcast.emit('new_message', message);

      // Kirim ACK ke client pengirim
      callback({
        status: 'success',
        message: {
          id: message.id,
          text: message.text,
          user: message.user,
          timestamp: message.timestamp,
          userId: '',
        },
      });
    } catch (error) {
      const errorMessage = handleError(error, 'Failed to send message');
      callback({
        status: 'error',
        error: errorMessage,
      });
    }
  };

  // Event handler untuk request history
  const handleHistoryRequest = async (
    page: number,
    callback: (res: MessageResponse) => void
  ) => {
    try {
      const history = await chatService.getMessagesPaginated(page);
      callback({
        status: 'success',
        data: history,
      });
    } catch (error) {
      const errorMessage = handleError(error, 'Failed to load history');
      callback({
        status: 'error',
        error: errorMessage,
        data: [],
      });
    }
  };

  // Event handler untuk disconnect
  const handleDisconnect = async () => {
    try {
      await chatService.close();
      logger.info(`Client ${socket.id} disconnected`);
    } catch (error) {
      handleError(error, 'Failed to clean up connection');
    }
  };

  // Daftarkan event handlers
  socket.on('message', handleMessage);
  socket.on('request_history', ({ page }, callback) =>
    handleHistoryRequest(page, callback)
  );
  socket.on('disconnect', handleDisconnect);

  // Inisialisasi koneksi
  logger.info(`Client ${socket.id} connected`);
}
