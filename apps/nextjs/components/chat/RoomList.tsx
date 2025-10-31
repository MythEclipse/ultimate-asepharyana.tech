'use client';

import React from 'react';
import { Plus, MessageCircle } from 'lucide-react';
import { ChatRoom } from '../../lib/chat-api';
import { Button } from '../ui/button';
import { format } from 'date-fns';

interface RoomListProps {
  rooms: ChatRoom[];
  selectedRoomId?: string;
  onSelectRoom: (roomId: string) => void;
  onCreateRoom: () => void;
}

export default function RoomList({
  rooms,
  selectedRoomId,
  onSelectRoom,
  onCreateRoom,
}: RoomListProps) {
  return (
    <div className="w-80 border-r bg-muted/30 flex flex-col">
      <div className="p-4 border-b bg-background">
        <div className="flex items-center justify-between mb-2">
          <h2 className="text-lg font-semibold">Chat Rooms</h2>
          <Button onClick={onCreateRoom} size="icon" variant="ghost">
            <Plus className="h-5 w-5" />
          </Button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto">
        {rooms.length === 0 ? (
          <div className="p-4 text-center text-muted-foreground">
            <MessageCircle className="h-12 w-12 mx-auto mb-2 opacity-50" />
            <p className="text-sm">No rooms yet</p>
            <p className="text-xs mt-1">Create one to start chatting</p>
          </div>
        ) : (
          <div className="space-y-1 p-2">
            {rooms.map((room) => (
              <button
                key={room.id}
                onClick={() => onSelectRoom(room.id)}
                className={`w-full text-left p-3 rounded-lg transition-colors ${
                  selectedRoomId === room.id
                    ? 'bg-primary text-primary-foreground'
                    : 'hover:bg-muted'
                }`}
              >
                <div className="flex items-center justify-between mb-1">
                  <h3 className="font-medium truncate">{room.name}</h3>
                  <MessageCircle className="h-4 w-4 flex-shrink-0 ml-2" />
                </div>
                {room.description && (
                  <p className="text-xs opacity-70 truncate">
                    {room.description}
                  </p>
                )}
                <p className="text-xs opacity-50 mt-1">
                  {format(new Date(room.updated_at), 'MMM dd, HH:mm')}
                </p>
              </button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
