import { Server as HTTPServer } from 'http';
import WebSocket from 'ws';
import handleConnection from '@/controllers/chatController';
import logger from '@/utils/logger';

export const initWebSocketServer = (server: HTTPServer) => {
  const wss = new WebSocket.Server({ server });

  wss.on('connection', (ws) => {
    handleConnection(ws);
  });

  logger.info('WebSocket server initialized');
};
