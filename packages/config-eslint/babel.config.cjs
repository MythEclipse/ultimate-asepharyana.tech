module.exports = {
  env: {
    // Konfigurasi untuk CommonJS (tidak berubah)
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
    
    // Konfigurasi untuk ES Modules (diupdate)
    esm: {
      presets: [
        [
          '@babel/preset-env',
          {
            targets: { node: 'current' },
            modules: false
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
            extensions: ['.js', '.jsx', '.ts', '.tsx']
          }
        ],
        // Tambahkan plugin untuk otomatis menambahkan .js
        ['add-import-extension', { 
          extension: 'js' 
        }]
      ]
    }
  },
  
  // Konfigurasi umum
  plugins: [],
  sourceMaps: true,
  retainLines: true
};