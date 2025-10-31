import { Elysia } from 'elysia';

export const logger = new Elysia()
  .onBeforeHandle(({ request }) => {
    const timestamp = new Date().toISOString();
    const method = request.method;
    const url = new URL(request.url);
    console.log(`[${timestamp}] ${method} ${url.pathname}`);
  })
  .onAfterHandle(({ request }) => {
    const timestamp = new Date().toISOString();
    const method = request.method;
    const url = new URL(request.url);
    console.log(`[${timestamp}] ${method} ${url.pathname} - Completed`);
  });
