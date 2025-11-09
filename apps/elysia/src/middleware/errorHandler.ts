import { Elysia } from 'elysia';
import { errorResponse } from '../utils/response';

/**
 * Global error handler middleware
 */
export const errorHandler = new Elysia()
  .onError(({ code, error, set }) => {
    const errorMessage = error instanceof Error ? error.message : String(error);
    const errorStack = error instanceof Error ? error.stack : undefined;

    console.error('Error:', {
      code,
      message: errorMessage,
      stack: process.env.NODE_ENV === 'development' ? errorStack : undefined,
    });

    // Handle different error types
    switch (code) {
      case 'VALIDATION':
        set.status = 400;
        return errorResponse(
          'VALIDATION_ERROR',
          'Invalid request data',
          process.env.NODE_ENV === 'development'
            ? { details: errorMessage }
            : undefined
        );

      case 'NOT_FOUND':
        set.status = 404;
        return errorResponse('NOT_FOUND', 'Resource not found');

      case 'PARSE':
        set.status = 400;
        return errorResponse('PARSE_ERROR', 'Invalid request format');

      case 'INTERNAL_SERVER_ERROR':
        set.status = 500;
        return errorResponse(
          'INTERNAL_ERROR',
          'An unexpected error occurred',
          process.env.NODE_ENV === 'development'
            ? { message: errorMessage }
            : undefined
        );

      case 'UNKNOWN':
      default:
        // Handle custom errors
        if (errorMessage.startsWith('Unauthorized')) {
          set.status = 401;
          return errorResponse('UNAUTHORIZED', errorMessage);
        }

        if (errorMessage.startsWith('Forbidden')) {
          set.status = 403;
          return errorResponse('FORBIDDEN', errorMessage);
        }

        if (errorMessage.includes('not found')) {
          set.status = 404;
          return errorResponse('NOT_FOUND', errorMessage);
        }

        if (errorMessage.includes('already exists')) {
          set.status = 409;
          return errorResponse('CONFLICT', errorMessage);
        }

        if (errorMessage.startsWith('Invalid')) {
          set.status = 400;
          return errorResponse('BAD_REQUEST', errorMessage);
        }

        // Default error response
        set.status = 500;
        return errorResponse(
          'INTERNAL_ERROR',
          process.env.NODE_ENV === 'production'
            ? 'An unexpected error occurred'
            : errorMessage
        );
    }
  });
