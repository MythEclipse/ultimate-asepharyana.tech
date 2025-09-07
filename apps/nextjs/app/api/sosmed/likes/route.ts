import { NextRequest, NextResponse } from 'next/server';
import logger from '../../../../utils/logger';
import { getDb, Likes } from '@asepharyana/services';

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
  const db = getDb();

  try {
    const { postId } = await req.json();
    logger.debug(`[POST /api/sosmed/likes] Payload`, { postId });

    // Use IP as anonymous user identifier
    const anonymousUserId = `anonymous_${ip.replace(/\./g, '_')}`;

    const existingLike = (await db
      .selectFrom('Likes')
      .selectAll()
      .where('userId', '=', anonymousUserId)
      .where('postId', '=', postId)
      .executeTakeFirst()) as Likes | undefined;

    if (existingLike) {
      logger.warn(`[POST /api/sosmed/likes] Already liked`, {
        ip,
        userId: anonymousUserId,
        postId,
      });
      return NextResponse.json({ message: 'Already liked' }, { status: 409 });
    }

    const like = (await db
      .insertInto('Likes')
      .values({
        postId,
        userId: anonymousUserId,
      })
      .returningAll()
      .executeTakeFirstOrThrow()) as Likes;

    logger.info(`[POST /api/sosmed/likes] Like created`, {
      ip,
      userId: anonymousUserId,
      postId,
      durationMs: Date.now() - start,
    });

    return NextResponse.json({ like }, { status: 201 });
  } catch (error) {
    logger.error(`[POST /api/sosmed/likes] Error`, {
      ip,
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
  const db = getDb();

  try {
    const { postId } = await req.json();
    logger.debug(`[DELETE /api/sosmed/likes] Payload`, { postId });

    // Use IP as anonymous user identifier
    const anonymousUserId = `anonymous_${ip.replace(/\./g, '_')}`;

    const existingLike = (await db
      .selectFrom('Likes')
      .selectAll()
      .where('userId', '=', anonymousUserId)
      .where('postId', '=', postId)
      .executeTakeFirst()) as Likes | undefined;

    if (!existingLike) {
      logger.warn(`[DELETE /api/sosmed/likes] Like not found`, {
        ip,
        userId: anonymousUserId,
        postId,
      });
      return NextResponse.json({ message: 'Like not found' }, { status: 404 });
    }

    await db
      .deleteFrom('Likes')
      .where('userId', '=', anonymousUserId)
      .where('postId', '=', postId)
      .execute();

    logger.info(`[DELETE /api/sosmed/likes] Like removed`, {
      ip,
      userId: anonymousUserId,
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
