import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

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
        rewrite: (requestPath) => requestPath
      }
    }
  },
  json: {
    stringify: true
  },
  build: {
    commonjsOptions: {
      transformMixedEsModules: true
    }
  }
})
