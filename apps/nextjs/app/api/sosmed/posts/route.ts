import { NextResponse } from 'next/server';
import { prisma } from '../../../../lib/prisma/service';
import logger from '../../../../utils/logger';
import { auth } from '../../../../auth';

function getIp(req: Request) {
  return (
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown'
  );
}

export async function POST(req: Request) {
  const session = await auth();
  const start = Date.now();
  const ip = getIp(req);

  if (!session?.user || !session.user.id) {
    logger.warn(`[POST /api/sosmed/posts] Unauthorized`, { ip });
    return NextResponse.json(
      { message: 'User not authenticated' },
      { status: 401 }
    );
  }

  try {
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
        authorId: session.user.id,
        image_url: imageUrl || '',
        userId: session.user.id,
      },
    });

    logger.info(`[POST /api/sosmed/posts] Post created`, {
      ip,
      userId: session.user.id,
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
      userId: session.user.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to create post' },
      { status: 500 }
    );
  }
}

export async function GET(req: Request) {
  const session = await auth();
  const start = Date.now();
  const ip = getIp(req);

  if (!session?.user || !session.user.id) {
    logger.warn(`[GET /api/sosmed/posts] Unauthorized`, { ip });
    return NextResponse.json(
      { message: 'User not authenticated' },
      { status: 401 }
    );
  }

  try {
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
      userId: session.user.id,
      count: sanitizedPosts.length,
      durationMs: Date.now() - start,
    });

    return NextResponse.json({ posts: sanitizedPosts }, { status: 200 });
  } catch (error) {
    logger.error(`[GET /api/sosmed/posts] Error`, {
      ip,
      userId: session.user.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to fetch posts' },
      { status: 500 }
    );
  }
}
export async function PUT(req: Request) {
  const session = await auth();
  const start = Date.now();
  const ip = getIp(req);

  if (!session?.user || !session.user.id) {
    logger.warn(`[PUT /api/sosmed/posts] Unauthorized`, { ip });
    return NextResponse.json(
      { message: 'User not authenticated' },
      { status: 401 }
    );
  }

  try {
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

    if (!post || post.userId !== session.user.id) {
      logger.warn(`[PUT /api/sosmed/posts] Not authorized to edit`, {
        ip,
        userId: session.user.id,
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
      userId: session.user.id,
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
      userId: session.user.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to update post' },
      { status: 500 }
    );
  }
}

export async function DELETE(req: Request) {
  const session = await auth();
  const start = Date.now();
  const ip = getIp(req);

  if (!session?.user || !session.user.id) {
    logger.warn(`[DELETE /api/sosmed/posts] Unauthorized`, { ip });
    return NextResponse.json(
      { message: 'User not authenticated' },
      { status: 401 }
    );
  }

  try {
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

    if (post.userId !== session.user.id) {
      logger.warn(`[DELETE /api/sosmed/posts] Not authorized to delete`, {
        ip,
        userId: session.user.id,
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
      userId: session.user.id,
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
      userId: session.user.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to delete post' },
      { status: 500 }
    );
  }
}
