import { Server as HTTPServer } from 'http';
import { WebSocketServer } from 'ws';
import type { WebSocket as WSWebSocket } from 'ws';
import handleConnection from '@/controllers/chatController';
import logger from '@/utils/logger';

export const initWebSocketServer = (server: HTTPServer) => {
  const wss = new WebSocketServer({ server });

  wss.on('connection', (ws: WSWebSocket) => {
    handleConnection(ws);
  });

  logger.info('WebSocket server initialized');
};
