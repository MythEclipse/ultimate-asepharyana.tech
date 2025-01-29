import { Request, Response } from 'express';
import { mergePdfs } from '@/services/pdfService';

export const mergePdfController = async (req: Request, res: Response) => {
  try {
    const files = req.files as Express.Multer.File[];

    if (files.length < 2) {
      return res
        .status(400)
        .json({ error: 'Please upload at least 2 PDF files' });
    }

    const mergedPdf = await mergePdfs(files);

    res.setHeader('Content-Type', 'application/pdf');
    res.setHeader('Content-Disposition', 'attachment; filename="merged.pdf"');
    return res.send(mergedPdf);
  } catch (error) {
    console.error('Merge error:', error);
    const errorMessage =
      error instanceof Error ? error.message : 'Unknown error';
    return res
      .status(500)
      .json({ error: `Failed to merge PDFs: ${errorMessage}` });
  }
};
