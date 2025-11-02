import { Elysia, t } from 'elysia';
import { prisma } from '../utils/prisma';
import { verifyJWT } from '../utils/jwt';

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

      const rooms = await prisma.chatRoom.findMany({
        include: {
          members: {
            include: {
              user: {
                select: {
                  id: true,
                  name: true,
                  email: true,
                  avatar: true,
                },
              },
            },
          },
          messages: {
            take: 1,
            orderBy: {
              createdAt: 'desc',
            },
            include: {
              user: {
                select: {
                  id: true,
                  name: true,
                  email: true,
                  avatar: true,
                },
              },
            },
          },
          _count: {
            select: {
              messages: true,
              members: true,
            },
          },
        },
        orderBy: {
          updatedAt: 'desc',
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

        const room = await prisma.chatRoom.create({
          data: {
            name,
            description,
            isPrivate: isPrivate || false,
            members: {
              create: {
                userId: payload.user_id,
                role: 'admin',
              },
            },
          },
          include: {
            members: {
              include: {
                user: {
                  select: {
                    id: true,
                    name: true,
                    email: true,
                    avatar: true,
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

      // Check if user is a member of the room
      const membership = await prisma.chatRoomMember.findFirst({
        where: {
          roomId,
          userId: payload.user_id,
        },
      });

      if (!membership) {
        set.status = 403;
        throw new Error('Not a member of this chat room');
      }

      const limit = query.limit ? parseInt(query.limit as string) : 50;
      const before = query.before as string | undefined;

      const messages = await prisma.chatMessage.findMany({
        where: {
          roomId,
          ...(before && {
            createdAt: {
              lt: new Date(before),
            },
          }),
        },
        include: {
          user: {
            select: {
              id: true,
              name: true,
              email: true,
              avatar: true,
            },
          },
        },
        orderBy: {
          createdAt: 'desc',
        },
        take: limit,
      });

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

        // Check if user is a member of the room
        const membership = await prisma.chatRoomMember.findFirst({
          where: {
            roomId,
            userId: payload.user_id,
          },
        });

        if (!membership) {
          set.status = 403;
          throw new Error('Not a member of this chat room');
        }

        const message = await prisma.chatMessage.create({
          data: {
            roomId,
            userId: payload.user_id,
            content,
          },
          include: {
            user: {
              select: {
                id: true,
                name: true,
                email: true,
                avatar: true,
              },
            },
          },
        });

        // Update room's updatedAt
        await prisma.chatRoom.update({
          where: { id: roomId },
          data: { updatedAt: new Date() },
        });

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

      // Check if room exists
      const room = await prisma.chatRoom.findUnique({
        where: { id: roomId },
      });

      if (!room) {
        set.status = 404;
        throw new Error('Chat room not found');
      }

      // Check if already a member
      const existingMember = await prisma.chatRoomMember.findFirst({
        where: {
          roomId,
          userId: payload.user_id,
        },
      });

      if (existingMember) {
        return {
          success: true,
          message: 'Already a member',
          member: existingMember,
        };
      }

      const member = await prisma.chatRoomMember.create({
        data: {
          roomId,
          userId: payload.user_id,
          role: 'member',
        },
        include: {
          user: {
            select: {
              id: true,
              name: true,
              email: true,
              avatar: true,
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

      // Find membership
      const membership = await prisma.chatRoomMember.findFirst({
        where: {
          roomId,
          userId: payload.user_id,
        },
      });

      if (!membership) {
        set.status = 404;
        throw new Error('Not a member of this chat room');
      }

      await prisma.chatRoomMember.delete({
        where: {
          id: membership.id,
        },
      });

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

      // Find message
      const message = await prisma.chatMessage.findUnique({
        where: { id: messageId },
      });

      if (!message) {
        set.status = 404;
        throw new Error('Message not found');
      }

      // Check if user is the sender or room admin
      const membership = await prisma.chatRoomMember.findFirst({
        where: {
          roomId: message.roomId,
          userId: payload.user_id,
        },
      });

      const isOwner = message.userId === payload.user_id;
      const isAdmin = membership?.role === 'admin';

      if (!isOwner && !isAdmin) {
        set.status = 403;
        throw new Error('Not authorized to delete this message');
      }

      await prisma.chatMessage.delete({
        where: { id: messageId },
      });

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
