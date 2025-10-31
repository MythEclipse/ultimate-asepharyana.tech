import { describe, expect, it } from 'bun:test';
import { app } from './index';

describe('Elysia Server', () => {
  it('should return welcome message', async () => {
    const response = await app.handle(new Request('http://localhost/')).then(res => res.text());
    expect(response).toBe('Hello from ElysiaJS!');
  });

  it('should return health status', async () => {
    const response = await app.handle(new Request('http://localhost/health')).then(res => res.json());
    expect(response).toHaveProperty('status', 'ok');
    expect(response).toHaveProperty('timestamp');
  });

  it('should return personalized greeting', async () => {
    const response = await app.handle(new Request('http://localhost/api/hello/World')).then(res => res.json());
    expect(response).toEqual({ message: 'Hello World!' });
  });

  it('should echo request body', async () => {
    const testBody = { test: 'data' };
    const response = await app.handle(
      new Request('http://localhost/api/echo', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(testBody),
      })
    ).then(res => res.json());
    expect(response).toEqual({ echo: testBody });
  });
});
