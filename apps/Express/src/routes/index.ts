import { Application } from 'express';
import { setChatRoutes } from './chatRoutes';
import { setPdfRoutes } from './pdfRoutes';

export function setupRoutes(app: Application) {
  setChatRoutes(app);
  setPdfRoutes(app);
  // Add other route modules here as they are created
}
