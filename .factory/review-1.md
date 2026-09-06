# Review 1 — plan a Supabase exit

**Verdict: FAIL.** Six findings remain: three High and three Medium. There is no claim inventory, so **19 public, testable claims are untested**. This is not a PASS.

**Reviewed:** 2026-09-06 UTC
**Live URL:** <https://supabase-exit-map.sociobot.in/>
**Implementation reviewed:** `7933c8eb95cd5fbbb2488edac9c60def935c6e52` (`fix: harden static site release`)
**Documentation HEAD:** `cf7025adb0a04618a3acc9d794044f532337f580`

`f6a57d2` and `cf7025a` are documentation-only commits. The current source build is therefore the `7933c8e` implementation, and it hash-matches live for the landing HTML, service worker, all emitted JS/CSS, hero image, and both legal pages.

## What a visitor sees first

- **Job:** make a local inventory of Supabase dependencies before planning an exit. This is only inferred after reading the lede; the H1 does not name it.
- **Audience:** not stated on the first screen. The brief identifies indie builders facing managed-backend price thresholds.
- **First action:** “Install the CLI”. The other action only scrolls to static example output; it does not run a sample.

## Findings

| Severity | Finding | Evidence and required correction |
| --- | --- | --- |
| High | No one-click CLI demo sandbox exists. | The packaged CLI installs and runs, but `supabase-exit-map --demo` exits 2: `unexpected argument '--demo'`. There is no `examples/` directory. `/demo` returns 200 but is the ordinary landing page (same title and H1), with no sample state, persistent “Demo — sample data, nothing is saved” label, Reset demo, or Start for real action. The visible “Read a sample map” link is static output, not a try-out. Ship a bundled realistic sample and `--demo` command, plus the required terminal recording and documented isolated demo path. |
| High | Required claim inventory and executable claim tests are missing. | `.factory/claims.json` does not exist. `package.json` has no `@claim:<id>` commands, so no declared claim command could be run from a clean checkout. The 19 untested visible promises are: local-only scan; no Supabase contact; no live auth/user-data read; config/migration/function inspection; dependency/checklist/replacement/effort output; documented exit-code behavior; no credentials; no telemetry; root-or-`supabase/` path support; “in seconds”; “ten minutes”; no site analytics; offline guide; browser-local license/plan storage; imported report not uploaded; daily license verification; local plan import/selection/save/print; free exports; and future Planning Room updates. Add one observable sandbox test per claim, or remove claims that cannot be proved. |
| High | The paid checkout path is broken. | On 2026-09-06, `HEAD https://api.sociobot.in/api/v1/products/supabase-exit-map/checkout` returned **404**. The live “Buy Planning Room” link points to that exact URL. Register/configure the product checkout or remove the paid offer until it works; then test the real link and a license-return recovery path. |
| Medium | The first screen does not meet the plain-words entry contract. | The H1 is “Know what has to move before you move it.” It does not state the CLI inventory job. The target audience is absent, and the required one-click sample action is absent. The lede is 27 words, exceeding the 22-word limit. The screen also does not present the required three short privacy/offline/price facts. Rewrite the screen around the job, named audience, sample action, and three factual lines. |
| Medium | Required document metadata is absent. | Fresh desktop and phone DOM checks found zero canonical links, zero Open Graph properties, and zero Twitter-card properties. The landing source also lacks an Apple touch icon. Add route-specific canonical/OG/Twitter metadata and a product-derived 1200×630 social image plus the touch icon. |
| Medium | There is no real 404 page. | `GET /404` returns **200** and the normal landing title/H1; `staticwebapp.config.json` has no `responseOverrides` 404 rewrite and no designed 404 document. Add a product-styled 404 with a route back, return HTTP 404 for unknown paths, and include the route in the site checks. |

## Claim-command result

No claim commands were declared. The required “run every declared claim command” check is therefore **not applicable**, but this is itself the High finding above. `untested_claim_count` is 19, not zero.

## Checks that passed

From this checkout, after `npm ci`:

```sh
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
```

All passed. The package was installed from `target/package/` into a clean temporary consumer root. Its installed `--version` was `0.1.0`, `--help` documented the public surface, an empty project with `--strict --json` exited 3, and a missing path exited 2.

Fresh live Chromium contexts at 1440×1000 and 390×844 found one H1, `lang=en`, a main landmark, no page/console errors, 390px document width on phone, no serious/critical axe WCAG 2.0/2.1/2.2 A/AA violations, and only same-origin requests during a normal load. Arrow-key tabs worked, the Skip link had a 3px focus outline, and a service-worker-controlled offline reload kept the H1 with zero console errors. `verify-url.sh` passed (HTTP 200, 571ms, title/lang/main/alt checks, no console errors). Reduced-motion behavior and local browser tests passed under `npm test`.

Lighthouse was attempted with the installed Playwright Chrome using `npm exec --yes lighthouse`; it again failed with “Unable to connect to Chrome”. No Lighthouse score is claimed.

## Earlier-review disposition

| Earlier item | Current disposition |
| --- | --- |
| Offline reload returned HTML for JS modules | Fixed. The local release browser test passed; current live offline reload also rendered the H1 without module/MIME or console errors. |
| 390px command blocks caused horizontal page overflow | Fixed. Fresh live phone width was exactly 390px; the local mobile overflow test passed. |
| Hashed assets revalidated after 30 seconds | Fixed. Live hashed JS returned `Cache-Control: public, max-age=31536000, immutable`. |
| CSP, frame, and permissions response headers were absent | Fixed. Live HTML and JS returned the configured CSP, `X-Frame-Options: DENY`, permissions policy, referrer policy, and `nosniff`. |
| Lighthouse could not attach in the verifier container | Still a measurement limitation. The same attempt failed in this review; direct browser, axe, byte-budget, and runtime checks passed, but no Lighthouse score is available. |

## Next steps

1. Build the required CLI demo sandbox and test it from a clean consumer environment.
2. Add `.factory/claims.json` and one tagged, observable test for every retained public promise.
3. Repair the paid checkout registration and verify purchase-return behavior.
4. Repair the first screen, metadata, and real 404 route; rebuild and request a new independent review.
