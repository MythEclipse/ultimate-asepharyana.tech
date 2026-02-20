import { Elysia, t } from 'elysia';
import { authMiddleware } from '../middleware/auth';
import { config } from '../config';
import { putObject, buildPublicUrl } from '../utils/minio';
import { getDb, users, eq } from '../services';

const MAX_SIZE_BYTES = 5 * 1024 * 1024; // 5MB
const ALLOWED_TYPES = new Set(['image/jpeg', 'image/png', 'image/webp']);

export const userAvatarRoutes = new Elysia({ prefix: '/api/users' })
  .use(authMiddleware)
  .post(
    '/avatar',
    async (ctx) => {
      const { body, set } = ctx;
      const { user } = ctx as unknown as {
        user: { id: string; email?: string | null; name?: string | null };
      };
      // body is FormData when content-type is multipart/form-data
      const form = body as unknown as FormData;
      const fileEntry = form?.get('avatar') || form?.get('file');

      if (!fileEntry || !(fileEntry instanceof File)) {
        set.status = 400;
        return {
          success: false,
          error: 'Avatar file is required (field: avatar)',
        };
      }

      const file = fileEntry as File;

      if (!ALLOWED_TYPES.has(file.type)) {
        set.status = 400;
        return {
          success: false,
          error: 'Unsupported file type. Use JPEG, PNG, or WEBP.',
        };
      }

      if (file.size > MAX_SIZE_BYTES) {
        set.status = 400;
        return { success: false, error: 'File too large. Max size is 5MB.' };
      }

      const name = file.name || 'avatar';
      const ext =
        (name.includes('.') ? name.split('.').pop() : undefined) ||
        (file.type === 'image/png'
          ? 'png'
          : file.type === 'image/webp'
            ? 'webp'
            : 'jpg');

      const objectName = `${config.minio.avatarPrefix}/${user.id}/${Date.now()}.${ext}`;

      const arrayBuffer = await file.arrayBuffer();
      const buffer = Buffer.from(arrayBuffer);

      await putObject(
        config.minio.bucket,
        objectName,
        buffer,
        buffer.length,
        file.type,
      );
      const url = buildPublicUrl(config.minio.bucket, objectName);

      const db = getDb();
      await db.update(users).set({ image: url }).where(eq(users.id, user.id));

      return {
        success: true,
        url,
      };
    },
    {
      detail: {
        tags: ['API'],
        summary: 'Upload user avatar (MinIO)',
        description:
          "Upload an avatar image and store it to MinIO. Updates the authenticated user's profile image URL.",
      },
      type: 'multipart/form-data',
      body: t.Object({
        avatar: t.File({ description: 'Avatar image (JPEG, PNG, WEBP)' }),
      }),
      response: {
        200: t.Object({ success: t.Boolean(), url: t.String() }),
        400: t.Object({ success: t.Boolean(), error: t.String() }),
        401: t.Object({ success: t.Boolean(), error: t.String() }),
      },
    },
  );
