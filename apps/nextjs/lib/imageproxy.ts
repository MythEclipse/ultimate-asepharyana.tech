import logger from '../utils/unified-logger';
import { BaseUrl } from '../utils/url-utils';
import { NextResponse } from 'next/server';
import { ImageProcessingOptions } from '../types/image';
import {
  processImageWithFallback,
  createImageResponse
} from '../utils/image-proxy';

export const revalidate = 0;

/**
 * Image proxy handler with fallback chain
 * Uses centralized image processing system for consistency and maintainability
 */
export async function imageProxy(url: string) {
  try {
    // Validate input URL
    if (!url || typeof url !== 'string') {
      logger.error('Invalid URL parameter provided to imageProxy');
      return NextResponse.json(
        { error: 'Invalid URL parameter' },
        { status: 400 }
      );
    }

    // Process image with fallback chain using centralized system
    const processingOptions: ImageProcessingOptions = {
      validateContent: true,
      timeout: 30000, // 30 second timeout for proxy operations
    };

    const result = await processImageWithFallback(url, {}, processingOptions);

    if (!result.success) {
      logger.error(`Failed to process image from URL: ${url}, Error: ${result.error}`);
      return NextResponse.json(
        { error: result.error || 'Failed to process image' },
        { status: 400 }
      );
    }

    // Return successful image response
    return createImageResponse(result);
  } catch (error) {
    logger.error(`Image proxy internal error: ${(error as Error).message}`);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}

// Re-export for backward compatibility
export { processImageWithFallback, createImageResponse } from '../utils/image-proxy';
