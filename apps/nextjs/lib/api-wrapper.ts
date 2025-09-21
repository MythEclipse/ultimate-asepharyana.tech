// apps/NextJS/lib/api-wrapper.ts
import { NextRequest, NextResponse } from 'next/server';
import logger from '../utils/unified-logger';
import { corsHeaders } from './corsHeaders';

type ApiHandler<T> = (
  req: NextRequest,
  props: { params: T },
) => Promise<NextResponse>;

function formatLogContext(context: Record<string, unknown>) {
  return Object.entries(context)

    .filter(([_, v]) => v !== undefined)
    .map(([k, v]) => `${k}=${v}`)
    .join(' | ');
}

export function withLogging<T>(handler: ApiHandler<T>) {
  return async (req: NextRequest, props: { params: T }) => {
    const ip =
      req.headers.get('x-forwarded-for') ||
      req.headers.get('remote-addr') ||
      'unknown';
    const url = req.url;
    const method = req.method;
    const requestId = req.headers.get('x-request-id') || undefined;
    const start = Date.now();

    try {
      const response = await handler(req, props);
      const duration = Date.now() - start;
      logger.info(
        `[Request processed] ${formatLogContext({
          ip,
          url,
          method,
          status: response.status,
          durationMs: duration,
          ...(requestId ? { requestId } : {}),
        })}`,
      );
      if (requestId) {
        response.headers.set('x-request-id', requestId);
      }
      return response;
    } catch (error: unknown) {
      const errorMessage =
        error instanceof Error ? error.message : 'Unknown error';
      const duration = Date.now() - start;
      logger.error(
        `[Error processing request] ${formatLogContext({
          ip,
          url,
          method,
          error: errorMessage,
          durationMs: duration,
          ...(requestId ? { requestId } : {}),
        })}`,
      );
      const response = NextResponse.json(
        {
          message: 'Failed to process request',
          error: errorMessage,
          ...(requestId ? { requestId } : {}),
        },
        { status: 500, headers: corsHeaders },
      );
      if (requestId) {
        response.headers.set('x-request-id', requestId);
      }
      return response;
    }
  };
}
