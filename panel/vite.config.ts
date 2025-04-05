import path from "path"
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  logLevel: 'info',
  plugins: [react()],
  build: {target: "esnext"},
  resolve: {
    alias: {
      "@shadcn": path.resolve(__dirname, "shadcn/components/ui"),
      "@": path.resolve(__dirname, "src"),
    },
  },
})
