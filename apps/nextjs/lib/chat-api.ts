import { UnifiedHttpClient } from '../utils/unified-http-client';
import { API_FALLBACK_URLS } from '../utils/url-utils';

// API Fallback Priority:
// 1. localhost:4091 (Rust)
// 2. localhost:3002 (Elysia)
// 3. ws.asepharyana.tech (Production Rust)
// 4. elysia.asepharyana.tech (Production Elysia)
const API_BASE_URLS = typeof window === 'undefined' ? API_FALLBACK_URLS.server : API_FALLBACK_URLS.client;

// WebSocket URLs (same priority)
const WS_BASE_URLS = API_BASE_URLS.map(url => url.replace('https://', 'wss://').replace('http://', 'ws://'));

const client = UnifiedHttpClient.createClientSide();

export interface ChatRoom {
  id: string;
  name: string;
  description?: string;
  created_by: string;
  created_at: string;
  updated_at: string;
}

export interface ChatMessage {
  id: string;
  room_id: string;
  user_id: string;
  user_name: string;
  content: string;
  message_type: string;
  created_at: string;
}

export interface RoomMember {
  room_id: string;
  user_id: string;
  user_name: string;
  joined_at: string;
}

export interface RoomResponse {
  room: ChatRoom;
  members: RoomMember[];
  message_count: number;
}

export interface MessagesResponse {
  messages: ChatMessage[];
  total: number;
  page: number;
  page_size: number;
}

// Room APIs
export const createRoom = async (name: string, description?: string) => {
  const res = await client.request<{ success: boolean; room: ChatRoom }>(
    `/api/chat/rooms`,
    'POST',
    { name, description }
  );
  return res.room;
};

export const getRooms = async () => {
  const res = await client.fetchJson<{ success: boolean; rooms: ChatRoom[] }>(
    `/api/chat/rooms`
  );
  return res.rooms;
};

export const getRoom = async (roomId: string) => {
  return client.fetchJson<RoomResponse>(`/api/chat/rooms/${roomId}`);
};

export const joinRoom = async (roomId: string) => {
  return client.request<{ success: boolean; message: string }>(
    `/api/chat/rooms/${roomId}/join`,
    'POST'
  );
};

export const leaveRoom = async (roomId: string) => {
  return client.request<{ success: boolean; message: string }>(
    `/api/chat/rooms/${roomId}/leave`,
    'POST'
  );
};

export const getRoomMembers = async (roomId: string) => {
  const res = await client.fetchJson<{ success: boolean; members: RoomMember[] }>(
    `/api/chat/rooms/${roomId}/members`
  );
  return res.members;
};

// Message APIs
export const sendMessage = async (roomId: string, content: string, messageType = 'text') => {
  const res = await client.request<{ success: boolean; message: ChatMessage }>(
    `/api/chat/rooms/${roomId}/messages`,
    'POST',
    { content, message_type: messageType }
  );
  return res.message;
};

export const getMessages = async (roomId: string, page = 1, pageSize = 50) => {
  const query = new URLSearchParams({ page: String(page), page_size: String(pageSize) }).toString();
  return client.fetchJson<MessagesResponse>(
    `/api/chat/rooms/${roomId}/messages?${query}`
  );
};

// WebSocket message type
export interface WsMessage {
  type: 'join' | 'leave' | 'message' | 'user_joined' | 'user_left' | 'error';
  room_id?: string;
  user_id?: string;
  user_name?: string;
  message?: ChatMessage;
}

// WebSocket connection with fallback
export const connectWebSocket = (onMessage: (data: WsMessage) => void) => {
  let currentUrlIndex = 0;
  let ws: WebSocket | null = null;

  const tryConnect = () => {
    if (currentUrlIndex >= WS_BASE_URLS.length) {
      console.error('[WebSocket] All endpoints failed');
      return null;
    }

    const wsUrl = `${WS_BASE_URLS[currentUrlIndex]}/ws/chat`;
    console.log(`[WebSocket] Trying ${wsUrl} (${currentUrlIndex + 1}/${WS_BASE_URLS.length})`);

    try {
      ws = new WebSocket(wsUrl);

      ws.onopen = () => {
        console.log(`[WebSocket] Connected to ${wsUrl}`);
      };

      ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          onMessage(data);
        } catch (error) {
          console.error('[WebSocket] Error parsing message:', error);
        }
      };

      ws.onerror = (error) => {
        console.error(`[WebSocket] Error with ${wsUrl}:`, error);
      };

      ws.onclose = () => {
        console.log(`[WebSocket] Disconnected from ${wsUrl}`);
        // Try next endpoint on close
        currentUrlIndex++;
        setTimeout(() => tryConnect(), 2000); // Retry after 2 seconds
      };

      return ws;
    } catch (error) {
      console.error(`[WebSocket] Failed to connect to ${wsUrl}:`, error);
      currentUrlIndex++;
      return tryConnect();
    }
  };

  return tryConnect();
};
