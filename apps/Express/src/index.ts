import express from 'express';
import http from 'http';
import { initWebSocketServer } from './services/websocketService';
import logger from './utils/logger';
import { errorHandler } from './utils/errorHandler';
import { config } from './config/config';
import { setupRoutes } from './routes';

const app = express();
const server = http.createServer(app);
const PORT = config.port;

// Configure Express middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Set up routes
setupRoutes(app);

// Initialize WebSocket server
initWebSocketServer(server);

// Error handling middleware
app.use(errorHandler);

// Start HTTP server
server.listen(PORT, () => {
  logger.info(`Server running at http://localhost:${PORT}`);
  logger.info(`WebSocket server is running`);
});
