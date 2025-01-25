import sqlite3 from 'sqlite3';
import dotenv from 'dotenv';
import logger from '../utils/logger';
import { User } from '../types/userTypes';

dotenv.config();

export class UserService {
    private static db: sqlite3.Database | null;

    constructor() {
        if (!UserService.db) {
            UserService.db = this.initializeDatabase();
        }
    }

    private initializeDatabase(): sqlite3.Database {
        const db = process.env.NODE_ENV === 'development'
            ? new sqlite3.Database(':memory:')
            : new sqlite3.Database('./database.sqlite');

        logger.info(process.env.NODE_ENV === 'development'
            ? 'Using in-memory SQLite database'
            : 'Using file-based SQLite database: database.sqlite');

        db.serialize(() => {
            db.run(`CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
            )`);
        });

        return db;
    }

    findUser(userId: number): Promise<User | undefined> {
        return new Promise((resolve, reject) => {
            if (!UserService.db) {
                reject(new Error('Database not initialized'));
                return;
            }
            const query = 'SELECT * FROM users WHERE id = ?';
            UserService.db.get(query, [userId], (err, row) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(row as User);
                }
            });
        });
    }

    saveUser(user: User): Promise<void> {
        return new Promise((resolve, reject) => {
            if (!UserService.db) {
                reject(new Error('Database not initialized'));
                return;
            }
            const query = 'INSERT INTO users (name, email, password) VALUES (?, ?, ?)';
            UserService.db.run(query, [user.name, user.email, user.password], function (err) {
                if (err) {
                    reject(err);
                } else {
                    user.id = this.lastID.toString();
                    resolve();
                }
            });
        });
    }

    updateUser(userId: number, userData: Partial<User>): Promise<void> {
        return new Promise((resolve, reject) => {
            if (!UserService.db) {
                reject(new Error('Database not initialized'));
                return;
            }
            const query = 'UPDATE users SET name = ?, email = ?, password = ? WHERE id = ?';
            UserService.db.run(query, [userData.name, userData.email, userData.password, userId], (err) => {
                if (err) {
                    reject(err);
                } else {
                    resolve();
                }
            });
        });
    }

    deleteUser(userId: number): Promise<void> {
        return new Promise((resolve, reject) => {
            if (!UserService.db) {
                reject(new Error('Database not initialized'));
                return;
            }
            const query = 'DELETE FROM users WHERE id = ?';
            UserService.db.run(query, [userId], (err) => {
                if (err) {
                    reject(err);
                } else {
                    resolve();
                }
            });
        });
    }
    closeDatabase(): Promise<void> {
        return new Promise((resolve, reject) => {
            if (UserService.db) {
                UserService.db.close((err) => {
                    if (err) {
                        reject(err);
                    } else {
                        UserService.db = null;
                        logger.info('Database connection closed');
                        resolve();
                    }
                });
            } else {
                resolve();
            }
        });
    }
    
}