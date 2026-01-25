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
let indexJsContent = readFileSync(indexJsPath, 'utf-8');

if (!indexJsContent.includes('./src/lib/types.js')) {
  indexJsContent = indexJsContent.replace(
    /export \* from '\.\/src\/lib\/database\.js';/,
    "export * from './src/lib/database.js';\nexport * from './src/lib/types.js';",
  );
}

if (!indexJsContent.includes('drizzle-orm')) {
  indexJsContent = indexJsContent.replace(
    /(export \* from '\.\/src\/lib\/types\.js';)/,
    "$1\n\n// Re-export commonly used drizzle-orm operators\nexport { eq, ne, gt, gte, lt, lte, isNull, isNotNull, inArray, notInArray, exists, notExists, between, notBetween, like, notLike, ilike, notIlike, and, or, not, sql, desc, asc } from 'drizzle-orm';",
  );
}

writeFileSync(indexJsPath, indexJsContent, 'utf-8');
console.log('✅ Fixed dist/index.js exports');

// Fix index.d.ts
const indexDtsPath = join(distDir, 'index.d.ts');
let indexDtsContent = readFileSync(indexDtsPath, 'utf-8');

if (!indexDtsContent.includes('./src/lib/types.js')) {
  indexDtsContent = indexDtsContent.replace(
    /export \* from '\.\/src\/lib\/database\.js';/,
    "export * from './src/lib/database.js';\nexport * from './src/lib/types.js';",
  );
}

if (!indexDtsContent.includes('drizzle-orm')) {
  indexDtsContent = indexDtsContent.replace(
    /(export \* from '\.\/src\/lib\/types\.js';)/,
    "$1\nexport { eq, ne, gt, gte, lt, lte, isNull, isNotNull, inArray, notInArray, exists, notExists, between, notBetween, like, notLike, ilike, notIlike, and, or, not, sql, desc, asc } from 'drizzle-orm';",
  );
}

writeFileSync(indexDtsPath, indexDtsContent, 'utf-8');
console.log('✅ Fixed dist/index.d.ts exports');

console.log('✅ Export fix completed successfully');
