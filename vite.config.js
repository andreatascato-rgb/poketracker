import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST ?? "localhost";

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [tailwindcss(), sveltekit()],

  // Opzioni Tauri: porta fissa, no clear per vedere errori Rust
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    // host esplicito così il webview Tauri raggiunge il server; HMR sempre attivo per evitare full reload → schermo nero
    host: true,
    hmr: {
      protocol: "ws",
      host,
      port: 1420,
      clientPort: 1420,
    },
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
