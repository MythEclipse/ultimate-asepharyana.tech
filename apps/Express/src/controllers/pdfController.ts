import { Request, Response } from 'express';
import { mergePdfs } from '../services/pdfService';
import { handleServiceError } from '../utils/errorUtils';

export const mergePdfController = async (req: Request, res: Response) => {
  try {
    const files = req.files as Express.Multer.File[];

    const mergedPdf = await mergePdfs(files);

    res.setHeader('Content-Type', 'application/pdf');
    res.setHeader('Content-Disposition', 'attachment; filename="merged.pdf"');
    return res.send(mergedPdf);
  } catch (error) {
    const err = handleServiceError(error, 'PdfController', 'merge PDFs');
    return res.status(500).json({ error: err.message });
  }
};
