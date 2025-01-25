import { Server as HTTPServer } from 'http';
import { Server as SocketIOServer, Socket } from 'socket.io';
import handleConnection from '@/controllers/chatController';
import logger from '@/utils/logger';

export const initWebSocketServer = (httpServer: HTTPServer): SocketIOServer => {
  const io = new SocketIOServer(httpServer, {
    cors: {
      origin: 'https://asepharyana.cloud',
      methods: ['GET', 'POST'],
    },
  });

  io.on('connection', (socket: Socket) => {
    logger.info(`New client connected: ${socket.id}`);
    handleConnection(socket);
  });

  return io;
};
