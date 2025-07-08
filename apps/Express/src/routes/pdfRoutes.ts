import { Application, Request, Response } from 'express';
import multer from 'multer';
import { mergePdfs } from '../services/pdfService';
import { handleServiceError } from '../utils/errorUtils';

const upload = multer({ storage: multer.memoryStorage() });

export function setPdfRoutes(app: Application) {
  app.post('/merge-pdfs', upload.array('files', 2), async (req: Request, res: Response) => {
    try {
      if (!req.files || (req.files as Express.Multer.File[]).length < 2) {
        res.status(400).json({ error: 'Please upload at least 2 PDF files' });
        return;
      }

      const mergedPdfBuffer = await mergePdfs(
        req.files as Express.Multer.File[]
      );
      res.contentType('application/pdf');
      res.setHeader('Content-Disposition', 'attachment; filename="merged.pdf"');
      res.send(mergedPdfBuffer);
    } catch (error) {
      const err = handleServiceError(error, 'PdfRoutes', 'merge PDFs');
      res.status(500).json({ error: err.message });
    }
  });
}