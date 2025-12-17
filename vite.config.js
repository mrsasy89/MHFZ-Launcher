import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from "path";

export default defineConfig({
  // prevent vite from obscuring rust errors
  clearScreen: false,

  // Tauri expects a fixed port, fail if that port is not available
  server: {
    strictPort: true,

    // ✅ FIX: Escludi directory Wine/Game dal watch di Vite
    watch: {
      ignored: [
        '**/node_modules/**',
        '**/.git/**',
        '**/game/**',                          // Escludi cartella game
        '**/MHFZ-Launcher-Linux-v1.4.6/**',   // Escludi build Linux
        '**/*.AppImage',
        '**/pfx/**',                           // Escludi Wine prefix
        '**/dosdevices/**',                    // Escludi Wine symlinks
        '**/.local/**',                        // Escludi dati locali
        '**/target/**',                        // Escludi build Rust
      ]
    }
  },

  plugins: [vue()],

                            // to access the Tauri environment variables set by the CLI with information about the current target
                            envPrefix: ['VITE_', 'TAURI_PLATFORM', 'TAURI_ARCH', 'TAURI_FAMILY', 'TAURI_PLATFORM_VERSION', 'TAURI_PLATFORM_TYPE', 'TAURI_DEBUG'],

                            build: {
                              // Tauri uses Chromium on Windows and WebKit on macOS and Linux
                              target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
                            // don't minify for debug builds
                            minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
                            // produce sourcemaps for debug builds
                            sourcemap: !!process.env.TAURI_DEBUG,
                            },

                            // ✅ Previeni warning sui symlink
                            resolve: {
                              preserveSymlinks: false
                            }
})
