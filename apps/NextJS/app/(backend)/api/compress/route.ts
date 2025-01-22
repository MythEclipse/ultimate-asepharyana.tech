// app/api/compress/route.ts
import { NextRequest, NextResponse } from 'next/server';
import sharp from 'sharp';
import axios from 'axios';
import path from 'path';
import ffmpeg from 'fluent-ffmpeg';
import fs from 'fs';
import { tmpdir } from 'os';
import { join } from 'path';

// Queue configuration
let isProcessing = false;
const queue: Array<{ task: () => Promise<Response>; resolve: (value: Response) => void }> = [];
const MAX_QUEUE_SIZE = 10;
const CACHE_DURATION = 600; // 10 menit dalam detik

async function processNext() {
  if (isProcessing || queue.length === 0) return;
  
  isProcessing = true;
  const nextJob = queue.shift();
  
  try {
    if (nextJob) {
      const result = await nextJob.task();
      nextJob.resolve(result);
    }
  } finally {
    isProcessing = false;
    processNext();
  }
}

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url);
  const url = searchParams.get('url');
  const size = searchParams.get('size');

  if (!url || !size) {
    return NextResponse.json(
      { error: 'Parameter url dan size diperlukan' },
      { status: 400 }
    );
  }

  if (queue.length >= MAX_QUEUE_SIZE) {
    return NextResponse.json(
      { error: 'Server sibuk, coba lagi nanti' },
      { status: 429 }
    );
  }

  return new Promise<Response>(resolve => {
    queue.push({
      task: async () => {
        try {
          const response = await axios.get(url, {
            responseType: 'arraybuffer',
            headers: { 'User-Agent': 'Mozilla/5.0' },
            timeout: 30000,
            maxContentLength: 100 * 1024 * 1024,
          });

          const buffer = Buffer.from(response.data);
          const ext = path.extname(url).toLowerCase();
          const sizeValue = parseFloat(size);
          const isPercentage = size.endsWith('%');

          // Dynamic compression based on size parameter
          const compressionProfile = {
            crf: isPercentage ? 28 + (100 - sizeValue)/2 : 28,
            videoBitrate: isPercentage ? `${1000 + (100 - sizeValue)*10}k` : '1500k',
            audioBitrate: '64k',
            scaleHeight: isPercentage ? 720 - (100 - sizeValue)*5 : 720
          };

          let result: Buffer;
          let contentType: string;

          if (['.jpg', '.jpeg', '.png'].includes(ext)) {
            // Image processing
            result = await sharp(buffer, { sequentialRead: true })
              .resize(800, 800, { fit: 'inside', withoutEnlargement: true })
              .jpeg({ 
                quality: Math.min(isPercentage ? sizeValue : 80, 100),
                mozjpeg: true,
                progressive: true 
              })
              .toBuffer();
            contentType = 'image/jpeg';
          } else if (['.mp4', '.mov', '.avi'].includes(ext)) {
            // Video processing with dynamic compression
            result = await new Promise<Buffer>((resolve, reject) => {
              const inputPath = join(tmpdir(), `vid_in_${Date.now()}${ext}`);
              const outputPath = join(tmpdir(), `vid_out_${Date.now()}.mp4`);

              fs.writeFileSync(inputPath, buffer);

              ffmpeg(inputPath)
                .videoCodec('libx264')
                .audioCodec('aac')
                .outputOptions([
                  `-crf ${compressionProfile.crf}`,
                  `-b:v ${compressionProfile.videoBitrate}`,
                  `-b:a ${compressionProfile.audioBitrate}`,
                  `-vf scale=-2:${compressionProfile.scaleHeight}`,
                  '-preset medium',
                  '-threads 1',
                  '-movflags +faststart',
                  '-pix_fmt yuv420p'
                ])
                .on('end', () => {
                  const outputBuffer = fs.readFileSync(outputPath);
                  fs.unlinkSync(inputPath);
                  fs.unlinkSync(outputPath);
                  resolve(outputBuffer);
                })
                .on('error', reject)
                .save(outputPath);
            });
            contentType = 'video/mp4';
          } else {
            throw new Error('Format tidak didukung');
          }

          return new Response(result, {
            headers: {
              'Content-Type': contentType,
              'Cache-Control': `public, max-age=${CACHE_DURATION}`,
              'Content-Length': result.length.toString()
            }
          });
        } catch (error) {
          console.error('Error processing:', error);
          return NextResponse.json(
            { error: 'Gagal memproses konten' },
            { status: 500 }
          );
        }
      },
      resolve
    });

    if (!isProcessing) processNext();
  });
}