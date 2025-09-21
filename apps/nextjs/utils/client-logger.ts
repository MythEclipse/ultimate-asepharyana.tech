import type { ILogger } from '../types/logger';

/**
 * Creates a client-side logger using console methods
 */
export function createClientLogger(): ILogger {
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
