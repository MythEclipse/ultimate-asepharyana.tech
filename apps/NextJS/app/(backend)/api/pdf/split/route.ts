import { NextResponse } from 'next/server'
import { PDFDocument } from 'pdf-lib'

export async function POST(request: Request) {
  const formData = await request.formData()
  const file = formData.get('file') as File
  const pages = formData.get('pages') as string

  const pdfDoc = await PDFDocument.load(await file.arrayBuffer())
  const newPdf = await PDFDocument.create()
  
  const pageIndices = pages.split(',').map(Number)
  const copiedPages = await newPdf.copyPages(pdfDoc, pageIndices)
  copiedPages.forEach(page => newPdf.addPage(page))

  const pdfBytes = await newPdf.save()
  
  return new NextResponse(pdfBytes, {
    headers: {
      'Content-Type': 'application/pdf',
      'Content-Disposition': 'attachment; filename="split.pdf"'
    }
  })
}