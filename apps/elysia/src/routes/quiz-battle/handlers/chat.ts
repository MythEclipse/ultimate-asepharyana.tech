// Chat System Handlers (In-Memory MVP)
import { wsManager } from '../ws-manager';
import type {
  WSMessage,
  ChatGlobalSendPayload,
  ChatGlobalMessagePayload,
  ChatPrivateSendPayload,
  ChatPrivateMessagePayload,
  ChatHistorySyncPayload,
  ChatHistoryDataPayload,
  ChatTypingPayload,
  ChatTypingIndicatorPayload,
} from '../types';
import { getDb, users, quizUserStats, eq } from '@asepharyana/services';

const globalMessages: ChatGlobalMessagePayload[] = [];
const privateMessages = new Map<string, ChatPrivateMessagePayload[]>();
const MAX_HISTORY = 100;

function getConversationId(userId1: string, userId2: string): string {
  return [userId1, userId2].sort().join('_');
}

export async function handleChatGlobalSend(ws: any, data: WSMessage<ChatGlobalSendPayload>) {
  const { userId, message } = data.payload;
  if (!message || message.trim().length === 0 || message.length > 500) {
    wsManager.sendToSession(ws, 'error', { message: 'Invalid message', code: 'INVALID_MESSAGE' });
    return;
  }
  const [user] = await getDb().select().from(users).where(eq(users.id, userId)).limit(1);
  if (!user) return;
  const chatMessage: ChatGlobalMessagePayload = {
    messageId: `msg_${Date.now()}_${userId}`,
    userId, username: user.username,
    message: message.trim(),
    timestamp: new Date().toISOString(),
  };
  globalMessages.push(chatMessage);
  if (globalMessages.length > MAX_HISTORY) globalMessages.shift();
  const connections = wsManager.getAllConnections();
  for (const conn of connections) wsManager.sendToSession(conn, 'chat.global.message', chatMessage);
}

export async function handleChatPrivateSend(ws: any, data: WSMessage<ChatPrivateSendPayload>) {
  const { userId, targetUserId, message } = data.payload;
  if (!message || message.trim().length === 0 || message.length > 1000) {
    wsManager.sendToSession(ws, 'error', { message: 'Invalid message', code: 'INVALID_MESSAGE' });
    return;
  }
  const [sender] = await getDb().select().from(users).where(eq(users.id, userId)).limit(1);
  const [receiver] = await getDb().select().from(users).where(eq(users.id, targetUserId)).limit(1);
  if (!sender || !receiver) return;
  const conversationId = getConversationId(userId, targetUserId);
  const chatMessage: ChatPrivateMessagePayload = {
    messageId: `pm_${Date.now()}_${userId}`,
    senderId: userId, senderUsername: sender.username,
    receiverId: targetUserId, receiverUsername: receiver.username,
    message: message.trim(),
    timestamp: new Date().toISOString(),
    isRead: false,
  };
  if (!privateMessages.has(conversationId)) privateMessages.set(conversationId, []);
  const conversation = privateMessages.get(conversationId)!;
  conversation.push(chatMessage);
  if (conversation.length > MAX_HISTORY) conversation.shift();
  wsManager.sendToSession(ws, 'chat.private.message', chatMessage);
  const receiverSession = wsManager.findSessionByUserId(targetUserId);
  if (receiverSession) wsManager.sendToSession(receiverSession, 'chat.private.message', chatMessage);
}

export async function handleChatHistorySync(ws: any, data: WSMessage<ChatHistorySyncPayload>) {
  const { userId, targetUserId, limit = 50, offset = 0 } = data.payload;
  let messages: any[] = [];
  let totalMessages = 0;
  if (targetUserId) {
    const conversationId = getConversationId(userId, targetUserId);
    const conversation = privateMessages.get(conversationId) || [];
    totalMessages = conversation.length;
    messages = conversation.slice(Math.max(0, conversation.length - offset - limit), conversation.length - offset);
  } else {
    totalMessages = globalMessages.length;
    messages = globalMessages.slice(Math.max(0, globalMessages.length - offset - limit), globalMessages.length - offset);
  }
  const response: ChatHistoryDataPayload = { messages, totalMessages, hasMore: offset + limit < totalMessages };
  wsManager.sendToSession(ws, 'chat.history.data', response);
}

export async function handleChatTyping(ws: any, data: WSMessage<ChatTypingPayload>) {
  const { userId, targetUserId, isTyping } = data.payload;
  const [user] = await getDb().select().from(users).where(eq(users.id, userId)).limit(1);
  if (!user) return;
  const typingData: ChatTypingIndicatorPayload = { userId, username: user.username, isTyping };
  if (targetUserId) {
    const targetSession = wsManager.findSessionByUserId(targetUserId);
    if (targetSession) wsManager.sendToSession(targetSession, 'chat.typing.indicator', typingData);
  } else {
    const connections = wsManager.getAllConnections();
    for (const conn of connections) if (conn !== ws) wsManager.sendToSession(conn, 'chat.typing.indicator', typingData);
  }
}

export async function handleChatMarkRead(ws: any, data: WSMessage<{ userId: string; targetUserId: string }>) {
  const { userId, targetUserId } = data.payload;
  const conversationId = getConversationId(userId, targetUserId);
  const conversation = privateMessages.get(conversationId);
  if (conversation) {
    for (const msg of conversation) {
      if (msg.receiverId === userId && msg.senderId === targetUserId) msg.isRead = true;
    }
  }
  wsManager.sendToSession(ws, 'chat.mark.read.success', { targetUserId, timestamp: new Date().toISOString() });
}
