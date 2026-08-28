import { defineConfig } from "vite";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { readFile, rm, writeFile } from "node:fs/promises";

const publicShell = ["/", "/privacy/", "/terms/", "/assets/exit-route-900.webp", "/assets/exit-route.webp", "/favicon.svg"];
const outputDirectory = resolve(import.meta.dirname, "dist/site");

function precacheServiceWorker() {
  return {
    name: "precache-service-worker",
    async closeBundle() {
      const manifestPath = resolve(outputDirectory, ".vite/manifest.json");
      const manifest = JSON.parse(await readFile(manifestPath, "utf8")) as Record<string, { file: string; css?: string[] }>;
      const hashedAppAssets = [...new Set(Object.values(manifest).flatMap((entry) => [entry.file, ...(entry.css || [])]))]
        .filter((file) => /\.(?:css|js)$/u.test(file))
        .map((file) => `/${file}`)
        .sort();
      const shell = [...new Set([...publicShell, ...hashedAppAssets])];
      const version = createHash("sha256").update(shell.join("\n")).digest("hex").slice(0, 12);
      const source = `const VERSION = "sem-shell-${version}";
const SHELL = ${JSON.stringify(shell, null, 2)};

self.addEventListener("install", (event) => {
  event.waitUntil(caches.open(VERSION).then((cache) => cache.addAll(SHELL)));
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(caches.keys().then((keys) => Promise.all(keys.filter((key) => key !== VERSION).map((key) => caches.delete(key)))));
  self.clients.claim();
});

self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET" || new URL(event.request.url).origin !== self.location.origin) return;
  event.respondWith(caches.match(event.request).then((cached) => {
    if (cached) return cached;
    return fetch(event.request).then((response) => {
      if (response.ok) caches.open(VERSION).then((cache) => cache.put(event.request, response.clone()));
      return response;
    }).catch(() => event.request.mode === "navigate" ? caches.match("/") : Response.error());
  }));
});
`;

      await writeFile(resolve(outputDirectory, "sw.js"), source);
      await rm(resolve(outputDirectory, ".vite"), { recursive: true, force: true });
    }
  };
}

export default defineConfig({
  root: "site",
  plugins: [precacheServiceWorker()],
  build: {
    outDir: "../dist/site",
    emptyOutDir: true,
    manifest: true,
    target: "es2022",
    cssCodeSplit: false,
    rollupOptions: {
      input: {
        main: resolve(import.meta.dirname, "site/index.html"),
        privacy: resolve(import.meta.dirname, "site/privacy/index.html"),
        terms: resolve(import.meta.dirname, "site/terms/index.html")
      }
    }
  }
});
