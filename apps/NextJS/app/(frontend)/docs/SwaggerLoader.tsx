// app/api-docs/SwaggerLoader.tsx
'use client';

import dynamic from 'next/dynamic';
import type { OpenAPIV3 } from 'openapi-types';

const SwaggerUIWrapper = dynamic(() => import('./SwaggerUIWrapper'), {
  ssr: false,
  loading: () => (
    <div className='loading-container'>
      <div className='loading-spinner' />
      <p>Memuat dokumentasi API...</p>
    </div>
  ),
});

interface SwaggerLoaderProps {
  spec: OpenAPIV3.Document;
}

export default function SwaggerLoader({ spec }: SwaggerLoaderProps) {
  return <SwaggerUIWrapper spec={spec} />;
}
