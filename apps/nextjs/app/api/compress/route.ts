import { NextRequest, NextResponse } from 'next/server';
import sharp from 'sharp';
import axios from 'axios';
import path from 'path';
import ffmpeg from 'fluent-ffmpeg';
import fs from 'fs';
import { tmpdir } from 'os';
import { join } from 'path';
import crypto from 'crypto';
import { ryzenCDN } from '../../../lib/ryzencdn';
import { corsHeaders } from '../../../lib/corsHeaders';

const CACHE_DIR = join(tmpdir(), 'compress-cache');
const CACHE_EXPIRY = 3600 * 1000;
const MAX_QUEUE_SIZE = 10;
const API_DISABLED = true;

if (!fs.existsSync(CACHE_DIR)) fs.mkdirSync(CACHE_DIR, { recursive: true });

let isProcessing = false;
const queue: Array<{
  task: () => Promise<NextResponse>;
  resolve: (value: NextResponse) => void;
}> = [];

const generateCacheKey = (url: string, sizeParam: string): string => {
  return (
    crypto
      .createHash('sha1')
      .update(url + sizeParam)
      .digest('hex') + '.cache'
  );
};

const processNext = async () => {
  if (isProcessing || queue.length === 0) return;
  isProcessing = true;
  const nextJob = queue.shift();
  if (nextJob) nextJob.resolve(await nextJob.task());
  isProcessing = false;
  processNext();
};

const getVideoMetadata = (
  inputPath: string
): Promise<{ duration: number; width: number; height: number }> => {
  return new Promise((resolve, reject) => {
    ffmpeg.ffprobe(inputPath, (err, metadata) => {
      if (err) return reject(err);
      const videoStream = metadata.streams.find(
        (s) => s.codec_type === 'video'
      );
      resolve({
        duration: metadata.format.duration || 1,
        width: videoStream?.width || 1280,
        height: videoStream?.height || 720,
      });
    });
  });
};

const compressImage = async (
  buffer: Buffer,
  targetKB: number,
  cacheKey: string
): Promise<{ buffer: Buffer; sizeReduction: number }> => {
  const cachePath = join(CACHE_DIR, cacheKey);
  if (
    fs.existsSync(cachePath) &&
    Date.now() - fs.statSync(cachePath).mtimeMs < CACHE_EXPIRY
  ) {
    const cachedBuffer = fs.readFileSync(cachePath);
    return {
      buffer: cachedBuffer,
      sizeReduction:
        ((buffer.length - cachedBuffer.length) / buffer.length) * 100,
    };
  }
  let quality = 85,
    bestBuffer = buffer;
  for (let i = 0; i < 8; i++) {
    const compressed = await sharp(buffer)
      .jpeg({ quality, mozjpeg: true, progressive: true })
      .toBuffer();
    const sizeKB = compressed.length / 1024;
    if (sizeKB > targetKB * 1.05) quality -= 5;
    else if (sizeKB < targetKB * 0.95) {
      quality += 5;
      bestBuffer = compressed;
    } else {
      fs.writeFileSync(cachePath, compressed as Uint8Array);
      return {
        buffer: compressed,
        sizeReduction:
          ((buffer.length - compressed.length) / buffer.length) * 100,
      };
    }
  }
  fs.writeFileSync(cachePath, bestBuffer as Uint8Array);
  return {
    buffer: bestBuffer,
    sizeReduction: ((buffer.length - bestBuffer.length) / buffer.length) * 100,
  };
};

const compressVideo = async (
  buffer: Buffer,
  targetMB: number,
  isPercentage: boolean,
  originalMB: number,
  cacheKey: string
): Promise<{ buffer: Buffer; sizeReduction: number }> => {
  const cachePath = join(CACHE_DIR, cacheKey);
  if (
    fs.existsSync(cachePath) &&
    Date.now() - fs.statSync(cachePath).mtimeMs < CACHE_EXPIRY
  ) {
    const cachedBuffer = fs.readFileSync(cachePath);
    return {
      buffer: cachedBuffer,
      sizeReduction:
        ((buffer.length - cachedBuffer.length) / buffer.length) * 100,
    };
  }
  const tempDir = tmpdir(),
    inputPath = join(tempDir, `vid_in_${Date.now()}.mp4`),
    outputPath = join(tempDir, `vid_out_${Date.now()}.mp4`);
  fs.writeFileSync(inputPath, buffer as Uint8Array);
  try {
    const metadata = await getVideoMetadata(inputPath);
    let finalTargetMB = isPercentage ? (originalMB * targetMB) / 100 : targetMB;
    let resultBuffer: Buffer;
    let attempts = 0;
    const minSizeMB = finalTargetMB - 3.5,
      maxSizeMB = finalTargetMB + 0.5;
    do {
      const ratio = Math.max(0.6, finalTargetMB / originalMB);
      let targetHeight = Math.max(
        360,
        Math.min(
          metadata.height,
          Math.round(metadata.height * Math.pow(ratio, 0.8))
        )
      );
      targetHeight -= targetHeight % 2;
      let targetWidth = Math.round(
        targetHeight * (metadata.width / metadata.height)
      );
      targetWidth -= targetWidth % 2;
      const crf = Math.min(
        32,
        Math.max(18, 24 - (originalMB - finalTargetMB) * 0.5)
      );
      await new Promise<void>((resolve, reject) => {
        ffmpeg(inputPath)
          .videoCodec('libx264')
          .audioCodec('aac')
          .outputOptions([
            `-vf scale=${targetWidth}:${targetHeight}`,
            `-crf ${crf}`,
            '-preset medium',
            '-movflags +faststart',
            '-pix_fmt yuv420p',
            '-y',
          ])
          .on('end', () => resolve())
          .on('error', reject)
          .save(outputPath);
      });
      resultBuffer = fs.readFileSync(outputPath);
      const actualMB = resultBuffer.length / 1024 ** 2;
      if (actualMB < minSizeMB) finalTargetMB *= 1.2;
      else if (actualMB > maxSizeMB) finalTargetMB *= 0.8;
      else break;
      attempts++;
    } while (attempts < 5);
    fs.writeFileSync(cachePath, resultBuffer as Uint8Array);
    return {
      buffer: resultBuffer,
      sizeReduction:
        ((buffer.length - resultBuffer.length) / buffer.length) * 100,
    };
  } finally {
    [inputPath, outputPath].forEach(
      (p) => fs.existsSync(p) && fs.unlinkSync(p)
    );
  }
};

export async function GET(req: NextRequest): Promise<NextResponse> {
  if (API_DISABLED) {
    return NextResponse.json(
      { error: 'API is currently disabled' },
      { status: 503, headers: corsHeaders }
    );
  }

  const { searchParams } = new URL(req.url);
  const url = searchParams.get('url');
  const sizeParam = searchParams.get('size');
  if (!url || !sizeParam)
    return NextResponse.json(
      { error: 'Parameter url dan size diperlukan' },
      { status: 400, headers: corsHeaders }
    );
  if (queue.length >= MAX_QUEUE_SIZE)
    return NextResponse.json(
      { error: 'Server sibuk, coba lagi nanti' },
      { status: 429, headers: corsHeaders }
    );
  return new Promise<NextResponse>((resolve) => {
    queue.push({
      task: async () => {
        try {
          const cacheKey = generateCacheKey(url, sizeParam);
          const response = await axios.get(url, {
            responseType: 'arraybuffer',
            headers: { 'User-Agent': 'Mozilla/5.0' },
            timeout: 45000,
          });
          const buffer = Buffer.from(response.data);
          const ext = path.extname(url).toLowerCase();
          const isPercentage = sizeParam.includes('%');
          const sizeValue = parseFloat(sizeParam.replace('%', ''));
          if (['.jpg', '.jpeg', '.png'].includes(ext)) {
            const result = await compressImage(
              buffer,
              isPercentage
                ? ((buffer.length / 1024) * sizeValue) / 100
                : sizeValue * 1024,
              cacheKey
            );
            const cdnLink = await ryzenCDN(result);
            return NextResponse.json(
              { link: cdnLink },
              { status: 200, headers: corsHeaders }
            );
          }
          if (['.mp4', '.mov', '.avi'].includes(ext)) {
            const result = await compressVideo(
              buffer,
              sizeValue,
              isPercentage,
              buffer.length / 1024 ** 2,
              cacheKey
            );
            const cdnLink = await ryzenCDN(result);
            return NextResponse.json(
              { link: cdnLink },
              { status: 200, headers: corsHeaders }
            );
          }
          return NextResponse.json(
            { error: 'Format tidak didukung' },
            { status: 400, headers: corsHeaders }
          );
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        } catch (error) {
          return NextResponse.json(
            { error: 'Kompresi gagal' },
            { status: 500, headers: corsHeaders }
          );
        }
      },
      resolve,
    });
    if (!isProcessing) processNext();
  });
}
