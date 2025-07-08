import { PDFDocument } from 'pdf-lib';
import { handleServiceError } from '../utils/errorUtils';

// Function to merge PDFs
export const mergePdfs = async (
  files: Express.Multer.File[]
): Promise<Buffer> => {
  try {
    const mergedPdfDoc = await PDFDocument.create();

    for (const file of files) {
      const fileBuffer = file.buffer;
      const existingPdfDoc = await PDFDocument.load(fileBuffer);
      const copiedPages = await mergedPdfDoc.copyPages(
        existingPdfDoc,
        existingPdfDoc.getPageIndices()
      );
      copiedPages.forEach((page) => mergedPdfDoc.addPage(page));
    }

    const pdfBytes = await mergedPdfDoc.save();
    return Buffer.from(pdfBytes);
  } catch (error) {
    throw handleServiceError(error, 'PdfService', 'merge PDFs');
  }
};
