// apps/NextJS/lib/logger.ts

import winston from 'winston';
import DailyRotateFile from 'winston-daily-rotate-file';

/**
 * Sends error details to the /api/log-error endpoint with retry logic.
 *
 * Usage:
 * ```ts
 * import { logErrorToApi } from '@/lib/logger';
 * import type { ErrorInfo } from 'react';
 *
 * function logErrorToService(error: Error, info: ErrorInfo) {
 *   logErrorToApi(error, info);
 * }
 * ```
 *
 * @param error Error object or string
 * @param info Additional error info (e.g., React ErrorInfo)
 * @param maxRetries Number of retry attempts (default: 3)
 */
export async function logErrorToApi(
  error: unknown,
  info?: unknown,
  maxRetries = 3
): Promise<void> {
  let attempt = 0;
  let lastError: unknown = null;
  while (attempt < maxRetries) {
    try {
      await fetch('/api/log-error', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          error: typeof error === 'object' ? JSON.stringify(error) : String(error),
          info: typeof info === 'object' ? JSON.stringify(info) : String(info),
        }),
      });
      return;
    } catch (e) {
      lastError = e;
      attempt++;
      await new Promise((res) => setTimeout(res, 500 * attempt)); // Exponential backoff
    }
  }
  // If all retries fail, fallback to console
  console.error('Failed to log error to API after retries:', error, info, lastError);
}

// Membuat format dasar yang bisa digunakan bersama
const baseFormat = winston.format.combine(
  winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
  winston.format.errors({ stack: true }), // Menampilkan stack trace untuk error
  winston.format.printf(({ timestamp, level, message, stack }) => {
    return `${timestamp} [${level}]: ${stack || message}`;
  })
);

const transports = [];

if (process.env.NODE_ENV !== 'production') {
  // Transport untuk Console (development)
  transports.push(
    new winston.transports.Console({
      format: winston.format.combine(
        winston.format.colorize(), // Warna hanya untuk console
        baseFormat
      ),
      handleExceptions: true,
    })
  );
}

const logger = winston.createLogger({
  level: process.env.NODE_ENV === 'production' ? 'info' : 'debug',
  format: baseFormat,
  transports,
  exceptionHandlers: [
    new DailyRotateFile({
      filename: 'logs/exceptions-%DATE%.log',
      datePattern: 'YYYY-MM-DD',
      zippedArchive: true,
      maxSize: '20m',
      maxFiles: '30d',
    }),
  ],
  rejectionHandlers: [
    new DailyRotateFile({
      filename: 'logs/rejections-%DATE%.log',
      datePattern: 'YYYY-MM-DD',
      zippedArchive: true,
      maxSize: '20m',
      maxFiles: '30d',
    }),
  ],
  exitOnError: false, // Biarkan winston menangani penutupan
});

// Handle error pada transport file
logger.transports.forEach((transport) => {
  if (transport instanceof winston.transports.File) {
    transport.on('error', (error) => {
      console.error('Error dalam transport file:', error);
    });
  }
});

export default logger;
