'use client';

import React, { useState, useEffect, useCallback } from 'react';
import { Users, Settings, MessageCircle as MessageCircleIcon } from 'lucide-react';
import { toast } from 'sonner';

import RoomList from './RoomList';
import MessageList from './MessageList';
import MessageInput from './MessageInput';
import { Button } from '../ui/button';
import {
  ChatRoom,
  ChatMessage,
  getRooms,
  getMessages,
  sendMessage as apiSendMessage,
  joinRoom as apiJoinRoom,
  createRoom as apiCreateRoom,
  connectWebSocket,
  WsMessage,
} from '../../lib/chat-api';

export default function NewChatClient() {
  const [rooms, setRooms] = useState<ChatRoom[]>([]);
  const [selectedRoomId, setSelectedRoomId] = useState<string | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [loading, setLoading] = useState(false);
  const [ws, setWs] = useState<WebSocket | null>(null);

  // Load rooms
  const loadRooms = useCallback(async () => {
    try {
      const fetchedRooms = await getRooms();
      setRooms(fetchedRooms);
    } catch (error) {
      console.error('Error loading rooms:', error);
      toast.error('Failed to load chat rooms');
    }
  }, []);

  // Load messages for selected room
  const loadMessages = useCallback(async (roomId: string) => {
    try {
      setLoading(true);
      const response = await getMessages(roomId);
      setMessages(response.messages);
    } catch (error) {
      console.error('Error loading messages:', error);
      toast.error('Failed to load messages');
    } finally {
      setLoading(false);
    }
  }, []);

  // Send message
  const handleSendMessage = useCallback(async () => {
    if (!selectedRoomId || !inputValue.trim()) return;

    try {
      const message = await apiSendMessage(selectedRoomId, inputValue.trim());
      setInputValue('');

      // Message will be added via WebSocket
      // But add it immediately for better UX
      setMessages((prev) => [...prev, message]);

      // Broadcast via WebSocket
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(
          JSON.stringify({
            type: 'message',
            room_id: selectedRoomId,
            message,
          })
        );
      }
    } catch (error) {
      console.error('Error sending message:', error);
      toast.error('Failed to send message');
    }
  }, [selectedRoomId, inputValue, ws]);

  // Select room
  const handleSelectRoom = useCallback(
    async (roomId: string) => {
      setSelectedRoomId(roomId);
      await loadMessages(roomId);

      // Join room via API
      try {
        await apiJoinRoom(roomId);

        // Notify via WebSocket
        if (ws && ws.readyState === WebSocket.OPEN) {
          ws.send(
            JSON.stringify({
              type: 'join',
              room_id: roomId,
              user_id: 'user_123', // TODO: Get from auth
              user_name: 'User 123',
            })
          );
        }
      } catch (error) {
        console.error('Error joining room:', error);
      }
    },
    [loadMessages, ws]
  );

  // Create room
  const handleCreateRoom = useCallback(async () => {
    const name = prompt('Enter room name:');
    if (!name) return;

    const description = prompt('Enter room description (optional):');

    try {
      const room = await apiCreateRoom(name, description || undefined);
      setRooms((prev) => [room, ...prev]);
      toast.success('Room created successfully');
      handleSelectRoom(room.id);
    } catch (error) {
      console.error('Error creating room:', error);
      toast.error('Failed to create room');
    }
  }, [handleSelectRoom]);

  // WebSocket message handler
  const handleWsMessage = useCallback((data: WsMessage) => {
    switch (data.type) {
      case 'message':
        if (data.message && data.room_id === selectedRoomId) {
          setMessages((prev) => {
            // Avoid duplicates
            if (prev.some((m) => m.id === data.message?.id)) {
              return prev;
            }
            return data.message ? [...prev, data.message] : prev;
          });
        }
        break;
      case 'user_joined':
        if (data.room_id === selectedRoomId) {
          toast.info(`${data.user_name} joined the room`);
        }
        break;
      case 'user_left':
        if (data.room_id === selectedRoomId) {
          toast.info(`${data.user_name} left the room`);
        }
        break;
      case 'error':
        console.error('WebSocket error:', data);
        break;
    }
  }, [selectedRoomId]);

  // Initialize
  useEffect(() => {
    loadRooms();

    // Setup WebSocket
    const websocket = connectWebSocket(handleWsMessage);
    setWs(websocket);

    return () => {
      websocket?.close();
    };
  }, [loadRooms, handleWsMessage]);

  const selectedRoom = rooms.find((r) => r.id === selectedRoomId);

  return (
    <div className="flex h-screen bg-background">
      {/* Room List */}
      <RoomList
        rooms={rooms}
        selectedRoomId={selectedRoomId || undefined}
        onSelectRoom={handleSelectRoom}
        onCreateRoom={handleCreateRoom}
      />

      {/* Chat Area */}
      <div className="flex-1 flex flex-col">
        {selectedRoom ? (
          <>
            {/* Chat Header */}
            <div className="h-16 border-b flex items-center justify-between px-6 bg-background">
              <div>
                <h2 className="font-semibold">{selectedRoom.name}</h2>
                {selectedRoom.description && (
                  <p className="text-sm text-muted-foreground">
                    {selectedRoom.description}
                  </p>
                )}
              </div>
              <div className="flex items-center gap-2">
                <Button variant="ghost" size="icon">
                  <Users className="h-5 w-5" />
                </Button>
                <Button variant="ghost" size="icon">
                  <Settings className="h-5 w-5" />
                </Button>
              </div>
            </div>

            {/* Messages */}
            <MessageList messages={messages} currentUserId="user_123" />

            {/* Input */}
            <MessageInput
              value={inputValue}
              onChange={setInputValue}
              onSend={handleSendMessage}
              disabled={loading}
            />
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center text-muted-foreground">
            <div className="text-center">
              <MessageCircleIcon className="h-16 w-16 mx-auto mb-4 opacity-50" />
              <p className="text-lg font-medium">No room selected</p>
              <p className="text-sm mt-1">
                Select a room or create a new one to start chatting
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
