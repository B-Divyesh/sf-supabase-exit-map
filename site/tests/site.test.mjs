import assert from "node:assert/strict";
import { readFile, stat } from "node:fs/promises";
import { test } from "node:test";

const read = (path) => readFile(new URL(`../${path}`, import.meta.url), "utf8");

test("every page has the required document landmarks and one h1", async () => {
  for (const path of ["index.html", "privacy/index.html", "terms/index.html"]) {
    const html = await read(path);
    assert.match(html, /<html lang="en">/);
    assert.match(html, /<title>[^<]+<\/title>/);
    assert.match(html, /<main\b/);
    assert.equal((html.match(/<h1\b/g) || []).length, 1, path);
    assert.doesNotMatch(html, /https:\/\/(fonts|cdn|unpkg|jsdelivr)\./);
  }
});

test("hero images have alternative text, dimensions, and stay in budget", async () => {
  const html = await read("index.html");
  assert.match(html, /<img[^>]+width="1536"[^>]+height="1024"[^>]+alt="[^"]+"/);
  for (const name of ["exit-route.webp", "exit-route-900.webp"]) {
    const info = await stat(new URL(`../public/assets/${name}`, import.meta.url));
    assert.ok(info.size <= 300_000, `${name} is ${info.size} bytes`);
  }
});

test("license flow follows the Sociobot one-time unlock contract", async () => {
  const source = await read("src/main.ts");
  assert.match(source, /sb_license:/);
  assert.match(source, /api\.sociobot\.in\/api\/v1\/products/);
  assert.match(source, /\/verify\?license=/);
  assert.match(source, /history\.replaceState/);
  assert.match(source, /86_400_000/);
  assert.match(source, /localStorage/);
  assert.match(source, /navigator\.onLine/);
});

test("motion and focus baselines are explicit", async () => {
  const css = await read("src/styles.css");
  assert.match(css, /:focus-visible/);
  assert.match(css, /prefers-reduced-motion: reduce/);
  assert.match(css, /min-height: 44px/);
  assert.match(css, /color-scheme: dark/);
});
