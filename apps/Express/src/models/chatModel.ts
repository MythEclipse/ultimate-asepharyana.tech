export interface ChatMessage {
  id: string;
  userId?: string;
  text: string;
  timestamp?: Date;
  user?: string;
}
