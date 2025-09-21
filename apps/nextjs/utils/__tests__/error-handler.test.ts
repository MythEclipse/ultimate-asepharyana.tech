import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
  createError,
  createNetworkError,
  createHttpError,
  toAppError,
  logError,
  withRetry,
  ErrorHandler,
} from '../error-handler';
import { ErrorCategory, ErrorSeverity } from '../../types/error';
import logger from '../logger';

// Mock the logger
vi.mock('../logger', () => ({
  default: {
    error: vi.fn(),
    warn: vi.fn(),
    info: vi.fn(),
    debug: vi.fn(),
  },
}));

describe('Error Handler', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('createError', () => {
    it('should create a basic error with required properties', () => {
      const error = createError('Test error', ErrorCategory.UNKNOWN);

      expect(error).toMatchObject({
        message: 'Test error',
        category: ErrorCategory.UNKNOWN,
        severity: ErrorSeverity.MEDIUM,
        statusCode: undefined,
        timestamp: expect.any(Date),
      });
      expect(error.name).toBe('Error');
      expect(error.stack).toBeDefined();
    });

    it('should create error with custom properties', () => {
      const error = createError('Test error', ErrorCategory.NETWORK, {
        severity: ErrorSeverity.HIGH,
        statusCode: 503,
        context: { url: 'https://example.com' },
      });

      expect(error).toMatchObject({
        message: 'Test error',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.HIGH,
        statusCode: 503,
        context: { url: 'https://example.com' },
      });
    });
  });

  describe('createNetworkError', () => {
    it('should create network error with proper defaults', () => {
      const error = createNetworkError('Network failed');

      expect(error).toMatchObject({
        message: 'Network failed',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.MEDIUM,
        code: undefined,
        url: undefined,
      });
    });

    it('should include original error context', () => {
      const originalError = new Error('Connection timeout');
      const error = createNetworkError('Network failed', {
        code: 'ETIMEDOUT',
        url: 'https://api.example.com',
        originalError,
        context: { timeout: 5000 },
      });

      expect(error.code).toBe('ETIMEDOUT');
      expect(error.url).toBe('https://api.example.com');
      expect(error.context).toMatchObject({
        code: 'ETIMEDOUT',
        url: 'https://api.example.com',
        timeout: 5000,
      });
    });
  });

  describe('createHttpError', () => {
    it('should create HTTP error with status code', () => {
      const error = createHttpError('Not found', 404);

      expect(error).toMatchObject({
        message: 'Not found',
        category: ErrorCategory.HTTP,
        statusCode: 404,
        statusText: undefined,
        url: undefined,
      });
    });

    it('should include status text and URL', () => {
      const error = createHttpError('Not found', 404, {
        statusText: 'Not Found',
        url: '/api/users/123',
      });

      expect(error).toMatchObject({
        statusCode: 404,
        statusText: 'Not Found',
        url: '/api/users/123',
      });
    });
  });

  describe('toAppError', () => {
    it('should convert Error to AppError', () => {
      const originalError = new Error('Test error');
      const appError = toAppError(originalError);

      expect(appError.message).toBe('Test error');
      expect(appError.category).toBe(ErrorCategory.UNKNOWN);
    });

    it('should convert string to AppError', () => {
      const appError = toAppError('String error');

      expect(appError.message).toBe('String error');
      expect(appError.category).toBe(ErrorCategory.UNKNOWN);
    });

    it('should preserve AppError as-is', () => {
      const originalError = createError('Original error', ErrorCategory.HTTP);
      const appError = toAppError(originalError);

      expect(appError).toBe(originalError);
    });

    it('should include context information', () => {
      const originalError = new Error('Test error');
      const appError = toAppError(originalError, {
        url: '/api/test',
        method: 'POST',
        userId: '123'
      });

      expect(appError.context).toMatchObject({
        originalError: originalError,
        url: '/api/test',
        method: 'POST',
        userId: '123',
      });
    });
  });

  describe('logError', () => {
    it('should log error with structured data', () => {
      const error = createError('Test error', ErrorCategory.HTTP);
      logError(error);

      expect(logger.error).toHaveBeenCalledWith('High severity error occurred', expect.objectContaining({
        message: 'Test error',
        category: 'HTTP',
        severity: 'HIGH',
        statusCode: undefined,
        timestamp: expect.any(String),
      }));
    });

    it('should include additional context', () => {
      const error = createError('Test error', ErrorCategory.NETWORK, {
        context: { url: 'https://api.example.com' },
      });
      logError(error, { userId: '123' });

      expect(logger.error).toHaveBeenCalledWith('Medium severity error occurred', expect.objectContaining({
        userId: '123',
        url: 'https://api.example.com',
      }));
    });
  });

  describe('withRetry', () => {
    it('should succeed on first attempt', async () => {
      const operation = vi.fn().mockResolvedValue('success');
      const result = await withRetry(operation, {
        enabled: true,
        maxAttempts: 3,
        delayMs: 100,
        backoffMultiplier: 2,
        maxDelayMs: 5000,
      });

      expect(result).toBe('success');
      expect(operation).toHaveBeenCalledTimes(1);
    });

    it('should retry on failure and succeed', async () => {
      const operation = vi.fn()
        .mockRejectedValueOnce(new Error('First fail'))
        .mockResolvedValueOnce('success');

      const result = await withRetry(operation, {
        enabled: true,
        maxAttempts: 3,
        delayMs: 10,
        backoffMultiplier: 2,
        maxDelayMs: 5000,
      });

      expect(result).toBe('success');
      expect(operation).toHaveBeenCalledTimes(2);
    });

    it('should throw after max attempts', async () => {
      const operation = vi.fn().mockRejectedValue(new Error('Persistent failure'));

      await expect(withRetry(operation, {
        enabled: true,
        maxAttempts: 3,
        delayMs: 10,
        backoffMultiplier: 2,
        maxDelayMs: 5000,
      })).rejects.toThrow('Persistent failure');
      expect(operation).toHaveBeenCalledTimes(3);
    });

    it('should not retry when disabled', async () => {
      const operation = vi.fn().mockRejectedValue(new Error('Should not retry'));

      await expect(withRetry(operation, {
        enabled: false,
        maxAttempts: 3,
        delayMs: 10,
        backoffMultiplier: 2,
        maxDelayMs: 5000,
      })).rejects.toThrow('Should not retry');
      expect(operation).toHaveBeenCalledTimes(1);
    });

    it('should use custom retry delay', async () => {
      const operation = vi.fn()
        .mockRejectedValueOnce(new Error('First fail'))
        .mockResolvedValueOnce('success');

      const start = Date.now();
      await withRetry(operation, {
        enabled: true,
        maxAttempts: 3,
        delayMs: 100,
        backoffMultiplier: 2,
        maxDelayMs: 5000,
      });
      const duration = Date.now() - start;

      expect(duration).toBeGreaterThanOrEqual(100);
      expect(operation).toHaveBeenCalledTimes(2);
    });
  });

  describe('ErrorHandler utility object', () => {
    it('should export all error creation functions', () => {
      expect(ErrorHandler.createError).toBe(createError);
      expect(ErrorHandler.createNetworkError).toBe(createNetworkError);
      expect(ErrorHandler.createHttpError).toBe(createHttpError);
      expect(ErrorHandler.toAppError).toBe(toAppError);
      expect(ErrorHandler.logError).toBe(logError);
      expect(ErrorHandler.withRetry).toBe(withRetry);
    });
  });
});
