import { readFile } from 'fs/promises';
import path from 'path';
import yaml from 'js-yaml';
import type { OpenAPIV3 } from 'openapi-types';
import SwaggerUIComponent from './SwaggerUIComponent';

export const metadata = {
  title: {
    default: 'Dokumentasi API Gratis',
    template: '%s | Dokumentasi API',
  },
  description:
    'Dokumentasi lengkap untuk API gratis kami dengan contoh request, parameter, dan response.',
  keywords:
    'API gratis, dokumentasi API, REST API, integrasi API, nextjs, openapi, swagger',
  openGraph: {
    title: 'Dokumentasi API Gratis',
    description:
      'Dokumentasi lengkap untuk API gratis kami dengan contoh request, parameter, dan response.',
    images: [
      {
        url: '/api-docs-og.png',
        width: 1200,
        height: 630,
      },
    ],
  },
};

async function getOpenApiSpec(): Promise<OpenAPIV3.Document> {
  try {
    const filePath = path.join(process.cwd(), 'public', 'OpenApi.yaml');
    const OpenApiYaml = await readFile(filePath, 'utf8');
    const spec = yaml.load(OpenApiYaml) as OpenAPIV3.Document;

    if (!spec.openapi?.startsWith('3.0.')) {
      throw new Error('Hanya mendukung OpenAPI versi 3.0.x');
    }

    return spec;
  } catch (error) {
    console.error('Error loading OpenAPI spec:', error);
    throw new Error(
      'Gagal memuat dokumentasi. Silakan coba lagi atau hubungi administrator.'
    );
  }
}

export default async function OpenApiDocsPage() {
  let openApiSpec: OpenAPIV3.Document;

  try {
    openApiSpec = await getOpenApiSpec();
  } catch (error) {
    return (
      <div className='error-container'>
        <h1>⚠️ Gagal Memuat Dokumentasi</h1>
        <p>{(error as Error).message}</p>
        <p>Silakan coba:</p>
        <ul>
          <li>Refresh halaman</li>
          <li>Periksa koneksi internet</li>
          <li>Hubungi tim support</li>
        </ul>
      </div>
    );
  }

  return (
    <main className='api-docs-container'>
      <SwaggerUIComponent spec={openApiSpec} />
    </main>
  );
}