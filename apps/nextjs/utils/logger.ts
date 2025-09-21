// Client-side logger - simple console logging
const clientLogger = {
  info: (...args: unknown[]) => console.log(...args),
  warn: (...args: unknown[]) => console.warn(...args),
  error: (...args: unknown[]) => console.error(...args),
  debug: (...args: unknown[]) => console.debug(...args),
  verbose: (...args: unknown[]) => console.log(...args),
  silly: (...args: unknown[]) => console.log(...args),
  http: (...args: unknown[]) => console.log(...args),
};

export default clientLogger;
