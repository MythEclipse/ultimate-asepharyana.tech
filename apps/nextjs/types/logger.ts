/**
 * Log levels supported by the unified logger
 */
export type LogLevel =
  | 'error'
  | 'warn'
  | 'info'
  | 'debug'
  | 'verbose'
  | 'silly'
  | 'http';

/**
 * Logger interface that provides consistent API across client and server environments
 */
export interface ILogger {
  /**
   * Log an error message
   */
  error: (...args: unknown[]) => void;

  /**
   * Log a warning message
   */
  warn: (...args: unknown[]) => void;

  /**
   * Log an info message
   */
  info: (...args: unknown[]) => void;

  /**
   * Log a debug message
   */
  debug: (...args: unknown[]) => void;

  /**
   * Log a verbose message
   */
  verbose: (...args: unknown[]) => void;

  /**
   * Log a silly message
   */
  silly: (...args: unknown[]) => void;

  /**
   * Log an HTTP message
   */
  http: (...args: unknown[]) => void;
}

/**
 * Error information for API logging
 */
export interface ErrorInfo {
  componentStack?: string;
  [key: string]: unknown;
}

/**
 * Configuration for the unified logger
 */
export interface LoggerConfig {
  /**
   * Minimum log level to output
   */
  level?: LogLevel;

  /**
   * Whether to enable colored output (server only)
   */
  enableColors?: boolean;

  /**
   * Whether to log to files (server only)
   */
  enableFileLogging?: boolean;

  /**
   * Directory for log files (server only)
   */
  logDirectory?: string;
}

/**
 * Runtime environment detection
 */
export type RuntimeEnvironment = 'client' | 'server' | 'unknown';
