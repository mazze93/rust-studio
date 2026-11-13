// @ts-check
import { defineConfig } from "astro/config";

// Fully static output served by Workers Static Assets: no adapter and no
// Worker script, so requests never invoke compute (see wrangler.jsonc).
export default defineConfig({
  site: "https://rust.mazzeleczzare.com",
  output: "static",
  trailingSlash: "always",
  build: { format: "directory", inlineStylesheets: "never" },
  // Shiki writes inline style attributes, which the CSP forbids; code blocks
  // use the primer's own styles instead.
  markdown: { syntaxHighlight: false },
  vite: {
    // Keep every script an external, hashed file so the CSP in
    // public/_headers can stay `script-src 'self'` with no inline exception.
    build: { assetsInlineLimit: 0 },
    // The site renders files that live outside site/: ../starter sources.
    server: { fs: { allow: [".."] } },
  },
});
