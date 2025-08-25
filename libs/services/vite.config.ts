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
    dts({
      entryRoot: 'src',
      tsconfigPath: path.join(__dirname, 'tsconfig.lib.json'),
    }),
  ],
  // Uncomment this if you are using workers.
  // worker: {
  //  plugins: [ nxViteTsPaths() ],
  // },
  // Configuration for building your library.
  // See: https://vitejs.dev/guide/build.html#library-mode
  build: {
    emptyOutDir: true,
    transformMixedEsModules: true,
    entry: 'src/index.ts',
    name: '@asepharyana/services',
    fileName: 'index',
    formats: ['es' as const],
    external: [],
    lib: {
      entry: 'src/index.ts',
      name: 'database',
      fileName: 'index',
      formats: ['es' as const],
    },
    rollupOptions: { external: [] },
  outDir: '../../dist/libs',
    reportCompressedSize: true,
    commonjsOptions: { transformMixedEsModules: true },
  },
}));
