import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '../../../../lib/prisma/service';
import logger from '../../../../utils/logger';
import { verifyJwt } from '../../../../lib/jwt';

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
  let userId: string | undefined;

  try {
    const token = req.cookies.get('token')?.value;
    if (!token) {
      logger.warn(`[POST /api/sosmed/posts] No token provided`, { ip });
      return NextResponse.json({ message: 'Authentication required' }, { status: 401 });
    }

    const decoded = await verifyJwt(token);
    if (!decoded || !decoded.userId) {
      logger.warn(`[POST /api/sosmed/posts] Invalid token or missing userId`, { ip });
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }
    userId = decoded.userId as string;

    const { content, imageUrl } = await req.json();
    logger.debug(`[POST /api/sosmed/posts] Payload`, { content, imageUrl });

    if (!content || typeof content !== 'string') {
      logger.warn(`[POST /api/sosmed/posts] Content required`, { ip });
      return NextResponse.json(
        { message: 'Content is required and must be a string' },
        { status: 400 }
      );
    }

    const newPost = await prisma.posts.create({
      data: {
        content,
        authorId: userId,
        image_url: imageUrl || '',
        userId: userId,
      },
    });

    logger.info(`[POST /api/sosmed/posts] Post created`, {
      ip,
      userId: userId,
      postId: newPost.id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Post created successfully!', post: newPost },
      { status: 201 }
    );
  } catch (error) {
    logger.error(`[POST /api/sosmed/posts] Error`, {
      ip,
      userId: userId,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to create post' },
      { status: 500 }
    );
  }
}

export async function GET(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let userId: string | undefined;

  try {
    const token = req.cookies.get('token')?.value;
    if (!token) {
      logger.warn(`[GET /api/sosmed/posts] No token provided`, { ip });
      return NextResponse.json({ message: 'Authentication required' }, { status: 401 });
    }

    const decoded = await verifyJwt(token);
    if (!decoded || !decoded.userId) {
      logger.warn(`[GET /api/sosmed/posts] Invalid token or missing userId`, { ip });
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }
    userId = decoded.userId as string;

    const posts = await prisma.posts.findMany({
      include: {
        user: {
          select: {
            id: true,
            name: true,
            image: true,
          },
        },
        comments: true,
        likes: true,
      },
      orderBy: {
        created_at: 'desc',
      },
    });

    const sanitizedPosts = await Promise.all(
      posts.map(
        async (post: {
          id: string;
          content: string;
          user: { id: string; name: string | null; image: string | null };
          comments: { id: string; content: string; userId: string }[];
          likes: { userId: string; postId: string }[];
        }) => {
          const commentsWithUser = await Promise.all(
            post.comments.map(async (comment) => {
              const user = await prisma.user.findUnique({
                where: { id: comment.userId },
                select: { id: true, name: true, image: true },
              });
              return {
                ...comment,
                user,
              };
            })
          );

          return {
            ...post,
            user: {
              id: post.user.id,
              name: post.user.name,
              image: post.user.image,
            },
            comments: commentsWithUser,
            likes: post.likes.map((like) => ({
              userId: like.userId,
              postId: like.postId,
            })),
          };
        }
      )
    );

    logger.info(`[GET /api/sosmed/posts] Success`, {
      ip,
      userId: userId,
      count: sanitizedPosts.length,
      durationMs: Date.now() - start,
    });

    return NextResponse.json({ posts: sanitizedPosts }, { status: 200 });
  } catch (error) {
    logger.error(`[GET /api/sosmed/posts] Error`, {
      ip,
      userId: userId,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to fetch posts' },
      { status: 500 }
    );
  }
}

export async function PUT(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let userId: string | undefined;

  try {
    const token = req.cookies.get('token')?.value;
    if (!token) {
      logger.warn(`[PUT /api/sosmed/posts] No token provided`, { ip });
      return NextResponse.json({ message: 'Authentication required' }, { status: 401 });
    }

    const decoded = await verifyJwt(token);
    if (!decoded || !decoded.userId) {
      logger.warn(`[PUT /api/sosmed/posts] Invalid token or missing userId`, { ip });
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }
    userId = decoded.userId as string;

    const { id, content } = await req.json();
    logger.debug(`[PUT /api/sosmed/posts] Payload`, { id, content });

    if (!id || !content || typeof content !== 'string') {
      logger.warn(`[PUT /api/sosmed/posts] ID/content required`, { ip });
      return NextResponse.json(
        { message: 'Post ID and content are required and must be valid' },
        { status: 400 }
      );
    }

    const post = await prisma.posts.findUnique({ where: { id } });

    if (!post || post.userId !== userId) {
      logger.warn(`[PUT /api/sosmed/posts] Not authorized to edit`, {
        ip,
        userId: userId,
        postId: id,
      });
      return NextResponse.json(
        { message: 'User not authorized to edit this post' },
        { status: 403 }
      );
    }

    const updatedPost = await prisma.posts.update({
      where: { id },
      data: {
        content,
      },
    });

    logger.info(`[PUT /api/sosmed/posts] Post updated`, {
      ip,
      userId: userId,
      postId: id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Post updated successfully!', post: updatedPost },
      { status: 200 }
    );
  } catch (error) {
    logger.error(`[PUT /api/sosmed/posts] Error`, {
      ip,
      userId: userId,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to update post' },
      { status: 500 }
    );
  }
}

export async function DELETE(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let userId: string | undefined;

  try {
    const token = req.cookies.get('token')?.value;
    if (!token) {
      logger.warn(`[DELETE /api/sosmed/posts] No token provided`, { ip });
      return NextResponse.json({ message: 'Authentication required' }, { status: 401 });
    }

    const decoded = await verifyJwt(token);
    if (!decoded || !decoded.userId) {
      logger.warn(`[DELETE /api/sosmed/posts] Invalid token or missing userId`, { ip });
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }
    userId = decoded.userId as string;

    const { id } = await req.json();
    logger.debug(`[DELETE /api/sosmed/posts] Payload`, { id });

    if (!id) {
      logger.warn(`[DELETE /api/sosmed/posts] ID required`, { ip });
      return NextResponse.json(
        { message: 'Post ID is required' },
        { status: 400 }
      );
    }

    const post = await prisma.posts.findUnique({ where: { id } });

    if (!post) {
      logger.warn(`[DELETE /api/sosmed/posts] Post not found`, {
        ip,
        postId: id,
      });
      return NextResponse.json({ message: 'Post not found' }, { status: 404 });
    }

    if (post.userId !== userId) {
      logger.warn(`[DELETE /api/sosmed/posts] Not authorized to delete`, {
        ip,
        userId: userId,
        postId: id,
      });
      return NextResponse.json(
        { message: 'User not authorized to delete this post' },
        { status: 403 }
      );
    }

    await prisma.posts.delete({ where: { id } });

    logger.info(`[DELETE /api/sosmed/posts] Post deleted`, {
      ip,
      userId: userId,
      postId: id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Post deleted successfully!' },
      { status: 200 }
    );
  } catch (error) {
    logger.error(`[DELETE /api/sosmed/posts] Error`, {
      ip,
      userId: userId,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to delete post' },
      { status: 500 }
    );
  }
}
