/**
 * Image Cache API Route for Elysia
 * Fallback endpoint for when Rust API is unavailable
 */

import Elysia, { t } from 'elysia';
import { getOrCacheImage } from '../services';

export const imageCacheRoutes = new Elysia({ prefix: '/api' })
  .post(
    '/image-cache',
    async ({ body }) => {
      const cdnUrl = await getOrCacheImage(body.url, undefined, undefined, {
        lazy: body.lazy ?? false,
      });
      const fromCache = cdnUrl !== body.url;

      return {
        success: true,
        original_url: body.url,
        cdn_url: cdnUrl,
        from_cache: fromCache,
        pending: body.lazy && !fromCache, // Indicates background upload in progress
      };
    },
    {
      body: t.Object({
        url: t.String(),
        lazy: t.Optional(t.Boolean()),
      }),
      detail: {
        tags: ['API'],
        summary: 'Cache an image to CDN',
        description:
          'Upload an image to Picser CDN. Set lazy=true to return immediately while caching in background.',
      },
    },
  )
  .post(
    '/image-cache/batch',
    async ({ body }) => {
      const results = await Promise.all(
        body.urls.map(async (url) => {
          const cdnUrl = await getOrCacheImage(url);
          return {
            original_url: url,
            cdn_url: cdnUrl,
            from_cache: cdnUrl !== url,
          };
        }),
      );

      return {
        success: true,
        results,
      };
    },
    {
      body: t.Object({
        urls: t.Array(t.String()),
      }),
      detail: {
        tags: ['API'],
        summary: 'Batch cache multiple images',
        description: 'Cache multiple images to Picser CDN at once',
      },
    },
  );
