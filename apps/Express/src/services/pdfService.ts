import { Request, Response } from 'express';
import multer from 'multer';
import { PDFDocument } from 'pdf-lib';

// Initialize Multer to handle file uploads
const upload = multer({ storage: multer.memoryStorage() });

// Function to merge PDFs
export const mergePdfs = async (files: Express.Multer.File[]): Promise<Buffer> => {
  const mergedPdfDoc = await PDFDocument.create();

  for (const file of files) {
    const fileBuffer = file.buffer;
    const existingPdfDoc = await PDFDocument.load(fileBuffer);
    const copiedPages = await mergedPdfDoc.copyPages(existingPdfDoc, existingPdfDoc.getPageIndices());
    copiedPages.forEach(page => mergedPdfDoc.addPage(page));
  }

  const pdfBytes = await mergedPdfDoc.save();
  return Buffer.from(pdfBytes);
};

// Express Route to handle file uploads and PDF merging
export const mergePdfRoute = (req: Request, res: Response) => {
  upload.array('files', 2)(req, res, async (err: any) => {
    if (err) {
      return res.status(400).json({ error: 'Failed to upload files' });
    }

    if (!req.files || (req.files as Express.Multer.File[]).length < 2) {
      return res.status(400).json({ error: 'Please upload at least 2 PDF files' });
    }

    try {
      const mergedPdfBuffer = await mergePdfs(req.files as Express.Multer.File[]);
      res.contentType('application/pdf');
      res.setHeader('Content-Disposition', 'attachment; filename="merged.pdf"');
      res.send(mergedPdfBuffer);
    } catch (error) {
      console.error('Error merging PDFs:', error);
      res.status(500).json({ error: 'Failed to merge PDFs' });
    }
  });
};
