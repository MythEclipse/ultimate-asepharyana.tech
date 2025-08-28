import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '../../../../lib/prisma/service';
import logger from '../../../../utils/logger';
import { verifyJwt } from '../../../../lib/jwt';

function getIp(req: Request) {
  return (
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown'
  );
}

async function postHandler(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let userId: string | undefined;

  try {
    const token = req.cookies.get('token')?.value;
    if (!token) {
      logger.warn(`[POST /api/sosmed/likes] No token provided`, { ip });
      return NextResponse.json(
        { message: 'Authentication required' },
        { status: 401 },
      );
    }

    const decoded = await verifyJwt(token);
    if (!decoded || !decoded.userId) {
      logger.warn(`[POST /api/sosmed/likes] Invalid token or missing userId`, {
        ip,
      });
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }
    userId = decoded.userId as string;

    const { postId } = await req.json();
    logger.debug(`[POST /api/sosmed/likes] Payload`, { postId });

    const existingLike = await prisma.likes.findUnique({
      where: {
        userId_postId: {
          userId: userId,
          postId,
        },
      },
    });

    if (existingLike) {
      logger.warn(`[POST /api/sosmed/likes] Already liked`, {
        ip,
        userId: userId,
        postId,
      });
      return NextResponse.json({ message: 'Already liked' }, { status: 409 });
    }

    const like = await prisma.likes.create({
      data: {
        postId,
        userId: userId,
      },
    });

    logger.info(`[POST /api/sosmed/likes] Like created`, {
      ip,
      userId: userId,
      postId,
      durationMs: Date.now() - start,
    });

    return NextResponse.json({ like }, { status: 201 });
  } catch (error) {
    logger.error(`[POST /api/sosmed/likes] Error`, {
      ip,
      userId: userId,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to like post' },
      { status: 500 },
    );
  }
}

async function deleteHandler(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let userId: string | undefined;

  try {
    const token = req.cookies.get('token')?.value;
    if (!token) {
      logger.warn(`[DELETE /api/sosmed/likes] No token provided`, { ip });
      return NextResponse.json(
        { message: 'Authentication required' },
        { status: 401 },
      );
    }

    const decoded = await verifyJwt(token);
    if (!decoded || !decoded.userId) {
      logger.warn(
        `[DELETE /api/sosmed/likes] Invalid token or missing userId`,
        { ip },
      );
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }
    userId = decoded.userId as string;

    const { postId } = await req.json();
    logger.debug(`[DELETE /api/sosmed/likes] Payload`, { postId });

    const existingLike = await prisma.likes.findUnique({
      where: {
        userId_postId: {
          userId: userId,
          postId,
        },
      },
    });

    if (!existingLike) {
      logger.warn(`[DELETE /api/sosmed/likes] Like not found`, {
        ip,
        userId: userId,
        postId,
      });
      return NextResponse.json({ message: 'Like not found' }, { status: 404 });
    }

    await prisma.likes.delete({
      where: {
        userId_postId: {
          userId: userId,
          postId,
        },
      },
    });

    logger.info(`[DELETE /api/sosmed/likes] Like removed`, {
      ip,
      userId: userId,
      postId,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Like removed successfully' },
      { status: 200 },
    );
  } catch (error) {
    logger.error(`[DELETE /api/sosmed/likes] Error`, {
      ip,
      userId: userId,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to remove like' },
      { status: 500 },
    );
  }
}

export const POST = postHandler;
export const DELETE = deleteHandler;
