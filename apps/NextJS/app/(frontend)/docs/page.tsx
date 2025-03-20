export const metadata = {
  title: {
    default: 'Dokumentasi API',
    template: '%s - Dokumentasi API',
  },
  description: 'Dokumentasi API untuk digunakan secara gratis',
  keywords: 'nextjs, api, free',
};

import React from 'react';
import SwaggerUI from 'swagger-ui-react';
import 'swagger-ui-react/swagger-ui.css';
import './custom.css';
import fs from 'fs';
import path from 'path';
import yaml from 'js-yaml';

async function getOpenApiSpec() {
  const filePath = path.join(process.cwd(), 'public', 'OpenApi.yaml');
  const OpenApiYaml = fs.readFileSync(filePath, 'utf8');
  return yaml.load(OpenApiYaml) as Record<string, unknown>;
}

export default async function OpenApiDocsPage() {
  const openApiSpec = await getOpenApiSpec();

  return <SwaggerUI spec={openApiSpec} displayOperationId={true} />;
}
