/**
 * Centralized error types for consistent error handling across the application
 */

export enum ErrorCategory {
  NETWORK = 'NETWORK',
  HTTP = 'HTTP',
  TIMEOUT = 'TIMEOUT',
  VALIDATION = 'VALIDATION',
  AUTHENTICATION = 'AUTHENTICATION',
  AUTHORIZATION = 'AUTHORIZATION',
  SERVER = 'SERVER',
  UNKNOWN = 'UNKNOWN',
}

export enum ErrorSeverity {
  LOW = 'LOW',
  MEDIUM = 'MEDIUM',
  HIGH = 'HIGH',
  CRITICAL = 'CRITICAL',
}

export interface AppError extends Error {
  category: ErrorCategory;
  severity: ErrorSeverity;
  statusCode?: number;
  originalError?: unknown;
  context?: Record<string, unknown>;
  timestamp: Date;
  userMessage?: string;
  retryable: boolean;
}

export interface NetworkError extends AppError {
  category: ErrorCategory.NETWORK;
  code?: string;
  url?: string;
}

export interface HttpError extends AppError {
  category: ErrorCategory.HTTP;
  statusCode: number;
  statusText?: string;
  url?: string;
  response?: Response;
}

export interface TimeoutError extends AppError {
  category: ErrorCategory.TIMEOUT;
  timeoutMs: number;
  url?: string;
}

export interface ValidationError extends AppError {
  category: ErrorCategory.VALIDATION;
  field?: string;
  value?: unknown;
}

export interface AuthenticationError extends AppError {
  category: ErrorCategory.AUTHENTICATION;
}

export interface AuthorizationError extends AppError {
  category: ErrorCategory.AUTHORIZATION;
  requiredRole?: string;
}

export interface ServerError extends AppError {
  category: ErrorCategory.SERVER;
  serverMessage?: string;
}

export interface ErrorResponse {
  error: {
    message: string;
    category: ErrorCategory;
    statusCode?: number;
    details?: Record<string, unknown>;
    timestamp: string;
    requestId?: string;
  };
}

export interface ErrorConfig {
  defaultUserMessage?: string;
  logLevel?: 'error' | 'warn' | 'info' | 'debug';
  includeStackTrace?: boolean;
  includeContext?: boolean;
  retryableByDefault?: boolean;
}

export interface RetryConfig {
  enabled: boolean;
  maxAttempts: number;
  delayMs: number;
  backoffMultiplier: number;
  maxDelayMs: number;
}

export interface ErrorHandlerConfig extends ErrorConfig {
  retry?: RetryConfig;
  fallback?: () => unknown;
  onError?: (error: AppError) => void;
  onRetry?: (attempt: number, error: AppError) => void;
}

// Type guards
export const isAppError = (error: unknown): error is AppError => {
  return error instanceof Error && 'category' in error && 'severity' in error;
};

export const isNetworkError = (error: unknown): error is NetworkError => {
  return isAppError(error) && error.category === ErrorCategory.NETWORK;
};

export const isHttpError = (error: unknown): error is HttpError => {
  return isAppError(error) && error.category === ErrorCategory.HTTP;
};

export const isTimeoutError = (error: unknown): error is TimeoutError => {
  return isAppError(error) && error.category === ErrorCategory.TIMEOUT;
};

export const isValidationError = (error: unknown): error is ValidationError => {
  return isAppError(error) && error.category === ErrorCategory.VALIDATION;
};

export const isAuthenticationError = (error: unknown): error is AuthenticationError => {
  return isAppError(error) && error.category === ErrorCategory.AUTHENTICATION;
};

export const isAuthorizationError = (error: unknown): error is AuthorizationError => {
  return isAppError(error) && error.category === ErrorCategory.AUTHORIZATION;
};

export const isServerError = (error: unknown): error is ServerError => {
  return isAppError(error) && error.category === ErrorCategory.SERVER;
};

// HTTP Status Code to Error Category mapping
export const HTTP_STATUS_TO_CATEGORY: Record<number, ErrorCategory> = {
  400: ErrorCategory.VALIDATION,
  401: ErrorCategory.AUTHENTICATION,
  403: ErrorCategory.AUTHORIZATION,
  404: ErrorCategory.NETWORK,
  408: ErrorCategory.TIMEOUT,
  429: ErrorCategory.SERVER,
  500: ErrorCategory.SERVER,
  502: ErrorCategory.SERVER,
  503: ErrorCategory.SERVER,
  504: ErrorCategory.TIMEOUT,
};

// Error severity mapping based on status codes
export const HTTP_STATUS_TO_SEVERITY: Record<number, ErrorSeverity> = {
  400: ErrorSeverity.LOW,
  401: ErrorSeverity.MEDIUM,
  403: ErrorSeverity.MEDIUM,
  404: ErrorSeverity.LOW,
  408: ErrorSeverity.MEDIUM,
  429: ErrorSeverity.MEDIUM,
  500: ErrorSeverity.HIGH,
  502: ErrorSeverity.HIGH,
  503: ErrorSeverity.HIGH,
  504: ErrorSeverity.MEDIUM,
};

// Network error codes that indicate retryability
export const RETRYABLE_NETWORK_CODES = new Set([
  'ECONNRESET',
  'ECONNREFUSED',
  'ENOTFOUND',
  'ETIMEDOUT',
  'ESOCKETTIMEDOUT',
  'EHOSTUNREACH',
  'EPIPE',
]);

// Default error messages for different categories
export const DEFAULT_ERROR_MESSAGES: Record<ErrorCategory, string> = {
  [ErrorCategory.NETWORK]: 'Network error occurred. Please check your connection and try again.',
  [ErrorCategory.HTTP]: 'Request failed. Please try again later.',
  [ErrorCategory.TIMEOUT]: 'Request timed out. Please try again.',
  [ErrorCategory.VALIDATION]: 'Invalid input. Please check your data and try again.',
  [ErrorCategory.AUTHENTICATION]: 'Authentication required. Please log in and try again.',
  [ErrorCategory.AUTHORIZATION]: 'You do not have permission to perform this action.',
  [ErrorCategory.SERVER]: 'Server error occurred. Please try again later.',
  [ErrorCategory.UNKNOWN]: 'An unexpected error occurred. Please try again.',
};
