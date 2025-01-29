// app/api/convert-from-pdf/route.ts
import { NextResponse } from 'next/server';
import { readFileSync, mkdirSync, rmSync, readdirSync } from 'fs';
import { join } from 'path';
import { Poppler } from 'node-poppler';
import { Document, Packer, Paragraph } from 'docx';
import JSZip from 'jszip';
import { IncomingForm } from 'formidable';
import pdfParse from 'pdf-parse';

export const runtime = 'nodejs';

import { IncomingMessage } from 'http';

async function parseForm(req: Request): Promise<{ fields: Record<string, string | string[]>; files: Record<string, { filepath: string }[]> }> {
  return new Promise((resolve, reject) => {
    const form = new IncomingForm();
    const incReq = req as unknown as IncomingMessage;
    form.parse(incReq, (err, fields, files) => {
      if (err) reject(err);
      else resolve({
        fields: Object.fromEntries(Object.entries(fields).map(([key, value]) => [key, value ?? ''])),
        files: Object.fromEntries(Object.entries(files).map(([key, value]) => [key, value ?? []]))
      });
    });
  });
}

async function convertPDFToDocx(pdfPath: string): Promise<Buffer> {
  const { text } = await pdfParse(readFileSync(pdfPath));
  
  const doc = new Document({
    sections: [{
      properties: {},
      children: [new Paragraph(text)]
    }]
  });

  return Packer.toBuffer(doc);
}

async function convertPDFToImages(pdfPath: string, format: 'jpg' | 'png'): Promise<Buffer[]> {
  const poppler = new Poppler();
  const tempDir = join(process.cwd(), 'temp', Date.now().toString());
  mkdirSync(tempDir, { recursive: true });

  const outputPrefix = join(tempDir, 'page');
  const options = {
    [format === 'png' ? 'pngFile' : 'jpegFile']: true,
  };

  await poppler.pdfToCairo(pdfPath, outputPrefix, options);

  const files = readdirSync(tempDir)
    .filter(file => file.endsWith(`.${format}`))
    .sort((a, b) => {
      const numA = parseInt(a.split('-')[1]);
      const numB = parseInt(b.split('-')[1]);
      return numA - numB;
    });

  const buffers = files.map(file => 
    readFileSync(join(tempDir, file))
  );

  rmSync(tempDir, { recursive: true, force: true });
  return buffers;
}

export async function POST(request: Request) {
  const zip = new JSZip();
  try {
    const { fields, files } = await parseForm(request);
    const format = (fields.format as string).toLowerCase();
    const file = files.file?.[0];

    if (!file || !format) {
      return NextResponse.json(
        { error: 'File and format are required' },
        { status: 400 }
      );
    }

    let result: Buffer | Buffer[];
    
    switch(format) {
      case 'docx':
        result = await convertPDFToDocx(file.filepath);
        break;

      case 'jpg':
      case 'jpeg':
      case 'png':
        const images = await convertPDFToImages(
          file.filepath, 
          format === 'jpg' ? 'jpg' : 'png'
        );
        result = images;
        break;

      default:
        return NextResponse.json(
          { error: 'Unsupported format' },
          { status: 400 }
        );
    }

    // Cleanup uploaded file
    rmSync(file.filepath, { force: true });

    // Handle response
    if (format === 'docx') {
      return new NextResponse(result as Buffer, {
        headers: {
          'Content-Type': 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
          'Content-Disposition': 'attachment; filename="converted.docx"'
        }
      });
    }
    else {
      const images = result as Buffer[];
      const imageFormat = format === 'jpg' ? 'jpeg' : format;

      if (images.length === 1) {
        return new NextResponse(images[0], {
          headers: {
            'Content-Type': `image/${imageFormat}`,
            'Content-Disposition': `attachment; filename="converted.${format}"`
          }
        });
      } else {
        images.forEach((img, i) => {
          zip.file(`page_${i + 1}.${format}`, img);
        });
        
        const zipBuffer = await zip.generateAsync({ type: 'nodebuffer' });
        return new NextResponse(zipBuffer, {
          headers: {
            'Content-Type': 'application/zip',
            'Content-Disposition': 'attachment; filename="converted_pages.zip"'
          }
        });
      }
    }
  } catch (error) {
    console.error('Conversion error:', error);
    return NextResponse.json(
      { error: 'Conversion failed' },
      { status: 500 }
    );
  }
}