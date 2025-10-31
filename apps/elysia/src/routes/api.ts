import { Elysia } from 'elysia';

export const apiRoutes = new Elysia({ prefix: '/api' })
  .get('/users', () => [
    { id: 1, name: 'John Doe' },
    { id: 2, name: 'Jane Smith' },
  ])
  .get('/users/:id', ({ params: { id } }) => ({
    id: parseInt(id),
    name: 'User ' + id,
  }))
  .post('/users', ({ body }) => ({
    message: 'User created',
    data: body,
  }))
  .put('/users/:id', ({ params: { id }, body }) => ({
    message: 'User updated',
    id: parseInt(id),
    data: body,
  }))
  .delete('/users/:id', ({ params: { id } }) => ({
    message: 'User deleted',
    id: parseInt(id),
  }));
