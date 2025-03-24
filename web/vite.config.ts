import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src')
    }
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8090',
        changeOrigin: true,
        rewrite: (path) => path
      }
    }
  },
  json: {
    stringify: true,
  },
  optimizeDeps: {
    include: ['@coral-xyz/anchor', '@solana/web3.js']
  },
  build: {
    commonjsOptions: {
      transformMixedEsModules: true,
    },
  }
})
