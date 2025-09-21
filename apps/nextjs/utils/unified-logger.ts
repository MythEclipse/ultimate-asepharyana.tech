import type { ILogger, LoggerConfig, RuntimeEnvironment, ErrorInfo } from '../types/logger';

/**
 * Detects the current runtime environment
 */
function detectRuntimeEnvironment(): RuntimeEnvironment {
  // Check if we're in a browser environment
  if (typeof window !== 'undefined') {
    return 'client';
  }

  // Check if we're in a Node.js environment
  if (typeof process !== 'undefined' && process.versions && process.versions.node) {
    return 'server';
  }

  return 'unknown';
}

/**
 * Creates a client-side logger using console methods
 */
function createClientLogger(): ILogger {
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

/**
 * Creates a server-side logger using console methods with additional formatting
 * This avoids importing Winston and file system modules that cause client-side issues
 */
function createServerLogger(config: LoggerConfig = {}): ILogger {
  const {
    level = process.env.NODE_ENV === 'production' ? 'info' : 'debug',
  } = config;

  // Map log levels to numbers for filtering
  const logLevels = {
    error: 0,
    warn: 1,
    info: 2,
    http: 3,
    verbose: 4,
    debug: 5,
    silly: 6,
  };

  const currentLevel = logLevels[level as keyof typeof logLevels] ?? 2;

  function shouldLog(level: keyof typeof logLevels): boolean {
    return logLevels[level] <= currentLevel;
  }

  function formatMessage(level: string, args: unknown[]): string {
    const timestamp = new Date().toISOString();
    const message = args.map(arg =>
      typeof arg === 'object' && arg !== null
        ? JSON.stringify(arg, null, 2)
        : String(arg)
    ).join(' ');
    return `${timestamp} [${level.toUpperCase()}]: ${message}`;
  }

  return {
    error: (...args: unknown[]) => {
      if (shouldLog('error')) {
        console.error(formatMessage('error', args));
      }
    },
    warn: (...args: unknown[]) => {
      if (shouldLog('warn')) {
        console.warn(formatMessage('warn', args));
      }
    },
    info: (...args: unknown[]) => {
      if (shouldLog('info')) {
        console.info(formatMessage('info', args));
      }
    },
    debug: (...args: unknown[]) => {
      if (shouldLog('debug')) {
        console.debug(formatMessage('debug', args));
      }
    },
    verbose: (...args: unknown[]) => {
      if (shouldLog('verbose')) {
        console.log(formatMessage('verbose', args));
      }
    },
    silly: (...args: unknown[]) => {
      if (shouldLog('silly')) {
        console.log(formatMessage('silly', args));
      }
    },
    http: (...args: unknown[]) => {
      if (shouldLog('http')) {
        console.log(formatMessage('http', args));
      }
    },
  };
}

/**
 * Creates a unified logger that automatically adapts to the runtime environment
 */
function createUnifiedLogger(config?: LoggerConfig): ILogger {
  const environment = detectRuntimeEnvironment();

  switch (environment) {
    case 'client':
      return createClientLogger();

    case 'server':
      return createServerLogger(config);

    default:
      // Fallback to client logger for unknown environments
      console.warn('Unknown runtime environment, falling back to client logger');
      return createClientLogger();
  }
}

/**
 * Sends error details to the /api/log-error endpoint with retry logic.
 * This function is only available on the server side.
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
  // This function should only work on the server side
  if (detectRuntimeEnvironment() !== 'server') {
    console.warn('logErrorToApi is only available on the server side');
    return;
  }

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
          // Use util.inspect for better object representation as a fallback if available
          if (typeof process !== 'undefined' && process.versions && process.versions.node) {
            try {
              const util = require('util');
              errorPayload = util.inspect(error, { depth: 3, breakLength: 120 });
            } catch {
              errorPayload = String(error);
            }
          } else {
            errorPayload = String(error);
          }
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

// Create the default unified logger instance
const unifiedLogger = createUnifiedLogger();

// Export the logger instance and utility functions
export default unifiedLogger;
export { createUnifiedLogger, detectRuntimeEnvironment };
