import { auth } from '@/auth';
import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma/service';
import logger from '@/lib/logger';

function getIp(req: NextRequest) {
  return (
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown'
  );
}

export const POST = auth(async function POST(req) {
  const session = await auth()
  const start = Date.now();
  const ip = getIp(req);

   if (!req.auth) {
    logger.warn(`[POST /api/sosmed/likes] Unauthorized`, { ip });
    return NextResponse.json({ message: 'Unauthorized' }, { status: 401 });
  }

  try {
    

    const { postId } = await req.json();
    logger.debug(`[POST /api/sosmed/likes] Payload`, { postId });

    if (!session?.user?.id) {
      logger.warn(`[POST /api/sosmed/likes] User ID missing in session`, { ip });
      return NextResponse.json({ message: 'User ID missing' }, { status: 400 });
    }

    const existingLike = await prisma.likes.findUnique({
      where: {
        userId_postId: {
          userId: session.user.id,
          postId,
        },
      },
    });

    if (existingLike) {
      logger.warn(`[POST /api/sosmed/likes] Already liked`, { ip, userId: session?.user.id, postId });
      return NextResponse.json({ message: 'Already liked' }, { status: 409 });
    }

    const like = await prisma.likes.create({
      data: {
        postId,
        userId: session?.user.id,
      },
    });

    logger.info(`[POST /api/sosmed/likes] Like created`, {
      ip,
      userId: session?.user.id,
      postId,
      durationMs: Date.now() - start,
    });

    return NextResponse.json({ like }, { status: 201 });
  } catch (error) {
    logger.error(`[POST /api/sosmed/likes] Error`, {
      ip,
      userId: session?.user?.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to like post' },
      { status: 500 }
    );
  }
});

export const DELETE = auth(async function DELETE(req) {
  const session = await auth()
  const start = Date.now();
  const ip = getIp(req);

 if (!req.auth) {
    logger.warn(`[DELETE /api/sosmed/likes] Unauthorized`, { ip });
    return NextResponse.json({ message: 'Unauthorized' }, { status: 401 });
  }

  try {
    

    const { postId } = await req.json();
    logger.debug(`[DELETE /api/sosmed/likes] Payload`, { postId });

    if (!session?.user?.id) {
      logger.warn(`[DELETE /api/sosmed/likes] User ID missing in session`, { ip });
      return NextResponse.json({ message: 'User ID missing' }, { status: 400 });
    }

    const existingLike = await prisma.likes.findUnique({
      where: {
        userId_postId: {
          userId: session.user.id,
          postId,
        },
      },
    });

    if (!existingLike) {
      logger.warn(`[DELETE /api/sosmed/likes] Like not found`, { ip, userId: session?.user.id, postId });
      return NextResponse.json(
        { message: 'Like not found' },
        { status: 404 }
      );
    }

    await prisma.likes.delete({
      where: {
        userId_postId: {
          userId: session.user.id,
          postId,
        },
      },
    });

    logger.info(`[DELETE /api/sosmed/likes] Like removed`, {
      ip,
      userId: session?.user.id,
      postId,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Like removed successfully' },
      { status: 200 }
    );
  } catch (error) {
    logger.error(`[DELETE /api/sosmed/likes] Error`, {
      ip,
      userId: session?.user?.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to remove like' },
      { status: 500 }
    );
  }
});
