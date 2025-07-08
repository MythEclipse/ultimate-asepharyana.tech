import dotenv from 'dotenv';

dotenv.config();

export const config = {
  port: process.env.PORT || 4091,
  env: process.env.NODE_ENV || 'development',
  database: {
    connectionString: process.env.DATABASE_URL || 'sqlite:./database.sqlite',
  },
};
