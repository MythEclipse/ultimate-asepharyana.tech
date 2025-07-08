import { Application } from 'express';

export function setChatRoutes(app: Application) {
  app.get('/', (_, res) => {
    res.redirect('https://asepharyana.cloud/chat');
  });
}
