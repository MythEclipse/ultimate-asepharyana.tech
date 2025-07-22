// src/tests/index.test.ts

// Mocks must be at the top of the file before any imports of the mocked module
jest.mock('../services/chatService', () => {
  // Use a factory function to return the mocked module
  const mockChatService = {
    // Mock the constructor
    ChatService: jest.fn().mockImplementation(() => ({
      loadMessages: jest.fn().mockResolvedValue([]), // Return an empty array for history
      saveMessage: jest.fn().mockImplementation((msg) => Promise.resolve({ ...msg, id: 'mock-id', timestamp: new Date() })),
      closeDatabase: jest.fn().mockResolvedValue(undefined),
    })),
  };
  return mockChatService;
});

import request from 'supertest';
import http from 'http';
import { AddressInfo } from 'net';
import express from 'express';
import { Request, Response } from 'express';
import { initWebSocketServer } from '../services/websocketService';
import logger from '../utils/logger';
import WebSocket from 'ws';
import dotenv from 'dotenv';
import { ChatService } from '../services/chatService'; // This will now import the mocked version

const MockChatService = ChatService as jest.MockedClass<typeof ChatService>;


dotenv.config();

describe('index.ts tests', () => {
  let server: http.Server;
  let port: number;
  let chatService: ChatService; // Reverted to let

  beforeAll(async () => {
    const app = express();
    app.get('/', (_req: Request, res: Response) => {
      logger.info('Received GET request at /');
      res.send('<html>Hello Test</html>');
    });
    server = http.createServer(app);
    initWebSocketServer(server);
    await new Promise<void>((resolve) => {
      server.listen(0, () => {
        port = (server.address() as AddressInfo).port;
        logger.info(`Test server running on port: ${port}`);
        resolve();
      });
    });
    // Now, chatService in this test file refers to the mocked one.
    // The instance inside chatController.ts will also use the mocked constructor.
    chatService = new ChatService(); // Instantiate here to get the mocked version for afterAll
  });

  afterAll(async () => {
    logger.info('Closing test server');
    await chatService.closeDatabase();
    server.close();
  });

  it('should respond to the root path with HTML content', async () => {
    logger.info('Testing root path /');
    const response = await request(`http://localhost:${port}`).get('/');
    logger.info(`Response status: ${response.status}`);
    logger.info(`Response text: ${response.text}`);
    expect(response.status).toBe(200);
    expect(response.text).toContain('<html>');
  });

  it('should initialize websocket service', () => {
    logger.info('Testing WebSocket service initialization');
    expect(server).toBeDefined();
  });

  it('should return 404 for an unknown path', async () => {
    logger.info('Testing unknown path /unknown');
    const response = await request(`http://localhost:${port}`).get('/unknown');
    logger.info(`Response status: ${response.status}`);
    expect(response.status).toBe(404);
  });

  describe('WebSocket tests', () => {
    it('should accept a new WebSocket connection', (done) => {
      logger.info('Testing WebSocket connection');
      const wsClient = new WebSocket(`ws://localhost:${port}`);
      wsClient.on('open', () => {
        logger.info('WebSocket connection established');
        wsClient.close();
        done();
      });
    });

    it('should broadcast messages to all clients', (done) => {
      logger.info('Testing WebSocket message broadcasting');
      const wsClient1 = new WebSocket(`ws://localhost:${port}`);
      const wsClient2 = new WebSocket(`ws://localhost:${port}`);
      let receivedCount = 0;

      const handleMessage = (data: WebSocket.RawData) => {
        const parsed = JSON.parse(data.toString());
        if (parsed.type === 'new_message') {
          const msg = parsed.message;
          logger.info(`WebSocket message received: ${msg.text}`);
          if (
            msg.text === 'Hello from client1' &&
            msg.email === 'client1@example.com' &&
            msg.imageProfile === 'https://example.com/client1-profile.png' &&
            msg.imageMessage === 'https://example.com/client1-message.png' &&
            msg.role === 'user'
          ) {
            receivedCount++;
            if (receivedCount === 2) {
              logger.info('Both clients received the message');
              wsClient1.close();
              wsClient2.close();
              done();
            }
          }
        }
      };

      wsClient1.on('message', handleMessage);
      wsClient2.on('message', handleMessage);

      wsClient1.on('open', () => {
        logger.info('Client 1 sending message');
        const message = {
          userId: 'Client1',
          text: 'Hello from client1',
          email: 'client1@example.com',
          imageProfile: 'https://example.com/client1-profile.png',
          imageMessage: 'https://example.com/client1-message.png',
          role: 'user',
        };
        wsClient1.send(JSON.stringify(message));
      });
    }, 20000);
  });
});
