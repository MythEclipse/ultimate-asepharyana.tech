import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
import * as path from 'path';

export default defineConfig(() => ({
  root: __dirname,
  cacheDir: '../node_modules/.vite/libs',
  plugins: [
    dts({
      entryRoot: 'src',
      tsconfigPath: path.join(__dirname, 'tsconfig.lib.json'),
    }),
  ],
  // Configuration for building your library.
  // See: https://vitejs.dev/guide/build.html#library-mode
  build: {
    emptyOutDir: true,
    ssr: true, // Build for Node.js (SSR mode)
    lib: {
      entry: 'src/index.ts',
      name: 'database',
      fileName: 'index',
      formats: ['es' as const],
    },
    rollupOptions: {
      // Externalize all Node.js built-ins and dependencies
      external: [
        'drizzle-orm',
        'drizzle-orm/mysql2',
        'mysql2',
        'mysql2/promise',
        'tslib',
        /^node:/,
        'net',
        'tls',
        'timers',
        'stream',
        'crypto',
        'zlib',
        'events',
        'fs',
        'path',
        'url',
        'util',
        'os',
      ],
    },
    outDir: 'dist',
    reportCompressedSize: true,
    commonjsOptions: { transformMixedEsModules: true },
  },
}));
