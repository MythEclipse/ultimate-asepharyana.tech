/**
 * Centralized error handling utilities for consistent error management across the application
 */

import logger from './logger';
import {
  AppError,
  NetworkError,
  HttpError,
  TimeoutError,
  ValidationError,
  AuthenticationError,
  AuthorizationError,
  ServerError,
  ErrorCategory,
  ErrorSeverity,
  ErrorHandlerConfig,
  ErrorResponse,
  RetryConfig,
  isAppError,
  HTTP_STATUS_TO_CATEGORY,
  HTTP_STATUS_TO_SEVERITY,
  RETRYABLE_NETWORK_CODES,
  DEFAULT_ERROR_MESSAGES,
} from '../types/error';

/**
 * Creates a standardized error object with consistent properties
 */
export function createError(
  message: string,
  category: ErrorCategory = ErrorCategory.UNKNOWN,
  options: {
    originalError?: unknown;
    statusCode?: number;
    context?: Record<string, unknown>;
    severity?: ErrorSeverity;
    userMessage?: string;
    retryable?: boolean;
  } = {}
): AppError {
  const {
    originalError,
    statusCode,
    context,
    severity,
    userMessage,
    retryable,
  } = options;

  const error = new Error(message) as AppError;
  error.category = category;
  error.severity = severity || getDefaultSeverity(category, statusCode);
  error.statusCode = statusCode;
  error.originalError = originalError;
  error.context = context || {};
  error.timestamp = new Date();
  error.userMessage = userMessage || getDefaultUserMessage(category, statusCode);
  error.retryable = retryable ?? isRetryableByDefault(category, statusCode);

  // Preserve stack trace from original error if available
  if (originalError instanceof Error && originalError.stack) {
    error.stack = originalError.stack;
  }

  return error;
}

/**
 * Creates a network error with specific properties
 */
export function createNetworkError(
  message: string,
  options: {
    code?: string;
    url?: string;
    originalError?: unknown;
    context?: Record<string, unknown>;
  } = {}
): NetworkError {
  const { code, url, originalError, context } = options;

  const error = createError(message, ErrorCategory.NETWORK, {
    originalError,
    context: { ...context, code, url },
    retryable: code ? RETRYABLE_NETWORK_CODES.has(code) : true,
  }) as NetworkError;

  error.code = code;
  error.url = url;

  return error;
}

/**
 * Creates an HTTP error with specific properties
 */
export function createHttpError(
  message: string,
  statusCode: number,
  options: {
    statusText?: string;
    url?: string;
    response?: Response;
    originalError?: unknown;
    context?: Record<string, unknown>;
  } = {}
): HttpError {
  const { statusText, url, response, originalError, context } = options;

  const category = HTTP_STATUS_TO_CATEGORY[statusCode] || ErrorCategory.HTTP;
  const error = createError(message, category, {
    statusCode,
    originalError,
    context: { ...context, statusText, url, response },
  }) as HttpError;

  error.statusText = statusText;
  error.url = url;
  error.response = response;

  return error;
}

/**
 * Creates a timeout error with specific properties
 */
export function createTimeoutError(
  message: string,
  timeoutMs: number,
  options: {
    url?: string;
    originalError?: unknown;
    context?: Record<string, unknown>;
  } = {}
): TimeoutError {
  const { url, originalError, context } = options;

  const error = createError(message, ErrorCategory.TIMEOUT, {
    originalError,
    context: { ...context, timeoutMs, url },
    retryable: true,
  }) as TimeoutError;

  error.timeoutMs = timeoutMs;
  error.url = url;

  return error;
}

/**
 * Creates a validation error with specific properties
 */
export function createValidationError(
  message: string,
  options: {
    field?: string;
    value?: unknown;
    originalError?: unknown;
    context?: Record<string, unknown>;
  } = {}
): ValidationError {
  const { field, value, originalError, context } = options;

  const error = createError(message, ErrorCategory.VALIDATION, {
    originalError,
    context: { ...context, field, value },
    retryable: false,
  }) as ValidationError;

  error.field = field;
  error.value = value;

  return error;
}

/**
 * Creates an authentication error with specific properties
 */
export function createAuthenticationError(
  message = 'Authentication required',
  options: {
    originalError?: unknown;
    context?: Record<string, unknown>;
  } = {}
): AuthenticationError {
  const { originalError, context } = options;
  return createError(message, ErrorCategory.AUTHENTICATION, {
    originalError,
    context,
    statusCode: 401,
    retryable: false,
  }) as AuthenticationError;
}

/**
 * Creates an authorization error with specific properties
 */
export function createAuthorizationError(
  message = 'You do not have permission to perform this action',
  options: {
    requiredRole?: string;
    originalError?: unknown;
    context?: Record<string, unknown>;
  } = {}
): AuthorizationError {
  const { requiredRole, originalError, context } = options;

  const error = createError(message, ErrorCategory.AUTHORIZATION, {
    originalError,
    context: { ...context, requiredRole },
    statusCode: 403,
    retryable: false,
  }) as AuthorizationError;

  error.requiredRole = requiredRole;

  return error;
}

/**
 * Creates a server error with specific properties
 */
export function createServerError(
  message: string,
  options: {
    serverMessage?: string;
    statusCode?: number;
    originalError?: unknown;
    context?: Record<string, unknown>;
  } = {}
): ServerError {
  const { serverMessage, statusCode = 500, originalError, context } = options;

  const error = createError(message, ErrorCategory.SERVER, {
    statusCode,
    originalError,
    context: { ...context, serverMessage },
  }) as ServerError;

  error.serverMessage = serverMessage;

  return error;
}

/**
 * Determines if an error is retryable by default based on category and status code
 */
function isRetryableByDefault(category: ErrorCategory, statusCode?: number): boolean {
  switch (category) {
    case ErrorCategory.NETWORK:
    case ErrorCategory.TIMEOUT:
      return true;
    case ErrorCategory.HTTP:
      return statusCode ? statusCode >= 500 : false;
    case ErrorCategory.VALIDATION:
    case ErrorCategory.AUTHENTICATION:
    case ErrorCategory.AUTHORIZATION:
      return false;
    default:
      return false;
  }
}

/**
 * Gets the default severity for an error based on category and status code
 */
function getDefaultSeverity(category: ErrorCategory, statusCode?: number): ErrorSeverity {
  if (statusCode && HTTP_STATUS_TO_SEVERITY[statusCode]) {
    return HTTP_STATUS_TO_SEVERITY[statusCode];
  }

  switch (category) {
    case ErrorCategory.NETWORK:
    case ErrorCategory.TIMEOUT:
      return ErrorSeverity.MEDIUM;
    case ErrorCategory.HTTP:
      return ErrorSeverity.HIGH;
    case ErrorCategory.VALIDATION:
      return ErrorSeverity.LOW;
    case ErrorCategory.AUTHENTICATION:
    case ErrorCategory.AUTHORIZATION:
      return ErrorSeverity.MEDIUM;
    case ErrorCategory.SERVER:
      return ErrorSeverity.HIGH;
    default:
      return ErrorSeverity.MEDIUM;
  }
}

/**
 * Gets the default user-friendly message for an error
 */
function getDefaultUserMessage(category: ErrorCategory, statusCode?: number): string {
  if (statusCode && DEFAULT_ERROR_MESSAGES[category]) {
    return DEFAULT_ERROR_MESSAGES[category];
  }
  return DEFAULT_ERROR_MESSAGES[category] || DEFAULT_ERROR_MESSAGES[ErrorCategory.UNKNOWN];
}

/**
 * Logs an error using the unified logger with appropriate context
 */
export function logError(error: AppError, context?: Record<string, unknown>): void {
  const logData = {
    message: error.message,
    category: error.category,
    severity: error.severity,
    statusCode: error.statusCode,
    userMessage: error.userMessage,
    retryable: error.retryable,
    timestamp: error.timestamp.toISOString(),
    context: { ...error.context, ...context },
  };

  // Add stack trace if available and not in production
  if (error.stack && process.env.NODE_ENV !== 'production') {
    logData.context = { ...logData.context, stack: error.stack };
  }

  switch (error.severity) {
    case ErrorSeverity.CRITICAL:
      logger.error('Critical error occurred', logData);
      break;
    case ErrorSeverity.HIGH:
      logger.error('High severity error occurred', logData);
      break;
    case ErrorSeverity.MEDIUM:
      logger.warn('Medium severity error occurred', logData);
      break;
    case ErrorSeverity.LOW:
      logger.info('Low severity error occurred', logData);
      break;
    default:
      logger.error('Error occurred', logData);
  }
}

/**
 * Converts an unknown error to an AppError
 */
export function toAppError(error: unknown, context?: Record<string, unknown>): AppError {
  if (isAppError(error)) {
    return error;
  }

  if (error instanceof Error) {
    // Try to infer category from error name or message
    let category = ErrorCategory.UNKNOWN;
    let statusCode: number | undefined;

    if (error.name === 'AbortError' || error.message.includes('timeout')) {
      category = ErrorCategory.TIMEOUT;
    } else if (error.message.includes('network') || error.message.includes('fetch')) {
      category = ErrorCategory.NETWORK;
    } else if (error.message.includes('validation') || error.message.includes('invalid')) {
      category = ErrorCategory.VALIDATION;
    } else if (error.message.includes('auth')) {
      category = ErrorCategory.AUTHENTICATION;
    }

    return createError(error.message, category, {
      originalError: error,
      context,
    });
  }

  // Handle non-Error objects
  const message = typeof error === 'string' ? error : 'An unknown error occurred';
  return createError(message, ErrorCategory.UNKNOWN, {
    originalError: error,
    context,
  });
}

/**
 * Creates an error response object for API responses
 */
export function createErrorResponse(
  error: AppError,
  requestId?: string
): ErrorResponse {
  return {
    error: {
      message: error.userMessage || error.message,
      category: error.category,
      statusCode: error.statusCode,
      details: error.context,
      timestamp: error.timestamp.toISOString(),
      requestId,
    },
  };
}

/**
 * Implements retry logic with exponential backoff
 */
export async function withRetry<T>(
  operation: () => Promise<T>,
  config: RetryConfig,
  onRetry?: (attempt: number, error: AppError) => void
): Promise<T> {
  const { enabled, maxAttempts, delayMs, backoffMultiplier, maxDelayMs } = config;

  if (!enabled) {
    return operation();
  }

  let lastError: AppError;

  for (let attempt = 1; attempt <= maxAttempts; attempt++) {
    try {
      return await operation();
    } catch (error) {
      lastError = toAppError(error);

      // Don't retry non-retryable errors
      if (!lastError.retryable) {
        throw lastError;
      }

      // Don't retry on the last attempt
      if (attempt === maxAttempts) {
        throw lastError;
      }

      // Calculate delay with exponential backoff
      const delay = Math.min(
        delayMs * Math.pow(backoffMultiplier, attempt - 1),
        maxDelayMs
      );

      if (onRetry) {
        onRetry(attempt, lastError);
      }

      // Wait before retrying
      await new Promise(resolve => setTimeout(resolve, delay));
    }
  }

  throw lastError!;
}

/**
 * Wraps a function with error handling and optional retry logic
 */
export function withErrorHandling<TArgs extends unknown[], TReturn>(
  fn: (...args: TArgs) => TReturn,
  config: ErrorHandlerConfig = {}
) {
  return async (...args: TArgs): Promise<TReturn> => {
    const {
      fallback,
      onError,
      onRetry,
      retry,
      includeStackTrace = true,
      includeContext = true,
    } = config;

    try {
      const operation = async () => fn(...args);

      if (retry?.enabled) {
        return await withRetry(operation, retry, onRetry);
      }

      return await operation();
    } catch (error) {
      const appError = toAppError(error, { function: fn.name, args });

      // Log the error
      logError(appError);

      // Call custom error handler
      if (onError) {
        onError(appError);
      }

      // Return fallback if provided
      if (fallback) {
        return fallback() as TReturn;
      }

      // Re-throw the error
      throw appError;
    }
  };
}

/**
 * Handles fetch response errors and converts them to AppError instances
 */
export async function handleFetchResponse(
  response: Response,
  url: string,
  context?: Record<string, unknown>
): Promise<void> {
  if (!response.ok) {
    let errorMessage: string;

    try {
      const errorData = await response.json().catch(() => ({ message: response.statusText }));
      errorMessage = errorData.message || response.statusText || 'Request failed';
    } catch {
      errorMessage = response.statusText || 'Request failed';
    }

    throw createHttpError(errorMessage, response.status, {
      statusText: response.statusText,
      url,
      response,
      context,
    });
  }
}

/**
 * Handles network errors and converts them to NetworkError instances
 */
export function handleNetworkError(
  error: unknown,
  url: string,
  context?: Record<string, unknown>
): NetworkError {
  const originalError = error as Error & { code?: string };
  const message = originalError.message || 'Network error occurred';
  const code = originalError.code;

  return createNetworkError(message, {
    code,
    url,
    originalError: error,
    context,
  });
}

/**
 * Handles timeout errors and converts them to TimeoutError instances
 */
export function handleTimeoutError(
  timeoutMs: number,
  url: string,
  context?: Record<string, unknown>
): TimeoutError {
  return createTimeoutError(`Request timed out after ${timeoutMs}ms`, timeoutMs, {
    url,
    context,
  });
}

// Export all error creation functions and utilities
export const ErrorHandler = {
  createError,
  createNetworkError,
  createHttpError,
  createTimeoutError,
  createValidationError,
  createAuthenticationError,
  createAuthorizationError,
  createServerError,
  toAppError,
  logError,
  createErrorResponse,
  withRetry,
  withErrorHandling,
  handleFetchResponse,
  handleNetworkError,
  handleTimeoutError,
};
