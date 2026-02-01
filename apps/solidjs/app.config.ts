import { defineConfig } from '@solidjs/start/config';

export default defineConfig({
  ssr: true,
  vite: {
    envPrefix: ['VITE_', 'GOOGLE_CLIENT_ID'],
  },
  server: {
    // Bundle tslib inline to fix module resolution issues
    externals: {
      inline: ['tslib'],
    },
    esbuild: {
      options: {
        target: 'esnext',
      },
    },
  },
});
