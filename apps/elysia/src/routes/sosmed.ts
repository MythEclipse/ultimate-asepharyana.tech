import { Elysia, t } from 'elysia';
import { getDb, posts, comments, likes, eq, desc, and } from '@asepharyana/services';
import type { NewPost, NewComment, NewLike } from '@asepharyana/services';
import { verifyJWT } from '../utils/jwt';

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

      const db = getDb();

      const postsList = await db.query.posts.findMany({
        orderBy: desc(posts.created_at),
        with: {
          user: {
            columns: {
              id: true,
              name: true,
              email: true,
              image: true,
            },
          },
          comments: {
            orderBy: desc(comments.created_at),
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
          likes: {
            with: {
              user: {
                columns: {
                  id: true,
                  name: true,
                  email: true,
                },
              },
            },
          },
        },
      });

      return {
        success: true,
        posts: postsList,
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

        const db = getDb();
        const postId = `post_${Date.now()}_${payload.user_id}`;

        const newPost: NewPost = {
          id: postId,
          userId: payload.user_id,
          authorId: payload.user_id,
          content: content || '',
          image_url: imageUrl || null,
        };

        await db.insert(posts).values(newPost);

        const createdPost = await db.query.posts.findFirst({
          where: eq(posts.id, postId),
          with: {
            user: {
              columns: {
                id: true,
                name: true,
                email: true,
                image: true,
              },
            },
            comments: true,
            likes: true,
          },
        });

        return {
          success: true,
          post: createdPost,
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

        const db = getDb();
        const existingPostResult = await db
          .select()
          .from(posts)
          .where(eq(posts.id, id))
          .limit(1);

        const existingPost = existingPostResult[0];

        if (!existingPost) {
          set.status = 404;
          throw new Error('Post not found');
        }

        if (existingPost.userId !== payload.user_id) {
          set.status = 403;
          throw new Error('Not authorized to edit this post');
        }

        await db
          .update(posts)
          .set({
            content,
            image_url: imageUrl || null,
          })
          .where(eq(posts.id, id));

        const updatedPost = await db.query.posts.findFirst({
          where: eq(posts.id, id),
          with: {
            user: {
              columns: {
                id: true,
                name: true,
                email: true,
                image: true,
              },
            },
            comments: true,
            likes: true,
          },
        });

        return {
          success: true,
          post: updatedPost,
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

      const db = getDb();
      const existingPostResult = await db
        .select()
        .from(posts)
        .where(eq(posts.id, id))
        .limit(1);

      const existingPost = existingPostResult[0];

      if (!existingPost) {
        set.status = 404;
        throw new Error('Post not found');
      }

      if (existingPost.userId !== payload.user_id) {
        set.status = 403;
        throw new Error('Not authorized to delete this post');
      }

      await db.delete(posts).where(eq(posts.id, id));

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

        const db = getDb();
        const commentId = `comment_${Date.now()}_${payload.user_id}`;
        const newComment: NewComment = {
          id: commentId,
          postId: id,
          userId: payload.user_id,
          authorId: payload.user_id,
          content,
          created_at: new Date(),
        };

        await db.insert(comments).values(newComment);

        const comment = await db.query.comments.findFirst({
          where: eq(comments.id, commentId),
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

        const db = getDb();
        const existingCommentResult = await db
          .select()
          .from(comments)
          .where(eq(comments.id, id))
          .limit(1);

        const existingComment = existingCommentResult[0];

        if (!existingComment) {
          set.status = 404;
          throw new Error('Comment not found');
        }

        if (existingComment.userId !== payload.user_id) {
          set.status = 403;
          throw new Error('Not authorized to edit this comment');
        }

        await db
          .update(comments)
          .set({ content })
          .where(eq(comments.id, id));

        const comment = await db.query.comments.findFirst({
          where: eq(comments.id, id),
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

      const db = getDb();
      const existingCommentResult = await db
        .select()
        .from(comments)
        .where(eq(comments.id, id))
        .limit(1);

      const existingComment = existingCommentResult[0];

      if (!existingComment) {
        set.status = 404;
        throw new Error('Comment not found');
      }

      if (existingComment.userId !== payload.user_id) {
        set.status = 403;
        throw new Error('Not authorized to delete this comment');
      }

      await db.delete(comments).where(eq(comments.id, id));

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

      const db = getDb();

      // Check if already liked
      const existingLikeResult = await db
        .select()
        .from(likes)
        .where(and(eq(likes.postId, id), eq(likes.userId, payload.user_id)))
        .limit(1);

      if (existingLikeResult.length > 0) {
        set.status = 400;
        return {
          success: false,
          error: 'Post already liked',
        };
      }

      const newLike: NewLike = {
        postId: id,
        userId: payload.user_id,
      };

      await db.insert(likes).values(newLike);

      const like = await db.query.likes.findFirst({
        where: and(eq(likes.postId, id), eq(likes.userId, payload.user_id)),
        with: {
          user: {
            columns: {
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

      const db = getDb();

      // Find the like
      const existingLikeResult = await db
        .select()
        .from(likes)
        .where(and(eq(likes.postId, id), eq(likes.userId, payload.user_id)))
        .limit(1);

      if (existingLikeResult.length === 0) {
        set.status = 404;
        return {
          success: false,
          error: 'Like not found',
        };
      }

      await db
        .delete(likes)
        .where(and(eq(likes.postId, id), eq(likes.userId, payload.user_id)));

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
