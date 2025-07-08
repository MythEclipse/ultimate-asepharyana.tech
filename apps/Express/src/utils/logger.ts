import pino from 'pino';
import { config } from '../config/config';

const logger = pino({
  transport: {
    target: 'pino-pretty',
    options: {
      colorize: true,
    },
  },
  level: config.env === 'development' ? 'debug' : 'info',
});

export default logger;
