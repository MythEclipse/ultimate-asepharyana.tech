// models/chatModel.ts
export interface ChatMessage {
  id?: string; // ID yang akan dihasilkan oleh database
  userId: string;
  text: string;
  email?: string;
  imageProfile?: string;
  imageMessage?: string;
  role?: string;
  timestamp?: Date; // Timestamp yang akan ditambahkan saat menyimpan pesan
}
