// models/chatModel.ts
export interface ChatMessage {
  id?: number; // ID yang akan dihasilkan oleh database
  user: string;
  text: string;
  email?: string;
  imageProfile?: string;
  imageMessage?: string;
  role: string;
  timestamp?: number; // Timestamp yang akan ditambahkan saat menyimpan pesan
}
