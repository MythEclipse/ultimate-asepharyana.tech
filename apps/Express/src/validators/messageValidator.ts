// validators/messageValidator.ts
import { z } from 'zod';

export const MessageValidator = z.object({
  text: z.string()
    .min(1, 'Message cannot be empty')
    .max(5000, 'Message too long (max 5000 characters)'),
  user: z.string().min(1, 'Username is required').max(50),
  userId: z.string().min(1, 'User ID is required')
});

export type MessageSchema = z.infer<typeof MessageValidator>;