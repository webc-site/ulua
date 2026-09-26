import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { join } from "node:path";
import { existsSync, readFileSync } from "node:fs";
import pugPreprocess from "./vite/pug/build.js";
import { stylusSveltePreprocess, stylusVitePlugin } from "./vite/stylus.js";

const wasmPkgServePlugin = () => ({
  name: "vite-plugin-wasm-pkg-serve",
  configureServer(server) {
    server.middlewares.use((req, res, next) => {
      const url_path = req.url ? req.url.split("?")[0] : "";
      if (url_path.startsWith("/pkg/")) {
        const file_rel = url_path.replace(/^\/pkg\//, ""),
          file_abs = join(import.meta.dirname, "public/pkg", file_rel);
        if (existsSync(file_abs)) {
          if (file_abs.endsWith(".js")) {
            res.setHeader("Content-Type", "application/javascript; charset=utf-8");
          } else if (file_abs.endsWith(".wasm")) {
            res.setHeader("Content-Type", "application/wasm");
          }
          res.end(readFileSync(file_abs));
          return;
        }
      }
      next();
    });
  },
});

export default defineConfig(({ mode }) => {
  const is_prod = mode === "production";

  return {
    base: "./",
    plugins: [
      wasmPkgServePlugin(),
      stylusVitePlugin(),
      svelte({
        preprocess: [pugPreprocess(), stylusSveltePreprocess()],
      }),
    ],

    build: {
      outDir: "dist",
      emptyOutDir: true,
      target: "esnext",
      minify: is_prod ? "oxc" : false,
      cssMinify: is_prod ? "lightningcss" : false,
      reportCompressedSize: true,
      chunkSizeWarningLimit: 1500,
    },
  };
});
