import axios from 'axios';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:4091';
const WS_BASE_URL = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:4091';

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
  const response = await axios.post<{ success: boolean; room: ChatRoom }>(
    `${API_BASE_URL}/api/chat/rooms`,
    { name, description }
  );
  return response.data.room;
};

export const getRooms = async () => {
  const response = await axios.get<{ success: boolean; rooms: ChatRoom[] }>(
    `${API_BASE_URL}/api/chat/rooms`
  );
  return response.data.rooms;
};

export const getRoom = async (roomId: string) => {
  const response = await axios.get<RoomResponse>(
    `${API_BASE_URL}/api/chat/rooms/${roomId}`
  );
  return response.data;
};

export const joinRoom = async (roomId: string) => {
  const response = await axios.post<{ success: boolean; message: string }>(
    `${API_BASE_URL}/api/chat/rooms/${roomId}/join`
  );
  return response.data;
};

export const leaveRoom = async (roomId: string) => {
  const response = await axios.post<{ success: boolean; message: string }>(
    `${API_BASE_URL}/api/chat/rooms/${roomId}/leave`
  );
  return response.data;
};

export const getRoomMembers = async (roomId: string) => {
  const response = await axios.get<{ success: boolean; members: RoomMember[] }>(
    `${API_BASE_URL}/api/chat/rooms/${roomId}/members`
  );
  return response.data.members;
};

// Message APIs
export const sendMessage = async (roomId: string, content: string, messageType = 'text') => {
  const response = await axios.post<{ success: boolean; message: ChatMessage }>(
    `${API_BASE_URL}/api/chat/rooms/${roomId}/messages`,
    { content, message_type: messageType }
  );
  return response.data.message;
};

export const getMessages = async (roomId: string, page = 1, pageSize = 50) => {
  const response = await axios.get<MessagesResponse>(
    `${API_BASE_URL}/api/chat/rooms/${roomId}/messages`,
    { params: { page, page_size: pageSize } }
  );
  return response.data;
};

// WebSocket message type
export interface WsMessage {
  type: 'join' | 'leave' | 'message' | 'user_joined' | 'user_left' | 'error';
  room_id?: string;
  user_id?: string;
  user_name?: string;
  message?: ChatMessage;
}

// WebSocket connection
export const connectWebSocket = (onMessage: (data: WsMessage) => void) => {
  const ws = new WebSocket(`${WS_BASE_URL}/ws/chat`);

  ws.onopen = () => {
    console.log('WebSocket connected');
  };

  ws.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data);
      onMessage(data);
    } catch (error) {
      console.error('Error parsing WebSocket message:', error);
    }
  };

  ws.onerror = (error) => {
    console.error('WebSocket error:', error);
  };

  ws.onclose = () => {
    console.log('WebSocket disconnected');
  };

  return ws;
};
