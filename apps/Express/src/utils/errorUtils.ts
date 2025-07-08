import logger from './logger';

export function handleServiceError(error: unknown, serviceName: string, operation: string): Error {
  let errorMessage: string;
  if (error instanceof Error) {
    errorMessage = error.message;
  } else {
    errorMessage = 'Unknown error';
  }
  logger.error(`Error in ${serviceName} - ${operation}: ${errorMessage}`, error);
  return new Error(`Failed to ${operation} in ${serviceName}: ${errorMessage}`);
}
