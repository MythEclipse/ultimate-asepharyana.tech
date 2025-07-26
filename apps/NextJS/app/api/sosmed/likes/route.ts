import { NextRequest, NextResponse } from 'next/server';
import { PrismaClient } from '@asepharyana/database';
import { getAuthenticatedUser } from '@/lib/authUtils';
import logger from '@/lib/logger';

const prisma = new PrismaClient();

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
  let user;
  try {
    user = await getAuthenticatedUser();
    logger.info(`[POST /api/sosmed/likes] Request received`, { ip, userId: user?.id });

    if (!user || !user.id) {
      logger.warn(`[POST /api/sosmed/likes] Unauthorized`, { ip });
      return NextResponse.json({ message: 'Unauthorized' }, { status: 401 });
    }

    const { postId } = await req.json();
    logger.debug(`[POST /api/sosmed/likes] Payload`, { postId });

    // Check if the user has already liked the post
    const existingLike = await prisma.likes.findUnique({
      where: {
        userId_postId: {
          userId: user.id,
          postId,
        },
      },
    });

    if (existingLike) {
      logger.warn(`[POST /api/sosmed/likes] Already liked`, { ip, userId: user.id, postId });
      return NextResponse.json({ message: 'Already liked' }, { status: 409 });
    }

    // Create a new like
    const like = await prisma.likes.create({
      data: {
        postId,
        userId: user.id,
      },
    });

    logger.info(`[POST /api/sosmed/likes] Like created`, {
      ip,
      userId: user.id,
      postId,
      likeId: like.id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json({ like }, { status: 201 });
  } catch (error) {
    logger.error(`[POST /api/sosmed/likes] Error`, {
      ip,
      userId: user?.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to like post' },
      { status: 500 }
    );
  }
}

export async function DELETE(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let user;
  try {
    user = await getAuthenticatedUser();
    logger.info(`[DELETE /api/sosmed/likes] Request received`, { ip, userId: user?.id });

    if (!user || !user.id) {
      logger.warn(`[DELETE /api/sosmed/likes] Unauthorized`, { ip });
      return NextResponse.json({ message: 'Unauthorized' }, { status: 401 });
    }

    const { postId } = await req.json();
    logger.debug(`[DELETE /api/sosmed/likes] Payload`, { postId });

    // Check if the like exists and belongs to the user
    const existingLike = await prisma.likes.findUnique({
      where: {
        userId_postId: {
          userId: user.id,
          postId,
        },
      },
    });

    if (!existingLike) {
      logger.warn(`[DELETE /api/sosmed/likes] Like not found`, { ip, userId: user.id, postId });
      return NextResponse.json(
        { message: 'Like not found or does not belong to the user' },
        { status: 404 }
      );
    }

    // Delete the like
    await prisma.likes.delete({
      where: {
        userId_postId: {
          userId: user.id,
          postId,
        },
      },
    });

    logger.info(`[DELETE /api/sosmed/likes] Like removed`, {
      ip,
      userId: user.id,
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
      userId: user?.id,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to remove like' },
      { status: 500 }
    );
  }
}
