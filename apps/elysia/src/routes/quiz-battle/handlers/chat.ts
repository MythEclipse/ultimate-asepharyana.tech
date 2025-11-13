// Chat Handler - In-Memory MVP
import type {
  WSMessage,
  ChatGlobalSendPayload,
  ChatPrivateSendPayload,
  ChatHistorySyncPayload,
  ChatTypingPayload,
  ChatGlobalMessagePayload,
  ChatPrivateMessagePayload,
} from '../types';

import { wsManager } from '../ws-manager';
import { getDb, users, eq } from '@asepharyana/services';

const globalMessages: ChatGlobalMessagePayload[] = [];
const privateMessages = new Map<string, ChatPrivateMessagePayload[]>();
const MAX_HISTORY = 100;

function getConversationId(userId1: string, userId2: string): string {
  return [userId1, userId2].sort().join('_');
}

export async function handleChatGlobalSend(sessionId: string, data: WSMessage<ChatGlobalSendPayload>) {
  const { userId, message } = data.payload;
  if (!message || message.trim().length === 0 || message.length > 500) {
    wsManager.sendToSession(sessionId, 'error', { message: 'Invalid message', code: 'INVALID_MESSAGE' });
    return;
  }
  const db = getDb();
  const [user] = await db.select().from(users).where(eq(users.id, userId)).limit(1);
  if (!user) return;
  const chatMessage: ChatGlobalMessagePayload = {
    messageId: `msg_${Date.now()}_${userId}`,
    sender: {
      userId,
      username: user.name || 'Anonymous',
      level: 1,
      avatarUrl: user.image || undefined,
    },
    message: message.trim(),
    timestamp: Date.now(),
  };
  globalMessages.push(chatMessage);
  if (globalMessages.length > MAX_HISTORY) globalMessages.shift();
  const connections = wsManager.getAllConnections();
  connections.forEach((conn) => {
    wsManager.sendToSession(conn.sessionId, 'chat:global:message', chatMessage);
  });
}

export async function handleChatPrivateSend(sessionId: string, data: WSMessage<ChatPrivateSendPayload>) {
  const { senderId, receiverId, message } = data.payload;
  if (!message || message.trim().length === 0 || message.length > 1000) {
    wsManager.sendToSession(sessionId, 'error', { message: 'Invalid message', code: 'INVALID_MESSAGE' });
    return;
  }
  const db = getDb();
  const [sender] = await db.select().from(users).where(eq(users.id, senderId)).limit(1);
  const [receiver] = await db.select().from(users).where(eq(users.id, receiverId)).limit(1);
  if (!sender || !receiver) return;
  const conversationId = getConversationId(senderId, receiverId);
  if (!privateMessages.has(conversationId)) {
    privateMessages.set(conversationId, []);
  }
  const conversation = privateMessages.get(conversationId);
  if (!conversation) return;
  const chatMessage: ChatPrivateMessagePayload = {
    messageId: `pm_${Date.now()}_${senderId}`,
    conversationId,
    sender: {
      userId: senderId,
      username: sender.name || 'Anonymous',
      status: 'online' as const,
    },
    message: message.trim(),
    timestamp: Date.now(),
    isRead: false,
  };
  conversation.push(chatMessage);
  if (conversation.length > MAX_HISTORY) conversation.shift();
  const senderConn = wsManager.getConnectionByUserId(senderId);
  const receiverConn = wsManager.getConnectionByUserId(receiverId);
  const senderSessionId = senderConn ? wsManager.getSessionIdByConnection(senderConn) : undefined;
  const receiverSessionId = receiverConn ? wsManager.getSessionIdByConnection(receiverConn) : undefined;
  if (senderSessionId) wsManager.sendToSession(senderSessionId, 'chat:private:message', chatMessage);
  if (receiverSessionId) wsManager.sendToSession(receiverSessionId, 'chat:private:message', chatMessage);
}

export async function handleChatHistorySync(sessionId: string, data: WSMessage<ChatHistorySyncPayload>) {
  const { userId, targetUserId } = data.payload;
  if (targetUserId) {
    // Private chat history
    const conversationId = getConversationId(userId, targetUserId);
    const messages = privateMessages.get(conversationId) || [];
    wsManager.sendToSession(sessionId, 'chat:history:data', {
      messages,
      totalMessages: messages.length,
      hasMore: false,
    });
  } else {
    // Global chat history
    wsManager.sendToSession(sessionId, 'chat:history:data', {
      messages: globalMessages,
      totalMessages: globalMessages.length,
      hasMore: false,
    });
  }
}

export async function handleChatTyping(sessionId: string, data: WSMessage<ChatTypingPayload>) {
  const { userId, targetUserId, isTyping } = data.payload;
  if (!targetUserId) return;
  const db = getDb();
  const [user] = await db.select().from(users).where(eq(users.id, userId)).limit(1);
  if (!user) return;
  const targetConn = wsManager.getConnectionByUserId(targetUserId);
  const targetSessionId = targetConn ? wsManager.getSessionIdByConnection(targetConn) : undefined;
  if (targetSessionId) {
    wsManager.sendToSession(targetSessionId, 'chat:typing:indicator', {
      userId,
      username: user.name || 'Anonymous',
      isTyping,
    });
  }
}

export async function handleChatMarkRead(sessionId: string, data: WSMessage<{ userId: string; targetUserId: string }>) {
  const { userId, targetUserId } = data.payload;
  const conversationId = getConversationId(userId, targetUserId);
  const messages = privateMessages.get(conversationId);
  if (messages) {
    messages.forEach((msg) => {
      if (msg.sender.userId === targetUserId && !msg.isRead) {
        msg.isRead = true;
      }
    });
  }
}

