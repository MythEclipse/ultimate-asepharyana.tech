#!/usr/bin/env node
/**
 * Post-build script to fix missing exports in dist/index.js and dist/index.d.ts
 * This is a workaround for TypeScript compiler not including all exports
 */

import { readFileSync, writeFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const distDir = join(__dirname, 'dist');

// Fix index.js
const indexJsPath = join(distDir, 'index.js');
const indexJsContent = readFileSync(indexJsPath, 'utf-8');

if (!indexJsContent.includes('./src/lib/types.js')) {
  const fixedContent = indexJsContent.replace(
    /export \* from '\.\/src\/lib\/database\.js';/,
    "export * from './src/lib/database.js';\nexport * from './src/lib/types.js';"
  );
  writeFileSync(indexJsPath, fixedContent, 'utf-8');
  console.log('✅ Fixed dist/index.js exports');
}

// Fix index.d.ts
const indexDtsPath = join(distDir, 'index.d.ts');
const indexDtsContent = readFileSync(indexDtsPath, 'utf-8');

if (!indexDtsContent.includes('./src/lib/types.js')) {
  const fixedContent = indexDtsContent.replace(
    /export \* from '\.\/src\/lib\/database\.js';/,
    "export * from './src/lib/database.js';\nexport * from './src/lib/types.js';"
  );
  writeFileSync(indexDtsPath, fixedContent, 'utf-8');
  console.log('✅ Fixed dist/index.d.ts exports');
}

console.log('✅ Export fix completed successfully');
