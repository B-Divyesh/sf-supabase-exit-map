import assert from "node:assert/strict";
import { readFile, stat } from "node:fs/promises";
import { test } from "node:test";

const dist = (path) => readFile(new URL("../../dist/site/" + path, import.meta.url), "utf8");

test("built routes provide landmarks, canonical metadata, and social previews", async () => {
  for (const [path, title] of [
    ["index.html", "Supabase Exit Map — plan a Supabase exit"],
    ["demo/index.html", "Demo — Supabase Exit Map"],
    ["privacy/index.html", "Privacy — Supabase Exit Map"],
    ["terms/index.html", "Terms — Supabase Exit Map"],
    ["404.html", "Page not found — Supabase Exit Map"]
  ]) {
    const html = await dist(path);
    assert.match(html, /<html lang="en">/);
    assert.match(html, new RegExp("<title>" + title + "</title>"));
    assert.match(html, /<main\b/);
    assert.equal((html.match(/<h1\b/g) || []).length, 1, path);
    assert.match(html, /<link rel="canonical" href="https:\/\/supabase-exit-map\.sociobot\.in\//);
    assert.match(html, /property="og:image" content="https:\/\/supabase-exit-map\.sociobot\.in\/assets\/exit-map-social\.webp"/);
    assert.match(html, /name="twitter:card" content="summary_large_image"/);
    assert.match(html, /apple-touch-icon/);
    assert.doesNotMatch(html, /https:\/\/(fonts|cdn|unpkg|jsdelivr)\./);
  }
});

test("built visual assets have usable dimensions and stay inside budgets", async () => {
  const html = await dist("index.html");
  assert.match(html, /<img src="\/assets\/exit-route\.webp" width="1536" height="1024" alt="[^"]+"/);
  for (const name of ["exit-route.webp", "exit-route-900.webp", "exit-map-social.webp"]) {
    const info = await stat(new URL("../public/assets/" + name, import.meta.url));
    assert.ok(info.size <= 300_000, name + " is " + info.size + " bytes");
  }
  const touch = await stat(new URL("../public/apple-touch-icon.png", import.meta.url));
  assert.ok(touch.size > 0);
});

test("production service worker precaches every route module and stylesheet", async () => {
  const sw = await dist("sw.js");
  const shellMatch = sw.match(/const SHELL = ([\s\S]+?);\n\nself\.addEventListener/u);
  assert.ok(shellMatch, "service worker has a serialized precache shell");
  const shell = JSON.parse(shellMatch[1]);
  assert.match(sw, /const VERSION = "sem-shell-[a-f0-9]{12}"/);
  assert.match(sw, /event\.request\.mode === "navigate" \? caches\.match\("\/"\) : Response\.error\(\)/);

  for (const page of ["index.html", "demo/index.html", "privacy/index.html", "terms/index.html", "404.html"]) {
    const html = await dist(page);
    const assets = [...html.matchAll(/(?:src|href)="(\/assets\/[^"]+\.(?:js|css))"/gu)].map((match) => match[1]);
    assert.ok(assets.length > 0, page + " references built assets");
    for (const asset of assets) assert.ok(shell.includes(asset), asset + " is precached");
  }
  for (const page of ["/demo/", "/404.html"]) assert.ok(shell.includes(page), page + " is in the app shell");
});

test("static deployment configuration serves a designed 404 and response security policy", async () => {
  const config = JSON.parse(await readFile(new URL("../public/staticwebapp.config.json", import.meta.url), "utf8"));
  assert.equal(config.responseOverrides["404"].rewrite, "/404.html");
  assert.equal(config.globalHeaders["X-Frame-Options"], "DENY");
  assert.equal(config.globalHeaders["Permissions-Policy"], "camera=(), microphone=(), geolocation=()");
  assert.match(config.globalHeaders["Content-Security-Policy"], /connect-src 'self' https:\/\/api\.sociobot\.in/);
  assert.match(config.globalHeaders["Content-Security-Policy"], /frame-ancestors 'none'/);
  for (const route of ["/assets/*", "/*.js", "/*.css"]) {
    assert.equal(config.routes.find((item) => item.route === route)?.headers["Cache-Control"], "public, max-age=31536000, immutable");
  }
  const notFound = await dist("404.html");
  assert.match(notFound, /<h1>Page not found<\/h1>/);
  assert.match(notFound, /Return to the home page/);
});
