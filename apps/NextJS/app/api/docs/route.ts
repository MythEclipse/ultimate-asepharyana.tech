import { NextResponse } from 'next/server';
import fs from 'fs';
import path from 'path';
import yaml from 'js-yaml';
import { withLogging } from '@/lib/api-wrapper';

const filePath = path.join(process.cwd(), 'public', 'OpenApi.yaml');
const OpenApiYaml = fs.readFileSync(filePath, 'utf8');
const OpenApiJson = yaml.load(OpenApiYaml);

async function handler() {
  return NextResponse.json(OpenApiJson);
}

export const GET = withLogging(handler);
