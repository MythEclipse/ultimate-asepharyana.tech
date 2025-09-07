import { NextRequest, NextResponse } from 'next/server';
import logger from '../../../../utils/logger';
import { getDb, Posts } from '@asepharyana/services';

function getIp(req: NextRequest) {
  return (
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown'
  );
}

export async function POST(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  const db = getDb();

  try {
    const { content, imageUrl } = await req.json();
    logger.debug(`[POST /api/sosmed/posts] Payload`, { content, imageUrl });

    if (!content || typeof content !== 'string') {
      logger.warn(`[POST /api/sosmed/posts] Content required`, { ip });
      return NextResponse.json(
        { message: 'Content is required and must be a string' },
        { status: 400 },
      );
    }

    // Use IP as anonymous user identifier
    const anonymousUserId = `anonymous_${ip.replace(/\./g, '_')}`;

    const newPost = (await db
      .insertInto('Posts')
      .values({
        content,
        authorId: anonymousUserId,
        image_url: imageUrl || '',
        userId: anonymousUserId,
      })
      .returningAll()
      .executeTakeFirstOrThrow()) as Posts;

    logger.info(`[POST /api/sosmed/posts] Post created`, {
      ip,
      userId: anonymousUserId,
      postId: newPost.id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Post created successfully!', post: newPost },
      { status: 201 },
    );
  } catch (error) {
    logger.error(`[POST /api/sosmed/posts] Error`, {
      ip,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to create post' },
      { status: 500 },
    );
  }
}

export async function GET(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  const db = getDb();

  try {
    const posts = await db
      .selectFrom('Posts')
      .selectAll()
      .leftJoin('User', 'User.id', 'Posts.userId')
      .select([
        'User.id as user_id',
        'User.name as user_name',
        'User.image as user_image',
      ])
      .orderBy('Posts.created_at', 'desc')
      .execute();

    const sanitizedPosts = await Promise.all(
      posts.map(async (post) => {
        const comments = await db
          .selectFrom('Comments')
          .selectAll()
          .where('Comments.postId', '=', post.id as string) // Cast to string
          .leftJoin('User', 'User.id', 'Comments.userId')
          .select([
            'User.id as user_id',
            'User.name as user_name',
            'User.image as user_image',
          ])
          .execute();

        const likes = await db
          .selectFrom('Likes')
          .selectAll()
          .where('Likes.postId', '=', post.id as string) // Cast to string
          .execute();

        return {
          ...post,
          user: {
            id: post.user_id,
            name: post.user_name,
            image: post.user_image,
          },
          comments: comments.map((comment) => ({
            ...comment,
            user: {
              id: comment.user_id,
              name: comment.user_name,
              image: comment.user_image,
            },
          })),
          likes: likes.map((like) => ({
            userId: like.userId,
            postId: like.postId,
          })),
        };
      }),
    );

    logger.info(`[GET /api/sosmed/posts] Success`, {
      ip,
      count: sanitizedPosts.length,
      durationMs: Date.now() - start,
    });

    return NextResponse.json({ posts: sanitizedPosts }, { status: 200 });
  } catch (error) {
    logger.error(`[GET /api/sosmed/posts] Error`, {
      ip,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to fetch posts' },
      { status: 500 },
    );
  }
}

export async function PUT(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  const db = getDb();

  try {
    const { id, content } = await req.json();
    logger.debug(`[PUT /api/sosmed/posts] Payload`, { id, content });

    if (!id || !content || typeof content !== 'string') {
      logger.warn(`[PUT /api/sosmed/posts] ID/content required`, { ip });
      return NextResponse.json(
        { message: 'Post ID and content are required and must be valid' },
        { status: 400 },
      );
    }

    // Use IP as anonymous user identifier
    const anonymousUserId = `anonymous_${ip.replace(/\./g, '_')}`;

    const post = (await db
      .selectFrom('Posts')
      .selectAll()
      .where('id', '=', id)
      .executeTakeFirst()) as Posts | undefined;

    if (!post || post.userId !== anonymousUserId) {
      logger.warn(`[PUT /api/sosmed/posts] Not authorized to edit`, {
        ip,
        userId: anonymousUserId,
        postId: id,
      });
      return NextResponse.json(
        { message: 'User not authorized to edit this post' },
        { status: 403 },
      );
    }

    const updatedPost = (await db
      .updateTable('Posts')
      .set({
        content,
      })
      .where('id', '=', id)
      .returningAll()
      .executeTakeFirstOrThrow()) as Posts;

    logger.info(`[PUT /api/sosmed/posts] Post updated`, {
      ip,
      userId: anonymousUserId,
      postId: id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Post updated successfully!', post: updatedPost },
      { status: 200 },
    );
  } catch (error) {
    logger.error(`[PUT /api/sosmed/posts] Error`, {
      ip,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to update post' },
      { status: 500 },
    );
  }
}

export async function DELETE(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  const db = getDb();

  try {
    const { id } = await req.json();
    logger.debug(`[DELETE /api/sosmed/posts] Payload`, { id });

    if (!id) {
      logger.warn(`[DELETE /api/sosmed/posts] ID required`, { ip });
      return NextResponse.json(
        { message: 'Post ID is required' },
        { status: 400 },
      );
    }

    // Use IP as anonymous user identifier
    const anonymousUserId = `anonymous_${ip.replace(/\./g, '_')}`;

    const post = (await db
      .selectFrom('Posts')
      .selectAll()
      .where('id', '=', id)
      .executeTakeFirst()) as Posts | undefined;

    if (!post) {
      logger.warn(`[DELETE /api/sosmed/posts] Post not found`, {
        ip,
        postId: id,
      });
      return NextResponse.json({ message: 'Post not found' }, { status: 404 });
    }

    if (post.userId !== anonymousUserId) {
      logger.warn(`[DELETE /api/sosmed/posts] Not authorized to delete`, {
        ip,
        userId: anonymousUserId,
        postId: id,
      });
      return NextResponse.json(
        { message: 'User not authorized to delete this post' },
        { status: 403 },
      );
    }

    await db.deleteFrom('Posts').where('id', '=', id).execute();

    logger.info(`[DELETE /api/sosmed/posts] Post deleted`, {
      ip,
      userId: anonymousUserId,
      postId: id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Post deleted successfully!' },
      { status: 200 },
    );
  } catch (error) {
    logger.error(`[DELETE /api/sosmed/posts] Error`, {
      ip,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to delete post' },
      { status: 500 },
    );
  }
}
