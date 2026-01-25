// Notifications System Handlers
import { wsManager } from '../ws-manager';
import type {
  WSMessage,
  NotificationListSyncPayload,
  NotificationListDataPayload,
  NotificationMarkReadPayload,
  NotificationMarkAllReadPayload,
  NotificationDeletePayload,
  NotificationReceivedPayload,
} from '../types';
import { getDb, quizNotifications, eq, and, desc } from '@asepharyana/services';

// Handler: Get notification list
export async function handleNotificationListSync(
  ws: any,
  data: WSMessage<NotificationListSyncPayload>,
) {
  const { userId, limit = 50, offset = 0, unreadOnly = false } = data.payload;
  const db = getDb();

  // Build query conditions
  const conditions = unreadOnly
    ? and(eq(quizNotifications.userId, userId), eq(quizNotifications.isRead, 0))
    : eq(quizNotifications.userId, userId);

  // Get notifications
  const notifs = await db
    .select()
    .from(quizNotifications)
    .where(conditions)
    .orderBy(desc(quizNotifications.createdAt))
    .limit(limit)
    .offset(offset);

  // Get total and unread counts
  const totalCount = notifs.length;

  const [unreadResult] = await db
    .select()
    .from(quizNotifications)
    .where(
      and(
        eq(quizNotifications.userId, userId),
        eq(quizNotifications.isRead, 0),
      ),
    );
  const unreadCount = unreadResult ? 1 : 0;

  // Map to payload format
  const notifications: NotificationReceivedPayload[] = notifs.map((n) => ({
    notificationId: n.id,
    notificationType: n.type as any,
    title: n.title,
    message: n.message,
    data: n.data ? JSON.parse(n.data) : undefined,
    priority: n.priority as any,
    timestamp: n.createdAt.getTime(),
    expiresAt: n.expiresAt?.getTime(),
  }));

  const response: NotificationListDataPayload = {
    notifications,
    totalCount,
    unreadCount,
    hasMore: offset + limit < totalCount,
  };

  wsManager.sendToSession(ws, 'notification.list.data', response);
}

// Handler: Mark notifications as read
export async function handleNotificationMarkRead(
  ws: any,
  data: WSMessage<NotificationMarkReadPayload>,
) {
  const { userId, notificationIds } = data.payload;
  const db = getDb();

  // Update notifications
  for (const notifId of notificationIds) {
    await db
      .update(quizNotifications)
      .set({ isRead: 1 })
      .where(
        and(
          eq(quizNotifications.id, notifId),
          eq(quizNotifications.userId, userId),
        ),
      );
  }

  wsManager.sendToSession(ws, 'notification.mark.read.success', {
    notificationIds,
    timestamp: new Date().toISOString(),
  });
}

// Handler: Mark all notifications as read
export async function handleNotificationMarkAllRead(
  ws: any,
  data: WSMessage<NotificationMarkAllReadPayload>,
) {
  const { userId } = data.payload;
  const db = getDb();

  await db
    .update(quizNotifications)
    .set({ isRead: 1 })
    .where(eq(quizNotifications.userId, userId));

  wsManager.sendToSession(ws, 'notification.mark.all.read.success', {
    userId,
    timestamp: new Date().toISOString(),
  });
}

// Handler: Delete notification
export async function handleNotificationDelete(
  ws: any,
  data: WSMessage<NotificationDeletePayload>,
) {
  const { userId, notificationId } = data.payload;
  const db = getDb();

  await db
    .delete(quizNotifications)
    .where(
      and(
        eq(quizNotifications.id, notificationId),
        eq(quizNotifications.userId, userId),
      ),
    );

  wsManager.sendToSession(ws, 'notification.delete.success', {
    notificationId,
    timestamp: new Date().toISOString(),
  });
}

// Helper: Send notification to user
export async function sendNotificationToUser(
  userId: string,
  type: string,
  title: string,
  message: string,
  data?: Record<string, any>,
  priority: string = 'medium',
) {
  const db = getDb();
  const notificationId = `notif_${Date.now()}_${userId}`;

  await db.insert(quizNotifications).values({
    id: notificationId,
    userId,
    type,
    title,
    message,
    data: data ? JSON.stringify(data) : null,
    isRead: 0,
    priority,
    createdAt: new Date(),
    expiresAt: null,
  });

  const notifPayload: NotificationReceivedPayload = {
    notificationId,
    notificationType: type as any,
    title,
    message,
    data,
    priority: priority as any,
    timestamp: Date.now(),
  };

  const session = wsManager.findSessionByUserId(userId);
  if (session) {
    wsManager.sendToSession(session, 'notification.received', notifPayload);
  }
}
