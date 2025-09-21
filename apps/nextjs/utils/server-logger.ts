import type { ILogger, LoggerConfig } from '../types/logger';

// This file should only be imported on the server side
// It uses dynamic imports to avoid webpack processing on the client

/**
 * Creates a server-side logger using Winston with file rotation and structured logging
 * This function uses dynamic imports to avoid webpack processing on the client side
 */
export function createServerLogger(config: LoggerConfig = {}): ILogger {
  // Check if we're actually on the server
  if (typeof process === 'undefined' || !process.versions || !process.versions.node) {
    console.warn('createServerLogger called on client side, returning console logger');
    return {
      error: (...args: unknown[]) => console.error(...args),
      warn: (...args: unknown[]) => console.warn(...args),
      info: (...args: unknown[]) => console.info(...args),
      debug: (...args: unknown[]) => console.debug(...args),
      verbose: (...args: unknown[]) => console.log(...args),
      silly: (...args: unknown[]) => console.log(...args),
      http: (...args: unknown[]) => console.log(...args),
    };
  }

  try {
    // Dynamically import server-only modules to avoid client-side webpack issues
    const winston = require('winston');
    const DailyRotateFile = require('winston-daily-rotate-file');

    const {
      level = process.env.NODE_ENV === 'production' ? 'info' : 'debug',
      enableColors = process.env.NODE_ENV !== 'production',
      enableFileLogging = process.env.NODE_ENV === 'production',
      logDirectory = 'logs',
    } = config;

    /**
     * Formats log arguments for consistent output
     */
    function formatLogArgs(args: unknown[]): string {
      return args
        .map((arg) => {
          if (typeof arg === 'object' && arg !== null) {
            try {
              return JSON.stringify(arg);
            } catch {
              // Fallback to util.inspect for complex objects if available
              try {
                const util = require('util');
                return util.inspect(arg, { depth: 3, breakLength: Infinity });
              } catch {
                return String(arg);
              }
            }
          }
          return String(arg);
        })
        .join(' ');
    }

    // Base format for structured logging
    const baseFormat = winston.format.combine(
      winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
      winston.format.errors({ stack: true }),
      winston.format.printf((info: any) => {
        return `${info.timestamp} [${info.level}]: ${info.stack || info.message}`;
      }),
    );

    const transports: any[] = [];

    // Console transport
    if (process.env.NODE_ENV !== 'production') {
      transports.push(
        new winston.transports.Console({
          format: enableColors
            ? winston.format.combine(winston.format.colorize(), baseFormat)
            : baseFormat,
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

    // File transports for production
    if (enableFileLogging) {
      transports.push(
        new DailyRotateFile({
          filename: `${logDirectory}/app-%DATE%.log`,
          datePattern: 'YYYY-MM-DD',
          zippedArchive: true,
          maxSize: '20m',
          maxFiles: '30d',
          level,
        }),
      );
    }

    const winstonLogger = winston.createLogger({
      level,
      format: baseFormat,
      transports,
      exceptionHandlers: enableFileLogging ? [
        new DailyRotateFile({
          filename: `${logDirectory}/exceptions-%DATE%.log`,
          datePattern: 'YYYY-MM-DD',
          zippedArchive: true,
          maxSize: '20m',
          maxFiles: '30d',
        }),
      ] : [],
      rejectionHandlers: enableFileLogging ? [
        new DailyRotateFile({
          filename: `${logDirectory}/rejections-%DATE%.log`,
          datePattern: 'YYYY-MM-DD',
          zippedArchive: true,
          maxSize: '20m',
          maxFiles: '30d',
        }),
      ] : [],
      exitOnError: false,
    });

    // Handle transport errors
    winstonLogger.transports.forEach((transport: any) => {
      if (transport instanceof winston.transports.File) {
        transport.on('error', (error: any) => {
          console.error('Error in file transport:', error);
        });
      }
    });

    return {
      error: (...args: unknown[]) => winstonLogger.error(formatLogArgs(args)),
      warn: (...args: unknown[]) => winstonLogger.warn(formatLogArgs(args)),
      info: (...args: unknown[]) => winstonLogger.info(formatLogArgs(args)),
      debug: (...args: unknown[]) => winstonLogger.debug(formatLogArgs(args)),
      verbose: (...args: unknown[]) => winstonLogger.verbose(formatLogArgs(args)),
      silly: (...args: unknown[]) => winstonLogger.silly(formatLogArgs(args)),
      http: (...args: unknown[]) => winstonLogger.http(formatLogArgs(args)),
    };
  } catch (error) {
    console.warn('Failed to create server logger, falling back to console:', error);
    // Fallback to console logging if server modules fail to load
    return {
      error: (...args: unknown[]) => console.error(...args),
      warn: (...args: unknown[]) => console.warn(...args),
      info: (...args: unknown[]) => console.info(...args),
      debug: (...args: unknown[]) => console.debug(...args),
      verbose: (...args: unknown[]) => console.log(...args),
      silly: (...args: unknown[]) => console.log(...args),
      http: (...args: unknown[]) => console.log(...args),
    };
  }
}
