# Handoff — Supabase Exit Map repair

## Release status: PASS

Repair commit: `7933c8eb95cd5fbbb2488edac9c60def935c6e52` (pushed to `main`).

Deployment class remains **static**. The production build was deployed to
<https://supabase-exit-map.sociobot.in/> on 2026-08-28 UTC.

## What changed

1. Replaced the hand-maintained public service worker with a Vite build plugin.
   It reads Vite's emitted manifest, writes a versioned cache name, and precaches
   every emitted JavaScript and CSS file plus each page shell, favicon, and both
   responsive hero images. Navigation alone may fall back to the cached shell;
   a missing script or stylesheet now returns an error rather than HTML.
2. Fixed the 390px command-block overflow by allowing both install-grid columns
   to shrink (`minmax(0, 1fr)` and `min-width: 0`) and constraining the terminal
   to its column. Long commands retain horizontal scrolling inside the terminal.
3. Replaced unsupported `site/public/_headers` with Azure Static Web Apps'
   `staticwebapp.config.json`. It supplies immutable caching for `/assets/*`,
   JS, and CSS; `X-Frame-Options: DENY`; the camera/microphone/geolocation
   permissions policy; `nosniff`; referrer policy; and a CSP that permits only
   same-origin assets plus the documented Sociobot license-verification API.
4. Added strict TypeScript checking and fixed the existing `HTMLTableSectionElement`
   typing issue found by it. Added pinned Playwright and axe browser test tooling.

## Regression coverage

- `site/tests/site.test.mjs` verifies the generated production service worker
  contains every JS/CSS file referenced by each built HTML page and verifies the
  Azure cache/security configuration.
- `site/tests/release.browser.mjs` verifies a controlled-service-worker offline
  reload has no module MIME errors; 390px document width is exactly 390px while
  command overflow remains internal; Arrow-key tabs work; and axe has no serious
  or critical violations at 1440px or 390px.

## Verification evidence

From a clean `npm ci` on Node 22 / Rust 1.98:

```sh
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
```

All passed: 5 Rust integration tests, 1 compiling doctest, 6 site/build tests,
and 4 Chromium browser tests. The browser suite includes axe WCAG 2 A/AA, 2.1
AA, and 2.2 AA checks with zero serious or critical violations at desktop and
390px mobile. `npm test` also runs `tsc --noEmit`.

The ready-to-publish package check succeeded (12 files; 51.4 KiB unpacked,
15.3 KiB compressed). A clean temporary consumer install with
`cargo install --path cli --root <temp> --force` ran `--version` (`0.1.0`) and
`--help` successfully. Nothing was published.

Built uncompressed budgets: entry JS 6,944 bytes plus 711-byte shared JS; CSS
10,445 bytes; desktop/mobile WebP 37,304 / 14,154 bytes; system fonts 0 bytes.

Live checks against the deployed URL found title, `lang=en`, one H1, a main
landmark, image alt text, and zero browser console errors. A fresh desktop load
made no third-party requests. At 390×844, `document.documentElement.scrollWidth`
was exactly `390`; after service-worker activation, an offline reload rendered
the H1 with no console errors.

Live response policy and caching are now present on HTML, JS, CSS, and WebP:

```text
Cache-Control: public, max-age=31536000, immutable  (assets)
Content-Security-Policy: default-src 'self'; ... connect-src 'self' https://api.sociobot.in; frame-ancestors 'none'; ...
Permissions-Policy: camera=(), microphone=(), geolocation=()
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
Referrer-Policy: strict-origin-when-cross-origin
```

Fresh production hashes matched the local `dist/site` output exactly:

| File | SHA-256 |
| --- | --- |
| `index.html` | `d8bc3e9dad10c3983944128c47eaa50798fb8c8dc3f62c1ec6cad3dfe14f9cb4` |
| `sw.js` | `1fccbc5708195b21dd07bf24e539375c997fe09142fd3ef83c48c1a80cfe7349` |
| `assets/main-CU0Ogf8H.js` | `9bb085c2ff1c12e822453454c4165ed3ef08f9b23dad66a2086f164ae1392b62` |
| `assets/style-DZM9qoIZ.css` | `1817b5f06adb4714055c45417824f0f039aa57da4936f79d9f592f47ff8d18f8` |

`/opt/fleet/lib/verify-url.sh` also passed against production (760ms measured
load; no errors; title/lang/H1/main/alt checks passed).

## Run and deploy

```sh
npm ci
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm test
npm run build
cargo package --manifest-path cli/Cargo.toml
```

Deploy the static artifact with the factory work-order command:

```sh
npm ci && npm run build:site
/opt/fleet/lib/deploy-static.sh supabase-exit-map /work/repo/dist/site
```

## Known gap

Lighthouse was attempted against the live URL with the preinstalled Chrome for
Testing (145.0.7632.6), but Lighthouse's browser process crashed before it
produced a report. No Lighthouse score is claimed. The direct build budgets,
Playwright desktop/mobile checks, axe checks, and live `verify-url.sh` check all
passed.
