// apps/NextJS/lib/api-wrapper.ts
import { NextRequest, NextResponse } from 'next/server';
import logger from './logger';
import { corsHeaders } from './corsHeaders';

type ApiHandler<T> = (
  req: NextRequest,
  props: { params: T }
) => Promise<NextResponse>;

export function withLogging<T>(handler: ApiHandler<T>) {
  return async (req: NextRequest, props: { params: T }) => {
    const ip =
      req.headers.get('x-forwarded-for') ||
      req.headers.get('remote-addr') ||
      'unknown';
    const url = req.url;

    try {
      const response = await handler(req, props);
      logger.info('Request processed', {
        ip,
        url,
        status: response.status,
      });
      return response;
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      logger.error('Error processing request', {
        ip,
        url,
        error: errorMessage,
      });
      const response = NextResponse.json(
        { message: 'Failed to process request' },
        { status: 500, headers: corsHeaders }
      );
      return response;
    }
  };
}