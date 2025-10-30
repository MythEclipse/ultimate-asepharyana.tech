/**
 * Unified Redis Client
 * Provides centralized Redis operations with connection management, error handling, and caching
 */

import { createClient, RedisClientType } from 'redis';
import logger from '../utils/unified-logger';
import { toAppError, logError } from '../utils/error-handler';

export interface RedisConfig {
  host: string;
  port: number;
  password?: string;
  db?: number;
  maxRetries: number;
  retryDelay: number;
  enableOfflineQueue: boolean;
  cacheTtl: number;
}

export interface RedisOperationResult<T> {
  success: boolean;
  data?: T;
  error?: Error;
  cached?: boolean;
}

export class RedisClient {
  private client: RedisClientType;
  private config: RedisConfig;
  private isConnected = false;
  private connectionPromise: Promise<void> | null = null;

  constructor(config: Partial<RedisConfig> = {}) {
    this.config = {
      host: process.env.REDIS_HOST || '127.0.0.1',
      port: parseInt(process.env.REDIS_PORT || '6379'),
      password: process.env.REDIS_PASSWORD || undefined,
      db: 0,
      maxRetries: 3,
      retryDelay: 1000,
      enableOfflineQueue: true,
      cacheTtl: 3600, // 1 hour
      ...config,
    };

    this.client = createClient({
      socket: {
        host: this.config.host,
        port: this.config.port,
      },
      password: this.config.password,
      database: this.config.db,
      disableOfflineQueue: !this.config.enableOfflineQueue,
    });

    this.setupEventHandlers();
  }

  /**
   * Set up Redis client event handlers
   */
  private setupEventHandlers(): void {
    this.client.on('error', (err) => {
      logger.error('[RedisClient] Connection error:', err);
      this.isConnected = false;
    });

    this.client.on('connect', () => {
      logger.info('[RedisClient] Connected successfully');
      this.isConnected = true;
    });

    this.client.on('ready', () => {
      logger.info('[RedisClient] Client ready');
      this.isConnected = true;
    });

    this.client.on('end', () => {
      logger.info('[RedisClient] Connection ended');
      this.isConnected = false;
    });

    this.client.on('reconnecting', () => {
      logger.info('[RedisClient] Reconnecting...');
    });
  }

  /**
   * Connect to Redis with retry logic
   */
  async connect(): Promise<void> {
    if (this.connectionPromise) {
      return this.connectionPromise;
    }

    this.connectionPromise = this.attemptConnection();

    try {
      await this.connectionPromise;
    } finally {
      this.connectionPromise = null;
    }
  }

  /**
   * Attempt to connect with retry logic
   */
  private async attemptConnection(): Promise<void> {
    for (let attempt = 1; attempt <= this.config.maxRetries; attempt++) {
      try {
        await this.client.connect();
        logger.info(
          `[RedisClient] Connected successfully on attempt ${attempt}`,
        );
        return;
      } catch (error) {
        logger.error(
          `[RedisClient] Connection attempt ${attempt} failed:`,
          error,
        );

        if (attempt === this.config.maxRetries) {
          throw toAppError(error, {
            operation: 'connect',
            attempt,
            maxRetries: this.config.maxRetries,
          });
        }

        // Wait before retrying
        await new Promise((resolve) =>
          setTimeout(resolve, this.config.retryDelay * attempt),
        );
      }
    }
  }

  /**
   * Disconnect from Redis
   */
  async disconnect(): Promise<void> {
    try {
      await this.client.disconnect();
      logger.info('[RedisClient] Disconnected successfully');
    } catch (error) {
      logError(toAppError(error), { operation: 'disconnect' });
      throw error;
    }
  }

  /**
   * Check if client is connected
   */
  isReady(): boolean {
    return this.isConnected && this.client.isReady;
  }

  /**
   * Execute a Redis operation with error handling and retry logic
   */
  private async executeOperation<T>(
    operation: () => Promise<T>,
    operationName: string,
    key?: string,
  ): Promise<RedisOperationResult<T>> {
    try {
      if (!this.isReady()) {
        await this.connect();
      }

      const result = await operation();
      return { success: true, data: result };
    } catch (error) {
      const appError = toAppError(error, { operation: operationName, key });
      logError(appError);
      return { success: false, error: appError };
    }
  }

  /**
   * Get value from Redis with optional caching
   */
  async get<T>(key: string): Promise<T | null> {
    const result = await this.executeOperation(
      () => this.client.get(key) as Promise<string | null>,
      'get',
      key,
    );

    if (result.success && result.data !== null && result.data !== undefined) {
      try {
        return JSON.parse(result.data) as T;
      } catch {
        return result.data as T;
      }
    }

    return null;
  }

  /**
   * Set value in Redis with optional expiration
   */
  async set(
    key: string,
    value: unknown,
    options?: { EX?: number; PX?: number; NX?: boolean; XX?: boolean },
  ): Promise<boolean> {
    const serializedValue =
      typeof value === 'string' ? value : JSON.stringify(value);

    const result = await this.executeOperation(
      () =>
        this.client.set(key, serializedValue, options) as Promise<
          string | null
        >,
      'set',
      key,
    );

    return result.success && result.data === 'OK';
  }

  /**
   * Delete a key from Redis
   */
  async del(key: string | string[]): Promise<number> {
    const keys = Array.isArray(key) ? key : [key];

    const result = await this.executeOperation(
      () => this.client.del(keys) as Promise<number>,
      'del',
      keys.join(','),
    );

    return result.success ? result.data || 0 : 0;
  }

  /**
   * Check if a key exists in Redis
   */
  async exists(key: string): Promise<boolean> {
    const result = await this.executeOperation(
      () => this.client.exists(key) as Promise<number>,
      'exists',
      key,
    );

    return result.success ? (result.data || 0) > 0 : false;
  }

  /**
   * Set a key to expire after a given number of seconds
   */
  async expire(key: string, seconds: number): Promise<boolean> {
    const result = await this.executeOperation(
      () => this.client.expire(key, seconds) as Promise<number>,
      'expire',
      key,
    );

    return result.success ? (result.data || 0) > 0 : false;
  }

  /**
   * Get the time to live for a key
   */
  async ttl(key: string): Promise<number> {
    const result = await this.executeOperation(
      () => this.client.ttl(key) as Promise<number>,
      'ttl',
      key,
    );

    return result.success ? result.data || -2 : -2;
  }

  /**
   * Get all keys matching a pattern
   */
  async keys(pattern: string): Promise<string[]> {
    const result = await this.executeOperation(
      () => this.client.keys(pattern) as Promise<string[]>,
      'keys',
      pattern,
    );

    return result.success ? result.data || [] : [];
  }

  /**
   * Increment a counter
   */
  async incr(key: string): Promise<number> {
    const result = await this.executeOperation(
      () => this.client.incr(key) as Promise<number>,
      'incr',
      key,
    );

    return result.success ? result.data || 0 : 0;
  }

  /**
   * Decrement a counter
   */
  async decr(key: string): Promise<number> {
    const result = await this.executeOperation(
      () => this.client.decr(key) as Promise<number>,
      'decr',
      key,
    );

    return result.success ? result.data || 0 : 0;
  }

  /**
   * Add a value to a set
   */
  async sadd(key: string, members: string | string[]): Promise<number> {
    const memberArray = Array.isArray(members) ? members : [members];

    const result = await this.executeOperation(
      () => this.client.sAdd(key, memberArray) as Promise<number>,
      'sadd',
      key,
    );

    return result.success ? result.data || 0 : 0;
  }

  /**
   * Get all members of a set
   */
  async smembers(key: string): Promise<string[]> {
    const result = await this.executeOperation(
      () => this.client.sMembers(key) as Promise<string[]>,
      'smembers',
      key,
    );

    return result.success ? result.data || [] : [];
  }

  /**
   * Check if a member exists in a set
   */
  async sismember(key: string, member: string): Promise<boolean> {
    const result = await this.executeOperation(
      () => this.client.sIsMember(key, member) as Promise<number>,
      'sismember',
      key,
    );

    return result.success ? (result.data || 0) > 0 : false;
  }

  /**
   * Add a value to a list
   */
  async lpush(key: string, values: string | string[]): Promise<number> {
    const valueArray = Array.isArray(values) ? values : [values];

    const result = await this.executeOperation(
      () => this.client.lPush(key, valueArray) as Promise<number>,
      'lpush',
      key,
    );

    return result.success ? result.data || 0 : 0;
  }

  /**
   * Get a range of values from a list
   */
  async lrange(key: string, start: number, stop: number): Promise<string[]> {
    const result = await this.executeOperation(
      () => this.client.lRange(key, start, stop) as Promise<string[]>,
      'lrange',
      key,
    );

    return result.success ? result.data || [] : [];
  }

  /**
   * Get the length of a list
   */
  async llen(key: string): Promise<number> {
    const result = await this.executeOperation(
      () => this.client.lLen(key) as Promise<number>,
      'llen',
      key,
    );

    return result.success ? result.data || 0 : 0;
  }

  /**
   * Execute a transaction (multi/exec)
   */
  async transaction(
    operations: Array<() => Promise<unknown>>,
  ): Promise<unknown[]> {
    const result = await this.executeOperation(async () => {
      const multi = this.client.multi();

      operations.forEach((operation) => {
        operation();
      });

      return multi.exec();
    }, 'transaction');

    return result.success ? result.data || [] : [];
  }

  /**
   * Flush all data from the current database
   */
  async flushdb(): Promise<boolean> {
    const result = await this.executeOperation(
      () => this.client.flushDb() as Promise<string>,
      'flushdb',
    );

    return result.success && result.data === 'OK';
  }

  /**
   * Get Redis client statistics
   */
  async getStats(): Promise<{
    connected: boolean;
    totalCommands: number;
    errorRate: number;
    avgResponseTime: number;
  }> {
    return {
      connected: this.isReady(),
      totalCommands: 0, // Would track from metrics
      errorRate: 0, // Would calculate from metrics
      avgResponseTime: 0, // Would calculate from metrics
    };
  }

  /**
   * Health check for Redis connection
   */
  async healthCheck(): Promise<boolean> {
    try {
      if (!this.isReady()) {
        await this.connect();
      }

      const result = await this.client.ping();
      return result === 'PONG';
    } catch (error) {
      logError(toAppError(error), { operation: 'healthCheck' });
      return false;
    }
  }

  /**
   * Get Redis info
   */
  async info(): Promise<string> {
    const result = await this.executeOperation(
      () => this.client.info() as Promise<string>,
      'info',
    );

    return result.success ? result.data || '' : '';
  }
}

/**
 * Factory function to create Redis client instances
 */
export function createRedisClient(config?: Partial<RedisConfig>): RedisClient {
  return new RedisClient(config);
}

/**
 * Pre-configured Redis client instances for common use cases
 */
export const defaultRedisClient = createRedisClient();

export const cacheRedisClient = createRedisClient({
  cacheTtl: 7200, // 2 hours
  maxRetries: 5,
});

export const sessionRedisClient = createRedisClient({
  cacheTtl: 86400, // 24 hours
  db: 1,
});

// Export singleton instance for backward compatibility
export const redis = defaultRedisClient;

export default RedisClient;
