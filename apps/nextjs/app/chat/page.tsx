'use client';
import dynamicImport from 'next/dynamic';
import { Loader2 } from 'lucide-react';

export const dynamic = 'force-dynamic';

const ChatClient = dynamicImport(() => import('../../components/chat/ChatClient'), {
  ssr: false,
  loading: () => (
    <div className="flex items-center justify-center h-screen">
      <Loader2 className="w-8 h-8 animate-spin mr-2" />
      <span className="text-lg">Loading chat...</span>
    </div>
  ),
});

export default function ChatPage() {
  return <ChatClient />;
}
