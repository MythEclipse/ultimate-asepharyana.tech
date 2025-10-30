/**
 * Utility functions for client-side components
 */

/**
 * Convert error to string for display
 * Handles Error objects, strings, and other types
 */
export function getErrorMessage(error: unknown): string {
  if (!error) return '';

  if (typeof error === 'string') {
    return error;
  }

  if (error instanceof Error) {
    return error.message;
  }

  // For other types, try to stringify
  try {
    return JSON.stringify(error);
  } catch {
    return 'An unknown error occurred';
  }
}
