module.exports = {
  env: {
    // Konfigurasi untuk CommonJS
    cjs: {
      presets: [
        [
          '@babel/preset-env',
          {
            targets: { node: 'current' },
            modules: 'commonjs'
          }
        ],
        '@babel/preset-typescript'
      ],
      plugins: [
        [
          'module-resolver',
          {
            alias: {
              '@': './src'
            }
          }
        ]
      ]
    },
    // Konfigurasi untuk ES Modules
    esm: {
      presets: [
        [
          '@babel/preset-env',
          {
            targets: { node: 'current' },
            modules: false // Pertahankan sintaks ESM
          }
        ],
        '@babel/preset-typescript'
      ],
      plugins: [
        [
          'module-resolver',
          {
            alias: {
              '@': './src'
            },
            extensions: ['.js', '.jsx', '.ts', '.tsx'] // Tambahkan ekstensi untuk ESM
          }
        ]
      ]
    }
  },
  // Konfigurasi umum untuk kedua environment
  plugins: [
    // Tambahkan plugin yang diperlukan oleh kedua environment di sini
  ]
};