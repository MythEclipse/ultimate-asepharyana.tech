import { Elysia, t } from 'elysia';
import { getDatabase } from '../utils/prisma';
import { verifyJWT } from '../utils/jwt';
import {
  chatRooms,
  chatRoomMembers,
  chatMessagesWithRoom,
  NewChatRoom,
  NewChatRoomMember,
} from '@asepharyana/services';
import { eq, and } from '@asepharyana/services';

export const chatRoutes = new Elysia({ prefix: '/api/chat' })
  // Get all chat rooms
  .get('/rooms', async ({ headers, set }) => {
    try {
      const authHeader = headers.authorization;
      if (!authHeader || !authHeader.startsWith('Bearer ')) {
        set.status = 401;
        throw new Error('Unauthorized');
      }

      const token = authHeader.substring(7);
      const payload = await verifyJWT(token);
      if (!payload) {
        set.status = 401;
        throw new Error('Invalid token');
      }

      const db = getDatabase();

      const rooms = await db.query.chatRooms.findMany({
        orderBy: (chatRooms, { desc }) => [desc(chatRooms.updatedAt)],
        with: {
          members: {
            with: {
              user: {
                columns: {
                  id: true,
                  name: true,
                  email: true,
                  image: true,
                },
              },
            },
          },
          messages: {
            limit: 1,
            orderBy: (messages, { desc }) => [desc(messages.createdAt)],
            with: {
              user: {
                columns: {
                  id: true,
                  name: true,
                  email: true,
                  image: true,
                },
              },
            },
          },
        },
      });

      return {
        success: true,
        rooms,
      };
    } catch (error) {
      console.error('Error fetching chat rooms:', error);
      set.status = 500;
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to fetch chat rooms',
      };
    }
  })

  // Create a new chat room
  .post(
    '/rooms',
    async ({ body, headers, set }) => {
      try {
        const authHeader = headers.authorization;
        if (!authHeader || !authHeader.startsWith('Bearer ')) {
          set.status = 401;
          throw new Error('Unauthorized');
        }

        const token = authHeader.substring(7);
        const payload = await verifyJWT(token);
        if (!payload) {
          set.status = 401;
          throw new Error('Invalid token');
        }

        const { name, description, isPrivate } = body as {
          name: string;
          description?: string;
          isPrivate?: boolean;
        };

        if (!name.trim()) {
          set.status = 400;
          throw new Error('Room name is required');
        }

        const db = getDatabase();
        const roomId = `room_${Date.now()}_${payload.user_id}`;
        const memberId = `member_${Date.now()}_${payload.user_id}`;

        // Create room
        const newRoom: NewChatRoom = {
          id: roomId,
          name,
          description: description || null,
          isPrivate: isPrivate ? 1 : 0,
        };

        await db.insert(chatRooms).values(newRoom);

        // Add creator as admin member
        const newMember: NewChatRoomMember = {
          id: memberId,
          roomId,
          userId: payload.user_id,
          role: 'admin',
        };

        await db.insert(chatRoomMembers).values(newMember);

        // Query with relations
        const room = await db.query.chatRooms.findFirst({
          where: (chatRooms, { eq }) => eq(chatRooms.id, roomId),
          with: {
            members: {
              with: {
                user: {
                  columns: {
                    id: true,
                    name: true,
                    email: true,
                    image: true,
                  },
                },
              },
            },
          },
        });

        return {
          success: true,
          room,
        };
      } catch (error) {
        console.error('Error creating chat room:', error);
        set.status = 500;
        return {
          success: false,
          error: error instanceof Error ? error.message : 'Failed to create chat room',
        };
      }
    },
    {
      body: t.Object({
        name: t.String(),
        description: t.Optional(t.String()),
        isPrivate: t.Optional(t.Boolean()),
      }),
    }
  )

  // Get messages from a chat room
  .get('/rooms/:roomId/messages', async ({ params: { roomId }, query, headers, set }) => {
    try {
      const authHeader = headers.authorization;
      if (!authHeader || !authHeader.startsWith('Bearer ')) {
        set.status = 401;
        throw new Error('Unauthorized');
      }

      const token = authHeader.substring(7);
      const payload = await verifyJWT(token);
      if (!payload) {
        set.status = 401;
        throw new Error('Invalid token');
      }

      const db = getDatabase();

      // Check if user is a member of the room
      const membershipResult = await db
        .select()
        .from(chatRoomMembers)
        .where(and(eq(chatRoomMembers.roomId, roomId), eq(chatRoomMembers.userId, payload.user_id)))
        .limit(1);

      if (membershipResult.length === 0) {
        set.status = 403;
        throw new Error('Not a member of this chat room');
      }

      const limit = query.limit ? parseInt(query.limit as string) : 50;
      const before = query.before as string | undefined;

      const messagesQuery = db.query.chatMessagesWithRoom.findMany({
        where: before
          ? and(
              eq(chatMessagesWithRoom.roomId, roomId),
              // lt(chatMessagesWithRoom.createdAt, new Date(before))
            )
          : eq(chatMessagesWithRoom.roomId, roomId),
        with: {
          user: {
            columns: {
              id: true,
              name: true,
              email: true,
              image: true,
            },
          },
        },
        orderBy: (messages, { desc }) => [desc(messages.createdAt)],
        limit,
      });

      const messages = await messagesQuery;

      return {
        success: true,
        messages: messages.reverse(),
      };
    } catch (error) {
      console.error('Error fetching messages:', error);
      set.status = 500;
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to fetch messages',
      };
    }
  })

  // Send a message to a chat room
  .post(
    '/rooms/:roomId/messages',
    async ({ params: { roomId }, body, headers, set }) => {
      try {
        const authHeader = headers.authorization;
        if (!authHeader || !authHeader.startsWith('Bearer ')) {
          set.status = 401;
          throw new Error('Unauthorized');
        }

        const token = authHeader.substring(7);
        const payload = await verifyJWT(token);
        if (!payload) {
          set.status = 401;
          throw new Error('Invalid token');
        }

        const { content } = body as { content: string };

        if (!content.trim()) {
          set.status = 400;
          throw new Error('Message content is required');
        }

        const db = getDatabase();

        // Check if user is a member of the room
        const membershipResult = await db
          .select()
          .from(chatRoomMembers)
          .where(and(eq(chatRoomMembers.roomId, roomId), eq(chatRoomMembers.userId, payload.user_id)))
          .limit(1);

        if (membershipResult.length === 0) {
          set.status = 403;
          throw new Error('Not a member of this chat room');
        }

        const messageId = `msg_${Date.now()}_${payload.user_id}`;
        const newMessage = {
          id: messageId,
          roomId,
          userId: payload.user_id,
          content,
        };

        await db.insert(chatMessagesWithRoom).values(newMessage);

        const message = await db.query.chatMessagesWithRoom.findFirst({
          where: (messages, { eq }) => eq(messages.id, messageId),
          with: {
            user: {
              columns: {
                id: true,
                name: true,
                email: true,
                image: true,
              },
            },
          },
        });

        // Update room's updatedAt
        await db
          .update(chatRooms)
          .set({ updatedAt: new Date() })
          .where(eq(chatRooms.id, roomId));

        return {
          success: true,
          message,
        };
      } catch (error) {
        console.error('Error sending message:', error);
        set.status = 500;
        return {
          success: false,
          error: error instanceof Error ? error.message : 'Failed to send message',
        };
      }
    },
    {
      body: t.Object({
        content: t.String(),
      }),
    }
  )

  // Join a chat room
  .post('/rooms/:roomId/join', async ({ params: { roomId }, headers, set }) => {
    try {
      const authHeader = headers.authorization;
      if (!authHeader || !authHeader.startsWith('Bearer ')) {
        set.status = 401;
        throw new Error('Unauthorized');
      }

      const token = authHeader.substring(7);
      const payload = await verifyJWT(token);
      if (!payload) {
        set.status = 401;
        throw new Error('Invalid token');
      }

      const db = getDatabase();

      // Check if room exists
      const roomResult = await db
        .select()
        .from(chatRooms)
        .where(eq(chatRooms.id, roomId))
        .limit(1);

      if (roomResult.length === 0) {
        set.status = 404;
        throw new Error('Chat room not found');
      }

      // Check if already a member
      const existingMemberResult = await db
        .select()
        .from(chatRoomMembers)
        .where(and(eq(chatRoomMembers.roomId, roomId), eq(chatRoomMembers.userId, payload.user_id)))
        .limit(1);

      if (existingMemberResult.length > 0) {
        return {
          success: true,
          message: 'Already a member',
          member: existingMemberResult[0],
        };
      }

      const memberId = `member_${Date.now()}_${payload.user_id}_${roomId}`;
      const newMember: NewChatRoomMember = {
        id: memberId,
        roomId,
        userId: payload.user_id,
        role: 'member',
      };

      await db.insert(chatRoomMembers).values(newMember);

      const member = await db.query.chatRoomMembers.findFirst({
        where: (members, { eq }) => eq(members.id, memberId),
        with: {
          user: {
            columns: {
              id: true,
              name: true,
              email: true,
              image: true,
            },
          },
        },
      });

      return {
        success: true,
        message: 'Joined chat room successfully',
        member,
      };
    } catch (error) {
      console.error('Error joining chat room:', error);
      set.status = 500;
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to join chat room',
      };
    }
  })

  // Leave a chat room
  .post('/rooms/:roomId/leave', async ({ params: { roomId }, headers, set }) => {
    try {
      const authHeader = headers.authorization;
      if (!authHeader || !authHeader.startsWith('Bearer ')) {
        set.status = 401;
        throw new Error('Unauthorized');
      }

      const token = authHeader.substring(7);
      const payload = await verifyJWT(token);
      if (!payload) {
        set.status = 401;
        throw new Error('Invalid token');
      }

      const db = getDatabase();

      // Find membership
      const membershipResult = await db
        .select()
        .from(chatRoomMembers)
        .where(and(eq(chatRoomMembers.roomId, roomId), eq(chatRoomMembers.userId, payload.user_id)))
        .limit(1);

      if (membershipResult.length === 0) {
        set.status = 404;
        throw new Error('Not a member of this chat room');
      }

      await db
        .delete(chatRoomMembers)
        .where(eq(chatRoomMembers.id, membershipResult[0].id));

      return {
        success: true,
        message: 'Left chat room successfully',
      };
    } catch (error) {
      console.error('Error leaving chat room:', error);
      set.status = 500;
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to leave chat room',
      };
    }
  })

  // Delete a message (only by sender or room admin)
  .delete('/messages/:messageId', async ({ params: { messageId }, headers, set }) => {
    try {
      const authHeader = headers.authorization;
      if (!authHeader || !authHeader.startsWith('Bearer ')) {
        set.status = 401;
        throw new Error('Unauthorized');
      }

      const token = authHeader.substring(7);
      const payload = await verifyJWT(token);
      if (!payload) {
        set.status = 401;
        throw new Error('Invalid token');
      }

      const db = getDatabase();

      // Find message
      const messageResult = await db
        .select()
        .from(chatMessagesWithRoom)
        .where(eq(chatMessagesWithRoom.id, messageId))
        .limit(1);

      if (messageResult.length === 0) {
        set.status = 404;
        throw new Error('Message not found');
      }

      const message = messageResult[0];

      // Check if user is the sender or room admin
      const membershipResult = await db
        .select()
        .from(chatRoomMembers)
        .where(and(eq(chatRoomMembers.roomId, message.roomId), eq(chatRoomMembers.userId, payload.user_id)))
        .limit(1);

      const isOwner = message.userId === payload.user_id;
      const isAdmin = membershipResult.length > 0 && membershipResult[0].role === 'admin';

      if (!isOwner && !isAdmin) {
        set.status = 403;
        throw new Error('Not authorized to delete this message');
      }

      await db.delete(chatMessagesWithRoom).where(eq(chatMessagesWithRoom.id, messageId));

      return {
        success: true,
        message: 'Message deleted successfully',
      };
    } catch (error) {
      console.error('Error deleting message:', error);
      set.status = 500;
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to delete message',
      };
    }
  });
