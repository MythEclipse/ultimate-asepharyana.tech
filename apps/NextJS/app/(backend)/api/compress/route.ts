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
const queue: (() => Promise<void>)[] = [];
const MAX_QUEUE_SIZE = 10;

async function processNext() {
  if (isProcessing || queue.length === 0) return;
  
  isProcessing = true;
  try {
    await queue.shift()!();
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
      { error: 'Missing parameters' },
      { status: 400 }
    );
  }

  if (queue.length >= MAX_QUEUE_SIZE) {
    return NextResponse.json(
      { error: 'Server busy. Try again later' },
      { status: 429 }
    );
  }

  return new Promise<Response>(resolve => {
    queue.push(async () => {
      try {
        const response = await axios.get(url, {
          responseType: 'arraybuffer',
          headers: { 'User-Agent': 'Mozilla/5.0' },
          timeout: 20000,
          maxContentLength: 50 * 1024 * 1024,
        });

        const buffer = Buffer.from(response.data);
        const ext = path.extname(url).toLowerCase();
        let result: Buffer;

        if (['.jpg', '.jpeg', '.png'].includes(ext)) {
          // Image compression
          result = await sharp(buffer, { sequentialRead: true })
            .resize(800, null, { withoutEnlargement: true })
            .jpeg({ 
              quality: Math.min(parseInt(size) || 80, 100),
              mozjpeg: true,
              force: false 
            })
            .toBuffer();
        } else if (['.mp4', '.mov', '.avi', '.mkv'].includes(ext)) {
          // Video compression with aggressive settings
          result = await new Promise<Buffer>((resolve, reject) => {
            const input = join(tmpdir(), `in_${Date.now()}${ext}`);
            const output = join(tmpdir(), `out_${Date.now()}.mp4`);
            
            fs.writeFileSync(input, buffer);
            
            ffmpeg(input)
              .videoCodec('libx264')
              .outputOptions([
                '-preset medium',
                '-crf 32',
                '-threads 1',
                '-tune stillimage',
                '-vf scale=-2:480', // Downscale to 480p
                '-b:v 800k', // Target bitrate
                '-maxrate 1000k', // Maximum bitrate
                '-bufsize 2000k', // Buffer size
                '-movflags +faststart',
                '-pix_fmt yuv420p',
                '-an' // Remove audio
              ])
              .on('start', (cmdline) => console.log('FFmpeg command:', cmdline))
              .on('progress', (progress) => console.log('Processing:', progress.timemark))
              .on('end', () => {
                const outputBuffer = fs.readFileSync(output);
                fs.unlinkSync(input);
                fs.unlinkSync(output);
                resolve(outputBuffer);
              })
              .on('error', (err) => {
                fs.unlinkSync(input);
                if (fs.existsSync(output)) fs.unlinkSync(output);
                reject(err);
              })
              .save(output);
          });
        } else {
          throw new Error('Unsupported format');
        }

        resolve(new Response(result, {
          headers: {
            'Content-Type': ext.startsWith('.mp4') ? 'video/mp4' : 'image/jpeg',
            'Cache-Control': 'public, max-age=86400',
            'Content-Length': result.length.toString()
          }
        }));
      } catch (error) {
        console.error('Processing error:', error);
        resolve(NextResponse.json(
          { error: 'Processing failed' },
          { status: 500 }
        ));
      }
    });

    if (!isProcessing) processNext();
  });
}