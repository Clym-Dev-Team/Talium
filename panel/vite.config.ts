import path from "path"
import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  base: "/panel",
  server: {
    proxy: {
      "/bot": "http://localhost:4771"
    },
    allowedHosts: ["joint-urgently-grubworm.ngrok-free.app"]
  },
  logLevel: 'info',
  plugins: [react()],
  build: {target: "esnext"},
  resolve: {
    alias: {
      "@shadcn": path.resolve(__dirname, "shadcn/components/ui"),
      "@i": path.resolve(__dirname, "src/icons"),
      "@s": path.resolve(__dirname, "src/shared"),
      "@c": path.resolve(__dirname, "src/components"),
      "@": path.resolve(__dirname, "src"),
    },
  },
})
