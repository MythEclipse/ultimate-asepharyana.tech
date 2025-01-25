export interface ChatMessage {
  user: string;
  text: string;
  email?: string;
  imageProfile?: string;
  imageMessage?: string;
  role?: string;
}
