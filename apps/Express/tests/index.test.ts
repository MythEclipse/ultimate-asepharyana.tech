import request from 'supertest';
import http from 'http';
import { AddressInfo } from 'net';
import express from 'express';
import { Request, Response } from 'express';
import { initWebSocketServer } from '../src/services/websocketService';
import logger from '../src/utils/logger';
import WebSocket from 'ws';
import { UserService } from '../src/services/userService';
import { User } from '../src/types/userTypes';
import dotenv from 'dotenv';

// Load environment variables
dotenv.config();
describe('index.ts tests', () => {
    let server: http.Server;
    let port: number;

    beforeAll(async () => {
        const app = express();

        app.get('/', (_req: Request, res: Response) => {
            logger.info('Received GET request at /');
            res.send('<html>Hello Test</html>');
        });

        server = http.createServer(app);

        // Initialize WebSocket server
        initWebSocketServer(server);

        // Wait for server to be ready
        await new Promise<void>((resolve) => {
            server.listen(0, () => {
                port = (server.address() as AddressInfo).port;
                logger.info(`Test server running on port: ${port}`);
                resolve();
            });
        });
    });

    afterAll(async () => {
        const userService = new UserService();
        logger.info('Closing test server');
        server.close();

        // Delete test database
        if (process.env.NODE_ENV === 'development') {
            await userService.closeDatabase();
            logger.info('Database connection closed');
        }
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

            const handleMessage = (data: WebSocket.MessageEvent) => {
                const msg = JSON.parse(data.toString());
                logger.info(`WebSocket message received: ${msg.text}`);
                if (msg.text === 'Hello from client1' && msg.email === 'client1@example.com' && msg.image === 'https://example.com/client1.png' && msg.role === 'user') {
                    receivedCount++;
                    if (receivedCount === 2) {
                        logger.info('Both clients received the message');
                        wsClient1.close();
                        wsClient2.close();
                        done();
                    }
                }
            };

            wsClient1.on('message', handleMessage);
            wsClient2.on('message', handleMessage);

            wsClient1.on('open', () => {
                logger.info('Client 1 sending message');
                const message = {
                    user: 'Client1',
                    text: 'Hello from client1',
                    email: 'client1@example.com',
                    image: 'https://example.com/client1.png',
                    role: 'user'
                };
                wsClient1.send(JSON.stringify(message));
            });
        });
    });
});

describe('UserService tests', () => {
    let userService: UserService;

    beforeAll(() => {
        logger.info('Initializing UserService for tests');
        userService = new UserService();
    });

    it('should save a new user', async () => {
        logger.info('Testing: save new user');
        const user: User = { id: '', name: 'John Doe', email: 'john@example.com', password: 'password123' };
        await userService.saveUser(user);
        const savedUser = await userService.findUser(parseInt(user.id, 10));
        logger.info({ userId: user.id }, 'User saved successfully');
        expect(savedUser).toBeDefined();
        expect(savedUser?.name).toBe('John Doe');
    });

    it('should find an existing user', async () => {
        logger.info('Testing: find existing user');
        const user: User = { id: '', name: 'Jane Doe', email: 'jane@example.com', password: 'password123' };
        await userService.saveUser(user);
        const foundUser = await userService.findUser(parseInt(user.id, 10));
        logger.info({ userId: user.id }, 'User found successfully');
        expect(foundUser).toBeDefined();
        expect(foundUser?.email).toBe('jane@example.com');
    });

    it('should update an existing user', async () => {
        logger.info('Testing: update existing user');
        const user: User = { id: '', name: 'Jake Doe', email: 'jake@example.com', password: 'password123' };
        await userService.saveUser(user);
        const updatedData = { name: 'Jake Smith', email: 'jake.smith@example.com', password: 'newpassword123' };
        await userService.updateUser(parseInt(user.id, 10), updatedData);
        const updatedUser = await userService.findUser(parseInt(user.id, 10));
        logger.info({ userId: user.id }, 'User updated successfully');
        expect(updatedUser).toBeDefined();
        expect(updatedUser?.name).toBe('Jake Smith');
    });

    it('should delete an existing user', async () => {
        logger.info('Testing: delete existing user');
        const user: User = { id: '', name: 'Jill Doe', email: 'jill@example.com', password: 'password123' };
        await userService.saveUser(user);
        await userService.deleteUser(parseInt(user.id, 10));
        const deletedUser = await userService.findUser(parseInt(user.id, 10));
        logger.info({ userId: user.id }, 'User deleted successfully');
        expect(deletedUser).toBeUndefined();
    });
});