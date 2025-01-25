import express from 'express';
import http from 'http';
import dotenv from 'dotenv';
import { initWebSocketServer } from './services/websocketService';
import { setUserRoutes } from './routes/userRoutes';
import { setChatRoutes } from './routes/chatRoutes';
import logger from './utils/logger';

dotenv.config();

const app = express();
const server = http.createServer(app);
const PORT = 4091;

// Configure Express middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Set up routes
setUserRoutes(app);
setChatRoutes(app);

// Initialize WebSocket server
initWebSocketServer(server);

// Start HTTP server
server.listen(PORT, () => {
  logger.info(`Server running at http://localhost:${PORT}`);
  logger.info(`WebSocket server is running`);
});
