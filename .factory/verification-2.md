# Independent verification 2 — PASS

**Candidate:** `f6a57d2399dd0e792696778f7011e1ff45217ba3` (`main`)  
**Live URL:** <https://supabase-exit-map.sociobot.in/>  
**Verified:** 2026-08-28 UTC  
**Scope:** Clean-install verification of the Rust CLI, publishable package, static site/PWA, and the live deployment. Product source was not changed.

## Decision

**PASS.** The candidate delivers the brief's local-only Supabase dependency
inventory and migration-planning workflow. The former deployment-only failures
were independently rechecked and are not present in this candidate or at the
live URL: the service worker precaches emitted modules, the 390px layout stays
within the viewport, and the live deployment sends immutable asset caching and
the configured response-security headers.

## Local gates

Fresh environment: Node `v22.23.2`, npm `10.9.8`, Rust/Cargo `1.98.0`.

```sh
npm ci
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
```

All passed. `npm test` ran 5 Rust integration tests, 1 compiling doctest,
strict TypeScript checking, 6 built-site tests, and 4 Chromium tests. The exact
production build produced `dist/site/` and `target/release/supabase-exit-map`.
`cargo package` successfully verified the ready-to-publish crate: 12 files,
52.0 KiB unpacked / 15.5 KiB compressed.

The packed `.crate` was unpacked into a clean temporary consumer directory and
installed with `cargo install --path <unpacked-crate> --root <temporary-root>`.
The installed binary ran the public Markdown API successfully.

## CLI end-to-end evidence

An independently made representative project contained local config, GitHub
Auth, `auth.users`, RLS, routines/triggers, `pg_cron`, Storage, Realtime,
Vault, `pg_net`, and a Deno Edge Function with a secret name and outbound call.

- `--json --output` returned schema `1.0`, 11 findings, `high` overall effort,
  and `live_project_queried: false`.
- The Markdown report included a portability route, open-source compatibility,
  hosted-service boundary, and phased checklist for each finding.
- The synthetic Vault secret value did not appear in JSON or Markdown.
- Complete input with `--strict --json` exited `0`; an empty directory with the
  same flags exited `3` with actionable warnings; a missing path and invalid
  `--format` each exited `2`.
- `--version` printed `0.1.0`; `--help` described the path, formats, `--json`,
  output, strict mode, and standard help/version flags.

Dependency inspection (`cargo tree`) found only CLI/serialization dependencies;
there is no HTTP or telemetry client. Static review and the report's
`live_project_queried: false` field support the local-only claim.

## Site, accessibility, PWA, and privacy

Fresh Chromium checks covered the live deployment at desktop 1440×1000 and
mobile 390×844.

- Desktop has the expected title, `lang=en`, one H1, main landmark, image alt,
  no console/page errors, a visible 3px Skip-link focus outline, and keyboard
  Arrow navigation (`tab-database` becomes selected).
- Mobile `document.documentElement.scrollWidth` was exactly `390`. Both command
  blocks remained inside the viewport with `overflow-x: auto`; the long install
  command scrolls internally.
- `@axe-core/playwright` WCAG 2 A/AA, 2.1 AA, and 2.2 AA checks reported zero
  serious or critical findings at both viewports.
- With `prefers-reduced-motion: reduce`, finding animation and transition
  duration was `0.00001s`.
- A mocked valid license return stripped `?license` from the URL, stored the
  token locally, unlocked Planning Room, loaded three example rows, and saved
  a local plan. A short invalid token kept it locked and displayed the recovery
  message. The normal first load made no third-party requests.
- Browser source review found no analytics, trackers, external fonts, or CDN
  scripts. The only deliberate external runtime endpoint is the documented
  Sociobot license verifier/checkout; a report is not included in that request.
- The live service worker activated as `/sw.js`, used versioned cache
  `sem-shell-77e82750da3d`, and survived an offline reload with the H1 rendered
  and zero module/MIME console errors. Its update path calls `skipWaiting`,
  `clients.claim`, and removes stale caches; `registration.update()` completed
  against the live worker.

`/opt/fleet/lib/verify-url.sh` also passed against production: HTTP 200,
730 ms measured load, title/lang/H1/main/alt checks, and no browser errors.

## Deployment identity, headers, and budgets

The fresh production build hashes matched the live deployment exactly:

| File | SHA-256 |
| --- | --- |
| `index.html` | `d8bc3e9dad10c3983944128c47eaa50798fb8c8dc3f62c1ec6cad3dfe14f9cb4` |
| `sw.js` | `1fccbc5708195b21dd07bf24e539375c997fe09142fd3ef83c48c1a80cfe7349` |
| `assets/main-CU0Ogf8H.js` | `9bb085c2ff1c12e822453454c4165ed3ef08f9b23dad66a2086f164ae1392b62` |
| `assets/styles-B5Qt9EMX.js` | `d2a32840421496e872ade591618d2fa5c33797605d1aec04301717e5a90757d0` |
| `assets/style-DZM9qoIZ.css` | `1817b5f06adb4714055c45417824f0f039aa57da4936f79d9f592f47ff8d18f8` |
| `assets/legal-BZvp_fgr.js` | `ab5efe3a1fca8fe7857e0e0f971feed4864f8e7ff44351032c3dbf5d14cffaca` |
| `assets/exit-route.webp` | `7b262010b1bc99f67dcdbe1f268819818eb414dd9fcea9c35220ed2684b1978c` |

`/privacy/` and `/terms/` also matched their local production HTML byte for
byte. Live content-hashed assets return `Cache-Control: public,
max-age=31536000, immutable`. HTML returns the appropriate short revalidation
policy. HTML, JS, CSS, WebP, and SW responses all included the configured CSP
(same-origin assets; `connect-src` permits only Sociobot verification),
`X-Frame-Options: DENY`, `Permissions-Policy: camera=(), microphone=(),
geolocation=()`, `X-Content-Type-Options: nosniff`, and
`Referrer-Policy: strict-origin-when-cross-origin`.

Production bytes: initial JS is 6,944 bytes plus a 711-byte shared chunk; CSS
is 10,445 bytes; self-hosted/remote font bytes are 0; desktop/mobile WebP hero
assets are 37,304 / 14,154 bytes. All are below the stated static-product
budgets.

## Defects by severity

| Severity | Result |
| --- | --- |
| Critical | None found |
| High | None found |
| Medium | None found |
| Low | None found |

## Verification limitation

Lighthouse `13.4.1` was attempted against live production with the preinstalled
Chrome for Testing `145.0.7632.6`, but Lighthouse could not connect to that
browser in this container, so no Lighthouse score is claimed. This is a test
environment/tooling limitation, not a product failure: direct browser checks,
axe, loading/error checks, live headers, PWA offline behavior, and byte budgets
all passed.
