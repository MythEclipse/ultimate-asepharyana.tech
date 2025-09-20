// apps/NextJS/lib/logger.ts

import winston from 'winston';
import DailyRotateFile from 'winston-daily-rotate-file';
import util from 'util';

/**
 * Sends error details to the /api/log-error endpoint with retry logic.
 *
 * Usage:
 * ```ts
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
  maxRetries = 3,
): Promise<void> {
  let attempt = 0;
  let lastError: unknown = null;
  while (attempt < maxRetries) {
    try {
      let errorPayload: unknown;
      if (error instanceof Error) {
        errorPayload = { message: error.message, stack: error.stack };
      } else if (typeof error === 'object' && error !== null) {
        try {
          errorPayload = JSON.stringify(error);
        } catch {
          // Use util.inspect for better object representation as a fallback
          errorPayload = util.inspect(error, { depth: 3, breakLength: 120 });
        }
      } else {
        errorPayload = String(error);
      }

      await fetch('/api/log-error', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          error: errorPayload,
          info:
            typeof info === 'object' && info !== null
              ? JSON.stringify(info)
              : String(info),
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
  console.error(
    'Failed to log error to API after retries:',
    error,
    info,
    lastError,
  );
}

// Membuat format dasar yang bisa digunakan bersama
const baseFormat = winston.format.combine(
  winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
  winston.format.errors({ stack: true }), // Menampilkan stack trace untuk error
  winston.format.printf(({ timestamp, level, message, stack }) => {
    return `${timestamp} [${level}]: ${stack || message}`;
  }),
);

const transports: winston.transport[] = [];

if (process.env.NODE_ENV !== 'production') {
  transports.push(
    new winston.transports.Console({
      format: winston.format.combine(winston.format.colorize(), baseFormat),
    }),
  );
} else {
  transports.push(
    new winston.transports.Console({
      level: 'warn',
      format: baseFormat,
    }),
  );
}

const winstonLogger = winston.createLogger({
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
winstonLogger.transports.forEach((transport) => {
  if (transport instanceof winston.transports.File) {
    transport.on('error', (error) => {
      console.error('Error dalam transport file:', error);
    });
  }
});

// Helper to format log arguments
function formatLogArgs(args: unknown[]): string {
  return args
    .map((arg) => {
      if (typeof arg === 'object' && arg !== null) {
        return util.inspect(arg, { depth: 3, breakLength: Infinity });
      }
      return String(arg);
    })
    .join(' ');
}

const logger = {
  info: (...args: unknown[]) => winstonLogger.info(formatLogArgs(args)),
  warn: (...args: unknown[]) => winstonLogger.warn(formatLogArgs(args)),
  error: (...args: unknown[]) => winstonLogger.error(formatLogArgs(args)),
  debug: (...args: unknown[]) => winstonLogger.debug(formatLogArgs(args)),
  verbose: (...args: unknown[]) => winstonLogger.verbose(formatLogArgs(args)),
  silly: (...args: unknown[]) => winstonLogger.silly(formatLogArgs(args)),
  http: (...args: unknown[]) => winstonLogger.http(formatLogArgs(args)),
};

export default logger;
