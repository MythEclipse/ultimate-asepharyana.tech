import { Request, Response, NextFunction } from 'express';
import logger from './logger';

// Middleware function to log IP and request details
export const requestLogger = (
  req: Request,
  res: Response,
  next: NextFunction
) => {
  const ip = req.ip || req.socket.remoteAddress || 'unknown';

  logger.info(
    {
      ip,
      method: req.method,
      path: req.path,
      timestamp: new Date().toISOString(),
    },
    `Incoming request from IP: ${ip}`
  );

  next();
};
