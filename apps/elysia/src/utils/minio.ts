/* eslint-disable @nx/enforce-module-boundaries */
import { Client } from 'minio';
import { config } from '../config';
/* eslint-enable @nx/enforce-module-boundaries */

const minioClient = new Client({
  endPoint: config.minio.endPoint,
  port: config.minio.port,
  useSSL: config.minio.useSSL,
  accessKey: config.minio.accessKey,
  secretKey: config.minio.secretKey,
});

export const ensureBucket = async (bucket: string) => {
  const exists = await minioClient.bucketExists(bucket).catch(() => false);
  if (!exists) {
    await minioClient.makeBucket(bucket, config.minio.region || 'us-east-1');
  }
};

export const buildPublicUrl = (bucket: string, objectName: string) => {
  if (config.minio.publicUrl) {
    const base = config.minio.publicUrl.replace(/\/$/, '');
    return `${base}/${bucket}/${objectName}`;
  }
  const protocol = config.minio.useSSL ? 'https' : 'http';
  const host = config.minio.endPoint;
  const port = config.minio.port ? `:${config.minio.port}` : '';
  return `${protocol}://${host}${port}/${bucket}/${objectName}`;
};

export const putObject = async (
  bucket: string,
  objectName: string,
  data: Buffer | string,
  size?: number,
  contentType?: string,
): Promise<unknown> => {
  await ensureBucket(bucket);
  return minioClient.putObject(bucket, objectName, data, size, {
    'Content-Type': contentType || 'application/octet-stream',
    'Cache-Control': 'public, max-age=31536000, immutable',
  });
};

export const deleteObject = async (bucket: string, objectName: string) => {
  try {
    await minioClient.removeObject(bucket, objectName);
  } catch {
    // ignore
  }
};
