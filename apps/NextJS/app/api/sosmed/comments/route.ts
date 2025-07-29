import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma/service';
import logger from '@/lib/logger';
import { auth } from '@/auth';

function getIp(req: NextRequest) {
  return (
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown'
  );
}

async function postHandler(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let user;
  try {
    const session = await auth();
    user = session?.user;
    
    if (!user || !user.id) {
      logger.warn(`[POST /api/sosmed/comments] Unauthorized`, { ip });
      return NextResponse.json({ message: 'Unauthorized' }, { status: 401 });
    }

    const { postId, content } = await req.json();
    logger.debug(`[POST /api/sosmed/comments] Payload`, { postId, content });

    if (!content) {
      logger.warn(`[POST /api/sosmed/comments] Content required`, { ip });
      return NextResponse.json(
        { message: 'Content is required' },
        { status: 400 }
      );
    }

    const comment = await prisma.comments.create({
      data: {
        postId,
        content,
        userId: user.id,
        authorId: user.id,
      },
    });

    logger.info(`[POST /api/sosmed/comments] Comment created`, {
      ip,
      userId: user.id,
      commentId: comment.id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      {
        comment: {
          id: comment.id,
          postId: comment.postId,
          content: comment.content,
          created_at: comment.created_at,
        },
      },
      { status: 201 }
    );
  } catch (error) {
    logger.error(`[POST /api/sosmed/comments] Error`, {
      ip,
      userId: user?.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to add comment' },
      { status: 500 }
    );
  }
}

async function getHandler(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  try {
    const { searchParams } = new URL(req.url);
    const postId = searchParams.get('postId');
    

    if (!postId) {
      logger.warn(`[GET /api/sosmed/comments] Post ID required`, { ip });
      return NextResponse.json(
        { message: 'Post ID is required' },
        { status: 400 }
      );
    }

    const comments = await prisma.comments.findMany({
      where: { postId: postId as string },
      include: {
        user: {
          select: {
            id: true,
            name: true,
            image: true,
          },
        },
      },
      orderBy: { created_at: 'desc' },
    });

    logger.info(`[GET /api/sosmed/comments] Success`, {
      ip,
      postId,
      count: comments.length,
      durationMs: Date.now() - start,
    });

    return NextResponse.json({ comments }, { status: 200 });
  } catch (error) {
    logger.error(`[GET /api/sosmed/comments] Error`, {
      ip,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to fetch comments' },
      { status: 500 }
    );
  }
}

async function putHandler(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let user;
  try {
    const session = await auth();
    user = session?.user;
    

    if (!user || !user.id) {
      logger.warn(`[PUT /api/sosmed/comments] Unauthorized`, { ip });
      return NextResponse.json({ message: 'Unauthorized' }, { status: 401 });
    }

    const { id, content } = await req.json();
    logger.debug(`[PUT /api/sosmed/comments] Payload`, { id, content });

    if (!id || !content) {
      logger.warn(`[PUT /api/sosmed/comments] ID/content required`, { ip });
      return NextResponse.json(
        { message: 'Comment ID and content are required' },
        { status: 400 }
      );
    }

    const comment = await prisma.comments.findUnique({ where: { id } });

    if (!comment || comment.userId !== user.id) {
      logger.warn(`[PUT /api/sosmed/comments] Not authorized to edit`, {
        ip,
        userId: user.id,
        commentId: id,
      });
      return NextResponse.json(
        { message: 'User not authorized to edit this comment' },
        { status: 403 }
      );
    }

    const updatedComment = await prisma.comments.update({
      where: { id },
      data: {
        content: `${content} -edited`,
      },
    });

    logger.info(`[PUT /api/sosmed/comments] Comment updated`, {
      ip,
      userId: user.id,
      commentId: id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Comment updated successfully!', comment: updatedComment },
      { status: 200 }
    );
  } catch (error) {
    logger.error(`[PUT /api/sosmed/comments] Error`, {
      ip,
      userId: user?.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to update comment' },
      { status: 500 }
    );
  }
}

async function deleteHandler(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let user;
  try {
    const session = await auth();
    user = session?.user;
    

    if (!user || !user.id) {
      logger.warn(`[DELETE /api/sosmed/comments] Unauthorized`, { ip });
      return NextResponse.json({ message: 'Unauthorized' }, { status: 401 });
    }

    const { id } = await req.json();
    logger.debug(`[DELETE /api/sosmed/comments] Payload`, { id });

    if (!id) {
      logger.warn(`[DELETE /api/sosmed/comments] ID required`, { ip });
      return NextResponse.json(
        { message: 'Comment ID is required' },
        { status: 400 }
      );
    }

    const comment = await prisma.comments.findUnique({ where: { id } });

    if (!comment || comment.userId !== user.id) {
      logger.warn(`[DELETE /api/sosmed/comments] Not authorized to delete`, {
        ip,
        userId: user.id,
        commentId: id,
      });
      return NextResponse.json(
        { message: 'User not authorized to delete this comment' },
        { status: 403 }
      );
    }

    await prisma.comments.delete({ where: { id } });

    logger.info(`[DELETE /api/sosmed/comments] Comment deleted`, {
      ip,
      userId: user.id,
      commentId: id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Comment deleted successfully!' },
      { status: 200 }
    );
  } catch (error) {
    logger.error(`[DELETE /api/sosmed/comments] Error`, {
      ip,
      userId: user?.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to delete comment' },
      { status: 500 }
    );
  }
}

// Export with auth protection
export const POST = auth(postHandler);
export const GET = auth(getHandler);
export const PUT = auth(putHandler);
export const DELETE = auth(deleteHandler);
