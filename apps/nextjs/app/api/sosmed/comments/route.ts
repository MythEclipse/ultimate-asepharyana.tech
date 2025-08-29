import { NextRequest, NextResponse } from 'next/server';
import logger from '../../../../utils/logger';
import { verifyJwt } from '../../../../lib/jwt';
import { getDb, Comments, User } from '@asepharyana/services';

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
  let userId: string | undefined;
  const db = getDb();
  try {
    const token = req.cookies.get('token')?.value;
    if (!token) {
      logger.warn(`[POST /api/sosmed/comments] No token provided`, { ip });
      return NextResponse.json(
        { message: 'Authentication required' },
        { status: 401 },
      );
    }

    const decoded = await verifyJwt(token);
    if (!decoded || !decoded.userId) {
      logger.warn(
        `[POST /api/sosmed/comments] Invalid token or missing userId`,
        { ip },
      );
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }
    userId = decoded.userId as string;

    const { postId, content } = await req.json();
    logger.debug(`[POST /api/sosmed/comments] Payload`, { postId, content });

    if (!content) {
      logger.warn(`[POST /api/sosmed/comments] Content required`, { ip });
      return NextResponse.json(
        { message: 'Content is required' },
        { status: 400 },
      );
    }

    const comment = (await db
      .insertInto('Comments')
      .values({
        postId,
        content,
        userId: userId,
        authorId: userId,
      })
      .returningAll()
      .executeTakeFirstOrThrow()) as Comments;

    logger.info(`[POST /api/sosmed/comments] Comment created`, {
      ip,
      userId: userId,
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
      { status: 201 },
    );
  } catch (error) {
    logger.error(`[POST /api/sosmed/comments] Error`, {
      ip,
      userId: userId,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to add comment' },
      { status: 500 },
    );
  }
}

async function getHandler(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  const db = getDb();
  try {
    const { searchParams } = new URL(req.url);
    const postId = searchParams.get('postId');

    if (!postId) {
      logger.warn(`[GET /api/sosmed/comments] Post ID required`, { ip });
      return NextResponse.json(
        { message: 'Post ID is required' },
        { status: 400 },
      );
    }

    const comments = await db
      .selectFrom('Comments')
      .selectAll()
      .where('Comments.postId', '=', postId)
      .leftJoin('User', 'User.id', 'Comments.userId')
      .select([
        'User.id as user_id',
        'User.name as user_name',
        'User.image as user_image',
      ])
      .orderBy('Comments.created_at', 'desc')
      .execute();

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
      { status: 500 },
    );
  }
}

async function putHandler(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let userId: string | undefined;
  const db = getDb();
  try {
    const token = req.cookies.get('token')?.value;
    if (!token) {
      logger.warn(`[PUT /api/sosmed/comments] No token provided`, { ip });
      return NextResponse.json(
        { message: 'Authentication required' },
        { status: 401 },
      );
    }

    const decoded = await verifyJwt(token);
    if (!decoded || !decoded.userId) {
      logger.warn(
        `[PUT /api/sosmed/comments] Invalid token or missing userId`,
        { ip },
      );
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }
    userId = decoded.userId as string;

    const { id, content } = await req.json();
    logger.debug(`[PUT /api/sosmed/comments] Payload`, { id, content });

    if (!id || !content) {
      logger.warn(`[PUT /api/sosmed/comments] ID/content required`, { ip });
      return NextResponse.json(
        { message: 'Comment ID and content are required' },
        { status: 400 },
      );
    }

    const comment = (await db
      .selectFrom('Comments')
      .selectAll()
      .where('id', '=', id)
      .executeTakeFirst()) as Comments | undefined;

    if (!comment || comment.userId !== userId) {
      logger.warn(`[PUT /api/sosmed/comments] Not authorized to edit`, {
        ip,
        userId: userId,
        commentId: id,
      });
      return NextResponse.json(
        { message: 'User not authorized to edit this comment' },
        { status: 403 },
      );
    }

    const updatedComment = (await db
      .updateTable('Comments')
      .set({
        content: `${content} -edited`,
      })
      .where('id', '=', id)
      .returningAll()
      .executeTakeFirstOrThrow()) as Comments;

    logger.info(`[PUT /api/sosmed/comments] Comment updated`, {
      ip,
      userId: userId,
      commentId: id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Comment updated successfully!', comment: updatedComment },
      { status: 200 },
    );
  } catch (error) {
    logger.error(`[PUT /api/sosmed/comments] Error`, {
      ip,
      userId: userId,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to update comment' },
      { status: 500 },
    );
  }
}

async function deleteHandler(req: NextRequest) {
  const start = Date.now();
  const ip = getIp(req);
  let userId: string | undefined;
  const db = getDb();
  try {
    const token = req.cookies.get('token')?.value;
    if (!token) {
      logger.warn(`[DELETE /api/sosmed/comments] No token provided`, { ip });
      return NextResponse.json(
        { message: 'Authentication required' },
        { status: 401 },
      );
    }

    const decoded = await verifyJwt(token);
    if (!decoded || !decoded.userId) {
      logger.warn(
        `[DELETE /api/sosmed/comments] Invalid token or missing userId`,
        { ip },
      );
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }
    userId = decoded.userId as string;

    const { id } = await req.json();
    logger.debug(`[DELETE /api/sosmed/comments] Payload`, { id });

    if (!id) {
      logger.warn(`[DELETE /api/sosmed/comments] ID required`, { ip });
      return NextResponse.json(
        { message: 'Comment ID is required' },
        { status: 400 },
      );
    }

    const comment = (await db
      .selectFrom('Comments')
      .selectAll()
      .where('id', '=', id)
      .executeTakeFirst()) as Comments | undefined;

    if (!comment || comment.userId !== userId) {
      logger.warn(`[DELETE /api/sosmed/comments] Not authorized to delete`, {
        ip,
        userId: userId,
        commentId: id,
      });
      return NextResponse.json(
        { message: 'User not authorized to delete this comment' },
        { status: 403 },
      );
    }

    await db.deleteFrom('Comments').where('id', '=', id).execute();

    logger.info(`[DELETE /api/sosmed/comments] Comment deleted`, {
      ip,
      userId: userId,
      commentId: id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json(
      { message: 'Comment deleted successfully!' },
      { status: 200 },
    );
  } catch (error) {
    logger.error(`[DELETE /api/sosmed/comments] Error`, {
      ip,
      userId: userId,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json(
      { message: 'Failed to delete comment' },
      { status: 500 },
    );
  }
}

// Export with auth protection
export const POST = postHandler;
export const GET = getHandler;
export const PUT = putHandler;
export const DELETE = deleteHandler;
