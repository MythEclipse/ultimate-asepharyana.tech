import { NextRequest, NextResponse } from 'next/server';
import sharp from 'sharp';
import axios from 'axios';
import path from 'path';
import ffmpeg from 'fluent-ffmpeg';
import fs from 'fs';
import { tmpdir } from 'os';
import { join } from 'path';
import crypto from 'crypto';
import fetch from 'node-fetch';
import FormData from 'form-data';
import { fileTypeFromBuffer } from 'file-type';

// Queue configuration
let isProcessing = false;
const queue: Array<{ task: () => Promise<Response>; resolve: (value: Response) => void }> = [];
const MAX_QUEUE_SIZE = 10;
// const CACHE_DURATION = 600;

// Cache configuration
const CACHE_DIR = join(tmpdir(), 'compress-cache');
const CACHE_EXPIRY = 3600 * 1000; // 1 jam

// Inisialisasi cache directory
if (!fs.existsSync(CACHE_DIR)) {
  fs.mkdirSync(CACHE_DIR, { recursive: true });
}

// Helper functions
function generateCacheKey(url: string, sizeParam: string): string {
  const hash = crypto.createHash('sha1');
  hash.update(url + sizeParam);
  return hash.digest('hex') + '.cache';
}

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

async function getVideoMetadata(inputPath: string) {
  return new Promise<{ duration: number; width: number; height: number }>((resolve, reject) => {
    ffmpeg.ffprobe(inputPath, (err, metadata) => {
      if (err) return reject(err);
      const videoStream = metadata.streams.find(s => s.codec_type === 'video');
      resolve({
        duration: metadata.format.duration || 1,
        width: videoStream?.width || 1280,
        height: videoStream?.height || 720
      });
    });
  });
}

async function compressImage(buffer: Buffer, targetKB: number, cacheKey: string): Promise<Buffer> {
  const cachePath = join(CACHE_DIR, cacheKey);
  
  // Cek cache
  if (fs.existsSync(cachePath)) {
    const stats = fs.statSync(cachePath);
    if (Date.now() - stats.mtimeMs < CACHE_EXPIRY) {
      return fs.readFileSync(cachePath);
    }
  }

  let low = 1;
  let high = 100;
  let quality = 85;
  let bestBuffer = buffer;

  for (let i = 0; i < 8; i++) {
    const compressed = await sharp(buffer)
      .jpeg({ quality, mozjpeg: true, progressive: true })
      .toBuffer();
    
    const currentSizeKB = compressed.length / 1024;
    
    if (currentSizeKB > targetKB * 1.05) {
      high = quality - 1;
    } else if (currentSizeKB < targetKB * 0.95) {
      low = quality + 1;
      bestBuffer = compressed;
    } else {
      fs.writeFileSync(cachePath, compressed);
      return compressed;
    }
    
    quality = Math.round((low + high) / 2);
  }
  
  fs.writeFileSync(cachePath, bestBuffer);
  return bestBuffer;
}

async function compressVideo(buffer: Buffer, targetMB: number, isPercentage: boolean, originalMB: number, cacheKey: string): Promise<Buffer> {
  const cachePath = join(CACHE_DIR, cacheKey);
  
  // Cek cache
  if (fs.existsSync(cachePath)) {
    const stats = fs.statSync(cachePath);
    if (Date.now() - stats.mtimeMs < CACHE_EXPIRY) {
      return fs.readFileSync(cachePath);
    }
  }

  const tempDir = tmpdir();
  const inputPath = join(tempDir, `vid_in_${Date.now()}.mp4`);
  const outputPath = join(tempDir, `vid_out_${Date.now()}.mp4`);
  
  fs.writeFileSync(inputPath, buffer);

  try {
    const metadata = await getVideoMetadata(inputPath);
    let finalTargetMB = isPercentage ? (originalMB * targetMB) / 100 : targetMB;
    let resultBuffer: Buffer;
    let attempts = 0;
    
    const minSizeMB = finalTargetMB - 0.5;
    const maxSizeMB = finalTargetMB + 0.5;

    do {
      const duration = Math.max(metadata.duration, 1);
      const ratio = Math.max(0.6, finalTargetMB / originalMB);
      
      let targetHeight = Math.max(360, Math.min(metadata.height, 
        Math.round(metadata.height * Math.pow(ratio, 0.8))));
      targetHeight = targetHeight % 2 === 0 ? targetHeight : targetHeight - 1;
      
      let targetWidth = Math.round(targetHeight * (metadata.width / metadata.height));
      targetWidth = targetWidth % 2 === 0 ? targetWidth : targetWidth - 1;

      const audioBitrate = 64;
      const targetBits = (finalTargetMB * 8 * 1024 * 1.1) - (audioBitrate * duration);
      const videoBitrate = Math.max(1200, targetBits / duration);

      const crf = Math.min(32, Math.max(18, 24 - ((originalMB - finalTargetMB) * 0.5)));

      await new Promise<void>((resolve, reject) => {
        ffmpeg(inputPath)
          .videoCodec('libx264')
          .audioCodec('aac')
          .outputOptions([
            `-vf scale=${targetWidth}:${targetHeight}`,
            `-b:v ${videoBitrate.toFixed(0)}k`,
            `-b:a ${audioBitrate}k`,
            '-preset medium',
            '-movflags +faststart',
            '-pix_fmt yuv420p',
            `-crf ${crf}`,
            '-y'
          ])
          .on('error', (err, _stdout, stderr) => {
            console.error('FFmpeg error:', stderr);
            reject(err);
          })
          .on('end', () => resolve())
          .save(outputPath);
      });

      resultBuffer = fs.readFileSync(outputPath);
      const actualMB = resultBuffer.length / (1024 ** 2);
      
      if (actualMB < minSizeMB) {
        finalTargetMB *= 1.2;
      } else if (actualMB > maxSizeMB) {
        finalTargetMB *= 0.8;
      } else {
        break;
      }
      
      attempts++;
    } while (attempts < 5);

    const actualMB = resultBuffer.length / (1024 ** 2);
    if (actualMB < minSizeMB || actualMB > maxSizeMB) {
      throw new Error(`Gagal mencapai toleransi setelah ${attempts} percobaan`);
    }

    fs.writeFileSync(cachePath, resultBuffer);
    return resultBuffer;
  } finally {
    [inputPath, outputPath].forEach(p => fs.existsSync(p) && fs.unlinkSync(p));
  }
}

const ryzenCDN = async (inp: Buffer | { buffer: Buffer; originalname?: string } | Array<Buffer | { buffer: Buffer; originalname?: string }>) => {
  try {
    const form = new FormData();
    const files = Array.isArray(inp) ? inp : [inp];

    for (const file of files) {
      const buffer = Buffer.isBuffer(file) ? file : file.buffer;
      if (!Buffer.isBuffer(buffer)) throw new Error('Invalid buffer format');

      const type = await fileTypeFromBuffer(buffer);
      if (!type) throw new Error('Unsupported file type');

      const originalName = 'originalname' in file ? (file.originalname || 'file').split('.').shift() : 'file';
      
      form.append('file', buffer, {
        filename: `${originalName}.${type.ext}`,
        contentType: type.mime
      });
    }

    const res = await fetch('https://api.ryzendesu.vip/api/uploader/ryzencdn', {
      method: 'POST',
      headers: {
        'accept': 'application/json',
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3',
        'Connection': 'keep-alive',
        'Accept-Encoding': 'gzip, deflate, br',
        ...form.getHeaders(),
      },
      body: form,
    });

    type RyzenCDNResponse = { success: boolean; message?: string; url?: string; [key: string]: string | boolean | number | object | null | undefined };
    const json = await res.json() as RyzenCDNResponse;
    if (!json.success) throw new Error(json.message || 'Upload failed');

    return Array.isArray(inp) ? (json as unknown as RyzenCDNResponse[]).map((f: RyzenCDNResponse) => f.url) : json.url;
    
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(`RyzenCDN Error: ${error.message}`);
    } else {
      throw new Error('RyzenCDN Error: Unknown error');
    }
  }
};

export { ryzenCDN };

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url);
  const url = searchParams.get('url');
  const sizeParam = searchParams.get('size');

  if (!url || !sizeParam) {
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
          const cacheKey = generateCacheKey(url, sizeParam);
          const cachePath = join(CACHE_DIR, cacheKey);
          let resultBuffer: Buffer;

          // Cek cache
          if (fs.existsSync(cachePath)) {
            const stats = fs.statSync(cachePath);
            if (Date.now() - stats.mtimeMs < CACHE_EXPIRY) {
              resultBuffer = fs.readFileSync(cachePath);
              const fileLink = await ryzenCDN(resultBuffer);
              return NextResponse.json(
                { 
                  status: 'success',
                  data: {
                    link: fileLink,
                    cached: true
                  }
                },
                { status: 200 }
              );
            }
          }

          // Proses download dan kompresi
          const response = await axios.get(url, {
            responseType: 'arraybuffer',
            headers: { 'User-Agent': 'Mozilla/5.0' },
            timeout: 45000,
            maxContentLength: 500 * 1024 * 1024,
          });

          const buffer = Buffer.from(response.data);
          const ext = path.extname(url).toLowerCase().split('?')[0];
          const isPercentage = sizeParam.includes('%');
          const sizeValue = parseFloat(sizeParam.replace('%', ''));

          // Validasi
          if (isNaN(sizeValue) || 
              (isPercentage && (sizeValue < 5 || sizeValue > 100)) || 
              (!isPercentage && sizeValue < 1)) {
            return NextResponse.json(
              { 
                status: 'error',
                message: 'Parameter size tidak valid'
              },
              { status: 400 }
            );
          }

          // Proses kompresi
          if (['.jpg', '.jpeg', '.png'].includes(ext)) {
            const originalKB = buffer.length / 1024;
            const targetKB = isPercentage ? 
              (originalKB * sizeValue) / 100 : 
              sizeValue * 1024;
            
            resultBuffer = await compressImage(buffer, targetKB, cacheKey);
          } else if (['.mp4', '.mov', '.avi'].includes(ext)) {
            const originalMB = buffer.length / (1024 ** 2);
            resultBuffer = await compressVideo(buffer, sizeValue, isPercentage, originalMB, cacheKey);
          } else {
            return NextResponse.json(
              { 
                status: 'error',
                message: 'Format tidak didukung'
              },
              { status: 400 }
            );
          }

          // Upload ke RyzenCDN
          const fileLink = await ryzenCDN(resultBuffer);

          return NextResponse.json(
            { 
              status: 'success',
              data: {
                link: fileLink,
                cached: false
              }
            },
            { status: 200 }
          );
        } catch (error) {
          console.error('Error processing:', error);
          return NextResponse.json(
            { 
              status: 'error',
              message: error instanceof Error ? error.message : 'Kompresi gagal'
            },
            { status: 500 }
          );
        }
      },
      resolve
    });

    if (!isProcessing) processNext();
  });
}