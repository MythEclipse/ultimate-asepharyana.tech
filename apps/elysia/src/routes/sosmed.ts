import { Elysia, t } from 'elysia';
import { prisma } from '../../utils/prisma';
import { verifyJWT } from '../../utils/jwt';

export const sosmedRoutes = new Elysia({ prefix: '/api/sosmed' })
  // Get all posts with comments and likes
  .get('/posts', async ({ headers, set }) => {
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

      const posts = await prisma.post.findMany({
        include: {
          user: {
            select: {
              id: true,
              name: true,
              email: true,
              avatar: true,
            },
          },
          comments: {
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
          },
          likes: {
            include: {
              user: {
                select: {
                  id: true,
                  name: true,
                  email: true,
                },
              },
            },
          },
        },
        orderBy: {
          createdAt: 'desc',
        },
      });

      return {
        success: true,
        posts,
      };
    } catch (error) {
      console.error('Error fetching posts:', error);
      set.status = 500;
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to fetch posts',
      };
    }
  })

  // Create a new post
  .post(
    '/posts',
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

        const { content, imageUrl } = body as { content: string; imageUrl?: string };

        if (!content && !imageUrl) {
          set.status = 400;
          throw new Error('Content or image is required');
        }

        const post = await prisma.post.create({
          data: {
            userId: payload.user_id,
            content: content || '',
            imageUrl,
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
            comments: true,
            likes: true,
          },
        });

        return {
          success: true,
          post,
        };
      } catch (error) {
        console.error('Error creating post:', error);
        set.status = 500;
        return {
          success: false,
          error: error instanceof Error ? error.message : 'Failed to create post',
        };
      }
    },
    {
      body: t.Object({
        content: t.String(),
        imageUrl: t.Optional(t.String()),
      }),
    }
  )

  // Update a post
  .put(
    '/posts/:id',
    async ({ params: { id }, body, headers, set }) => {
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

        const { content, imageUrl } = body as { content: string; imageUrl?: string };

        // Check if post belongs to user
        const existingPost = await prisma.post.findUnique({
          where: { id },
        });

        if (!existingPost) {
          set.status = 404;
          throw new Error('Post not found');
        }

        if (existingPost.userId !== payload.user_id) {
          set.status = 403;
          throw new Error('Not authorized to edit this post');
        }

        const post = await prisma.post.update({
          where: { id },
          data: {
            content,
            imageUrl,
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
            comments: true,
            likes: true,
          },
        });

        return {
          success: true,
          post,
        };
      } catch (error) {
        console.error('Error updating post:', error);
        set.status = 500;
        return {
          success: false,
          error: error instanceof Error ? error.message : 'Failed to update post',
        };
      }
    },
    {
      body: t.Object({
        content: t.String(),
        imageUrl: t.Optional(t.String()),
      }),
    }
  )

  // Delete a post
  .delete('/posts/:id', async ({ params: { id }, headers, set }) => {
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

      // Check if post belongs to user
      const existingPost = await prisma.post.findUnique({
        where: { id },
      });

      if (!existingPost) {
        set.status = 404;
        throw new Error('Post not found');
      }

      if (existingPost.userId !== payload.user_id) {
        set.status = 403;
        throw new Error('Not authorized to delete this post');
      }

      await prisma.post.delete({
        where: { id },
      });

      return {
        success: true,
        message: 'Post deleted successfully',
      };
    } catch (error) {
      console.error('Error deleting post:', error);
      set.status = 500;
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to delete post',
      };
    }
  })

  // Add a comment to a post
  .post(
    '/posts/:id/comments',
    async ({ params: { id }, body, headers, set }) => {
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
          throw new Error('Comment content is required');
        }

        const comment = await prisma.comment.create({
          data: {
            postId: id,
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

        return {
          success: true,
          comment,
        };
      } catch (error) {
        console.error('Error adding comment:', error);
        set.status = 500;
        return {
          success: false,
          error: error instanceof Error ? error.message : 'Failed to add comment',
        };
      }
    },
    {
      body: t.Object({
        content: t.String(),
      }),
    }
  )

  // Update a comment
  .put(
    '/comments/:id',
    async ({ params: { id }, body, headers, set }) => {
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

        // Check if comment belongs to user
        const existingComment = await prisma.comment.findUnique({
          where: { id },
        });

        if (!existingComment) {
          set.status = 404;
          throw new Error('Comment not found');
        }

        if (existingComment.userId !== payload.user_id) {
          set.status = 403;
          throw new Error('Not authorized to edit this comment');
        }

        const comment = await prisma.comment.update({
          where: { id },
          data: { content },
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
          comment,
        };
      } catch (error) {
        console.error('Error updating comment:', error);
        set.status = 500;
        return {
          success: false,
          error: error instanceof Error ? error.message : 'Failed to update comment',
        };
      }
    },
    {
      body: t.Object({
        content: t.String(),
      }),
    }
  )

  // Delete a comment
  .delete('/comments/:id', async ({ params: { id }, headers, set }) => {
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

      // Check if comment belongs to user
      const existingComment = await prisma.comment.findUnique({
        where: { id },
      });

      if (!existingComment) {
        set.status = 404;
        throw new Error('Comment not found');
      }

      if (existingComment.userId !== payload.user_id) {
        set.status = 403;
        throw new Error('Not authorized to delete this comment');
      }

      await prisma.comment.delete({
        where: { id },
      });

      return {
        success: true,
        message: 'Comment deleted successfully',
      };
    } catch (error) {
      console.error('Error deleting comment:', error);
      set.status = 500;
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to delete comment',
      };
    }
  })

  // Like a post
  .post('/posts/:id/like', async ({ params: { id }, headers, set }) => {
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

      // Check if already liked
      const existingLike = await prisma.like.findFirst({
        where: {
          postId: id,
          userId: payload.user_id,
        },
      });

      if (existingLike) {
        set.status = 400;
        return {
          success: false,
          error: 'Post already liked',
        };
      }

      const like = await prisma.like.create({
        data: {
          postId: id,
          userId: payload.user_id,
        },
        include: {
          user: {
            select: {
              id: true,
              name: true,
              email: true,
            },
          },
        },
      });

      return {
        success: true,
        like,
      };
    } catch (error) {
      console.error('Error liking post:', error);
      set.status = 500;
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to like post',
      };
    }
  })

  // Unlike a post
  .delete('/posts/:id/like', async ({ params: { id }, headers, set }) => {
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

      // Find the like
      const existingLike = await prisma.like.findFirst({
        where: {
          postId: id,
          userId: payload.user_id,
        },
      });

      if (!existingLike) {
        set.status = 404;
        return {
          success: false,
          error: 'Like not found',
        };
      }

      await prisma.like.delete({
        where: {
          id: existingLike.id,
        },
      });

      return {
        success: true,
        message: 'Post unliked successfully',
      };
    } catch (error) {
      console.error('Error unliking post:', error);
      set.status = 500;
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to unlike post',
      };
    }
  });
