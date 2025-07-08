import WebSocket from 'ws';
import http from 'http';
import express from 'express';
import { AddressInfo } from 'net';
import { initWebSocketServer } from '../services/websocketService';
import { ChatService } from '../services/chatService';
import logger from '../utils/logger';

// Mock ChatService to control database interactions
jest.mock('../services/chatService');
const MockChatService = ChatService as jest.MockedClass<typeof ChatService>;

describe('WebSocket Integration Tests', () => {
  let server: http.Server;
  let port: number;

  beforeAll((done) => {
    const app = express();
    server = http.createServer(app);
    initWebSocketServer(server);

    server.listen(0, () => {
      port = (server.address() as AddressInfo).port;
      logger.info(`Test WebSocket server running on port: ${port}`);
      done();
    });
  });

  afterAll((done) => {
    logger.info('Closing test WebSocket server');
    server.close(done);
  });

  beforeEach(() => {
    // Clear all mocks before each test
    jest.clearAllMocks();
    // Ensure loadMessages always returns a promise
    MockChatService.prototype.loadMessages.mockResolvedValue([]);
  });

  it('should establish a WebSocket connection', (done) => {
    const ws = new WebSocket(`ws://localhost:${port}`);

    ws.on('open', () => {
      expect(ws.readyState).toBe(WebSocket.OPEN);
      ws.close();
      done();
    });

    ws.on('error', (err) => {
      done(err);
    });
  });

  it('should receive historical messages upon connection', (done) => {
    const mockMessages = [
      { id: '1', userId: 'user1', text: 'hello', timestamp: new Date().toISOString() },
      { id: '2', userId: 'user2', text: 'hi', timestamp: new Date().toISOString() },
    ];
    MockChatService.prototype.loadMessages.mockResolvedValueOnce(mockMessages);

    const ws = new WebSocket(`ws://localhost:${port}`);

    ws.on('message', (message) => {
      const parsed = JSON.parse(message.toString());
      if (parsed.type === 'history') {
        expect(parsed.messages).toEqual(mockMessages.reverse()); // Messages are reversed in controller
        expect(MockChatService.prototype.loadMessages).toHaveBeenCalledTimes(1);
        ws.close();
        done();
      }
    });

    ws.on('error', (err) => {
      done(err);
    });
  });

  it('should send and receive a new message', (done) => {
    const ws1 = new WebSocket(`ws://localhost:${port}`);
    const ws2 = new WebSocket(`ws://localhost:${port}`);

    const testMessage = {
      userId: 'testUser',
      text: 'This is a test message',
      email: 'test@example.com',
      imageProfile: '',
      imageMessage: '',
      role: 'user',
    };

    MockChatService.prototype.saveMessage.mockImplementation((msg) =>
      Promise.resolve({ ...msg, id: 'mock-id', timestamp: new Date() })
    );

    let ws1Open = false;
    let ws2Open = false;

    const checkBothOpen = () => {
      if (ws1Open && ws2Open) {
        ws1.send(JSON.stringify(testMessage));
      }
    };

    ws1.on('open', () => {
      ws1Open = true;
      checkBothOpen();
    });

    ws2.on('open', () => {
      ws2Open = true;
      checkBothOpen();
    });

    let messagesReceived = 0;
    const expectedMessages = 2; // One for each client

    const handleMessage = (message: WebSocket.RawData) => {
      const parsed = JSON.parse(message.toString());
      if (parsed.type === 'new_message') {
        expect(parsed.message.text).toBe(testMessage.text);
        expect(parsed.message.userId).toBe(testMessage.userId);
        messagesReceived++;
        if (messagesReceived === expectedMessages) {
          expect(MockChatService.prototype.saveMessage).toHaveBeenCalledTimes(1);
          ws1.close();
          ws2.close();
          done();
        }
      }
    };

    ws1.on('message', handleMessage);
    ws2.on('message', handleMessage);

    ws1.on('error', (err) => done(err));
    ws2.on('error', (err) => done(err));
  }, 10000);

  it('should handle invalid message format', (done) => {
    const ws = new WebSocket(`ws://localhost:${port}`);

    ws.on('open', () => {
      ws.send('invalid json');
    });

    ws.on('message', (message) => {
      const parsed = JSON.parse(message.toString());
      if (parsed.type === 'error') {
        expect(parsed.message).toBe('Invalid message format');
        ws.close();
        done();
      }
    });

    ws.on('error', (err) => {
      done(err);
    });
  });

  it('should handle message missing required fields', (done) => {
    const ws = new WebSocket(`ws://localhost:${port}`);

    ws.on('open', () => {
      ws.send(JSON.stringify({ userId: 'test' })); // Missing text field
    });

    ws.on('message', (message) => {
      const parsed = JSON.parse(message.toString());
      if (parsed.type === 'error') {
        expect(parsed.message).toBe('Message validation failed');
        expect(parsed.errors).toContain('Message text is required and must be a non-empty string.');
        ws.close();
        done();
      }
    });

    ws.on('error', (err) => {
      done(err);
    });
  });
});
