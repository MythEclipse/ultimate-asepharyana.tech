import sqlite3 from 'sqlite3';
import { ChatMessage } from '../models/chatModel';
import logger from '../utils/logger';

export class ChatService {
  private static db: sqlite3.Database | null;

  constructor() {
    if (!ChatService.db) {
      ChatService.db = this.initializeDatabase();
    }
  }

  private initializeDatabase(): sqlite3.Database {
    const db =
      process.env.NODE_ENV === 'development'
        ? new sqlite3.Database(':memory:')
        : new sqlite3.Database('./database.sqlite');

    db.serialize(() => {
      db.run(`CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user TEXT NOT NULL,
                text TEXT NOT NULL,
                email TEXT,
                imageProfile TEXT,
                imageMessage TEXT,
                role TEXT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )`);
    });

    return db;
  }

  saveMessage(message: ChatMessage): Promise<void> {
    return new Promise((resolve, reject) => {
      if (!ChatService.db) {
        reject(new Error('Database not initialized'));
        return;
      }
      const query =
        'INSERT INTO messages (user, text, email, imageProfile, imageMessage, role) VALUES (?, ?, ?, ?, ?, ?)';
      ChatService.db.run(
        query,
        [
          message.user,
          message.text,
          message.email,
          message.imageProfile,
          message.imageMessage,
          message.role,
        ],
        (err) => {
          if (err) {
            reject(err);
          } else {
            resolve();
          }
        }
      );
    });
  }

  loadMessages(limit: number = 50): Promise<ChatMessage[]> {
    return new Promise((resolve, reject) => {
      if (!ChatService.db) {
        reject(new Error('Database not initialized'));
        return;
      }
      const query =
        'SELECT user, text, email, imageProfile, imageMessage, role FROM messages ORDER BY timestamp DESC LIMIT ?';
      ChatService.db.all(query, [limit], (err, rows) => {
        if (err) {
          reject(err);
        } else {
          resolve(rows as ChatMessage[]);
        }
      });
    });
  }

  closeDatabase(): Promise<void> {
    return new Promise((resolve, reject) => {
      if (ChatService.db) {
        ChatService.db.close((err) => {
          if (err) {
            reject(err);
          } else {
            ChatService.db = null;
            logger.info('Chat database connection closed');
            resolve();
          }
        });
      } else {
        resolve();
      }
    });
  }
}
