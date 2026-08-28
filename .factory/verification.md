# Verification — FAIL

**Candidate:** `cb65dced7262cd38b57c974a5d73f4a04ab2ff76` (`main`)  
**URL:** https://supabase-exit-map.sociobot.in/  
**Verified:** 2026-08-28 UTC  
**Scope:** independent clean-checkout QA of the Rust CLI, built static site, and live deployment. No product source was changed.

## Decision

**FAIL.** The CLI meets the core local-inventory job, but the shipped documentation PWA fails an offline reload, the required 390px layout has document-level horizontal overflow, and the live deployment does not apply the repository's immutable cache policy. These are present in the candidate that is live, not a stale-deployment issue.

## Reproducible defects

### High — offline PWA reload is not functional

1. Load the production build, wait for `sem-shell-v1` to activate, set the browser offline, then reload.
2. The HTML shell and H1 render, but JavaScript module requests are not precached. The service worker falls back to `/` (HTML) for them.
3. Chromium logs twice: `Failed to load module script: Expected a JavaScript-or-Wasm module script but the server responded with a MIME type of "text/html".`

`sw.js` precaches `/`, legal pages, the mobile hero, and favicon, but not the hashed JS/CSS assets. This contradicts the privacy page's claim that the guide works offline and fails the requested PWA offline-reload check.

### Medium — 390px viewport has horizontal page overflow

At a 390×844 viewport, `document.documentElement.scrollWidth` is **494px**. The two install command `.code-block` containers are 477.875px wide and extend from x=16 to x=493.875, rather than scrolling within the 358px content column. The command `pre` has `min-width: max-content`, expanding its CSS grid track. This fails the required mobile check and the design contract's “commands horizontally scrollable” intent.

### Medium — live hashed assets are revalidated every 30 seconds, not immutable

The candidate includes `site/public/_headers` declaring `Cache-Control: public, max-age=31536000, immutable` for assets, JS, and CSS. Fresh `curl -I` against the live deployment instead returns this for `/assets/main-CU0Ogf8H.js`, `/assets/style-BXcYgBsB.css`, and the WebPs:

```
cache-control: public, must-revalidate, max-age=30
```

This misses the stated static-product caching requirement and adds avoidable revalidation to every visit.

### Medium — live response-policy headers are incomplete

The live HTML/assets have HSTS, `Referrer-Policy: strict-origin-when-cross-origin`, and `X-Content-Type-Options: nosniff`, but do **not** send the candidate's configured `X-Frame-Options: DENY` or `Permissions-Policy`, and send no Content-Security-Policy. The `/_headers` file is therefore not being honoured as a deployment header configuration.

## Passing evidence

### Clean install, quality, package

From the clean candidate checkout:

```sh
npm ci
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
```

All commands passed. `npm test` passed 5 Rust integration tests, 1 compiling doctest, and 4 site tests. The exact build produced `dist/site/` and `target/release/supabase-exit-map`. `cargo package` verified the publishable crate (12 files, 51.4 KiB unpacked / 15.3 KiB compressed).

Built budgets: initial JS is 6,944 + 711 bytes uncompressed; CSS is 10,376 bytes; system fonts add 0 bytes; desktop/mobile hero WebPs are 37,304/14,154 bytes. All are within the byte budgets. A Lighthouse score is not reported: Lighthouse could not attach to the preinstalled Playwright Chromium in this container, so no score is claimed.

### CLI consumer and end-to-end behavior

`cargo install --path cli --root /tmp/sem-consumer-20260827 --force` installed a clean consumer binary. Its `--version` was `0.1.0`; `--help` documented all public flags and exit behavior.

An independent representative project containing config, Auth, Storage, Realtime, Edge Function, extension, RLS, trigger, cron, Vault, and `pg_net` signals produced version `1.0` JSON with ten findings and overall `high` effort. Markdown contained both “Open-source compatibility” and “Hosted-service boundary”. The Vault evidence was `select vault.create_secret([arguments redacted])`; neither test secret value appeared in JSON or Markdown. `live_project_queried` was `false`.

Boundary/recovery results:

- Complete supplied Supabase directory with `--strict --json`: exit 0.
- Empty directory with `--strict --json`: exit 3, zero findings, four actionable warnings.
- Missing path: exit 2.
- Invalid `--format`: exit 2.
- `--output` produced valid JSON and Markdown files.

### Browser, accessibility, privacy, and PWA evidence

Fresh Chromium exercised the built site at desktop 1440×1000 and mobile 390×844.

- Desktop: correct title, `lang=en`, exactly one H1, main landmark, loaded hero alt, visible 3px focus ring on the Skip link, keyboard Arrow navigation for tabs, and a useful short-token recovery message.
- Mobile: valid mocked license return stripped `?license` from the URL, saved the token locally, unlocked Planning Room, imported the example, created 3 rows, and saved a browser-local plan. Reduced motion changed animation and transition durations to `0.01ms`.
- `@axe-core/playwright` with WCAG 2 A/AA, 2.1 AA, and 2.2 AA tags found **zero serious or critical findings** (zero violations at both viewports).
- A normal first load made no third-party requests. With an explicit valid license, the only runtime external request was the documented `GET https://api.sociobot.in/api/v1/products/supabase-exit-map/verify?...`; no report data was sent. Static review found no analytics, trackers, CDN scripts, or remote fonts.
- The service worker activated and cache `sem-shell-v1` existed; its offline reload failure is recorded above.

### Live deployment identity

Fresh hashes confirm the live site equals the candidate production build:

| File | SHA-256 |
| --- | --- |
| `index.html` | `668441980ca778726f513ec4befb0a7494c326cd9a2f0a1be98a55e83a145111` |
| `assets/main-CU0Ogf8H.js` | `9bb085c2ff1c12e822453454c4165ed3ef08f9b23dad66a2086f164ae1392b62` |
| `assets/style-BXcYgBsB.css` | `a039040b2288f47596ca73293172dc9c0a78cf3ba8b7e557d8b8f8ac2c8acb9b` |
| `sw.js` | `f412d12a81d10c4f140fe11500d5f9bcacbe147f9c015af959a3887c9ab9f1ed` |

## Required next steps

1. Version the service-worker cache and precache the built JS/CSS (or use a generated precache manifest); retest an offline reload with zero console errors.
2. Constrain the mobile command blocks to the content width and retain overflow on the block itself; retest `scrollWidth === 390`.
3. Configure the deployment to honour immutable caching for content-hashed assets and send the intended frame/permissions headers; add a CSP appropriate for the static site and Sociobot license endpoint.
4. Rebuild and redeploy, then perform a new verification report against the new commit and live hashes.
