import WebSocket from 'ws';
import { ChatService } from '@/services/chatService';
import logger from '@/utils/logger';

const HEARTBEAT_INTERVAL = 30000;
const clients = new Set<WebSocket>();
const chatService = new ChatService();

class ConnectionManager {
  private activeConnections = 0;

  add(client: WebSocket) {
    clients.add(client);
    this.activeConnections++;
    this.logStats();
  }

  remove(client: WebSocket) {
    if (clients.delete(client)) {
      this.activeConnections--;
      this.logStats();
    }
  }

  private logStats() {
    logger.info(`Active connections: ${this.activeConnections}`);
  }

  broadcast(message: object) {
    const data = JSON.stringify(message);
    clients.forEach(client => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(data);
      }
    });
  }
}

const manager = new ConnectionManager();

export default function handleConnection(ws: WebSocket) {
  let heartbeat: NodeJS.Timeout;

  const cleanup = () => {
    clearInterval(heartbeat);
    manager.remove(ws);
    logger.info('Client disconnected');
  };

  const sendHeartbeat = () => {
    if (ws.readyState === WebSocket.OPEN) {
      ws.ping();
    }
  };

  ws.on('open', () => {
    manager.add(ws);
    logger.info('New client connected');
    heartbeat = setInterval(sendHeartbeat, HEARTBEAT_INTERVAL);
  });

  ws.on('pong', () => {
    // Reset timeout jika diperlukan
  });

  ws.on('close', cleanup);
  ws.on('error', cleanup);

  ws.on('message', async (data) => {
    try {
      const parsedData = JSON.parse(data.toString());
      const message = {
        text: parsedData.text,
        userId: parsedData.userId,
        user: parsedData.user,
        timestamp: new Date(),
      };

      const savedMessage = await chatService.saveMessage(message);
      manager.broadcast({
        ...savedMessage,
        timestamp: savedMessage.timestamp.getTime(),
      });
    } catch (error) {
      logger.error('Message handling error:', error);
    }
  });
}