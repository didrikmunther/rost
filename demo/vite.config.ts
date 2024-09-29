import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import wasm from 'vite-plugin-wasm';

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    target: 'esnext',
  },
  base: 'rost',
  plugins: [react(), wasm()],
})
