import { Server as HTTPServer } from 'http';
import { Server as SocketIOServer } from 'socket.io';
import handleConnection from '@/controllers/chatController';
import logger from '@/utils/logger';

export const initWebSocketServer = (server: HTTPServer) => {
  const io = new SocketIOServer(server);

  io.on('connection', (socket) => {
    handleConnection(socket);
  });

  logger.info('Socket.IO server initialized');
};
