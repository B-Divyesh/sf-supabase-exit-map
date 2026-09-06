# Verification 4 — FAIL

**Verdict: FAIL.** There are four findings and two untested public claims. The
live product is not eligible for PASS until each is corrected and independently
verified.

## Scope

- **Live URL:** <https://supabase-exit-map.sociobot.in/>
- **Verified:** 2026-09-06 UTC
- **Implementation candidate:** `d8a61fb3d596eae0953735dca6591d32cf481e5b`
  (`fix: add isolated CLI demo and verified site routes`)
- **Documentation candidate:** `355dfe9075cd3d6f5c25a2cef4747bbc2e7e7aee`
  (`docs: record handoff report revision`)

The later commits after `d8a61fb` are documentation-only. A production build
from the clean documentation candidate hash-matched live `index.html`, all
route documents, `sw.js`, and the emitted JavaScript and CSS modules.

## What appears first

Fresh desktop (1440x1000) and phone (390x844) Chromium contexts showed the
same first screen before scrolling:

- **Job:** “Map Supabase dependencies before you migrate.”
- **Audience:** “For indie builders facing managed-backend costs who need a
  safe exit plan.”
- **First action:** “Try it with sample data,” which opens `/demo/` and states
  that it runs the bundled project and shows a populated report.

This satisfies the first-screen plain-words entry requirement.

## Findings

| Severity | Finding | Reproduction and required correction |
| --- | --- | --- |
| Medium | Leaving the browser demo does not discard demo storage. | In a fresh live context, open `/demo/`, then activate **Start for real**. The browser returns to `/#install`, but `localStorage` still contains `demo:supabase-exit-map:sample-v1`. The demo does not alter normal `sb_` keys, but the sandbox contract requires leaving demo mode to discard its data (or explicitly offer to keep it). Remove only the `demo:` namespace before navigation, and cover this path in the demo claim. |
| Medium | Several mobile links have tap boxes smaller than the required 44x44 CSS px. | At 390px, the header Home wordmark is 212x26, Demo is 45x23, and footer links such as Privacy and Terms are 51x22 and 41x22. The same applies to visible inline action links. Give link controls adequate padding/minimum target areas without reducing the readable layout, then test at phone width. |
| Low | Invalid Planning Room report JSON gives a parser message without a next step. | With a recorded valid license response, unlock Planning Room, enter `{not json`, and choose **Build planning table**. The status says `Expected property name or '}' in JSON at position 1...`; it does not say that a valid Exit Map JSON report is required or what to do next. Replace this with a short actionable recovery message while retaining useful detail if appropriate. |
| Medium | Two public claims are not completely covered by declared sandbox tests. | `site-privacy` only records requests on `/demo/`, while its `where` field includes the landing privacy fact and the privacy notice’s site-wide no-analytics/no-third-party statement; it does not prove those pages make no third-party requests. Separately, the landing, README, and terms say Planning Room can print a completed plan, but no `@claim:` test invokes or observes `window.print`. Add separate observable sandbox tests (or remove/narrow the public copy). |

**Finding count: 4. Untested public-claim count: 2.**

## Clean-checkout commands

A new clone at `355dfe9` was used. `npm ci` completed with no reported
vulnerabilities. The following all passed:

```sh
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
```

`npm test` passed six Rust integration tests, one compiling doctest,
TypeScript checking, twelve built-site/CLI tests, and eight Chromium tests.
`npm run build` produced `dist/site/` and the release binary. `cargo package`
verified a 15-file crate (56.5 KiB unpacked, 16.8 KiB compressed).

Every command declared in `.factory/claims.json` was run separately and
passed: `cli-demo`, `dependency-inventory`, `cli-paths`, `cli-output`,
`cli-errors`, `local-scan`, `no-cli-network`, `read-only-scan`,
`offline-reload`, `demo-sandbox`, `site-privacy`, `license-restore`,
`local-planning`, and `planning-price`. Their passing status does not remove
the two coverage gaps above.

The packaged crate was installed into a separate temporary consumer root. Its
installed binary reported `0.1.0`, provided help for all public flags, and
`--demo --json` produced eleven findings, four high- and six medium-effort
items, a local-only scope (`live_project_queried: false`), and a temporary
report path.

## Live product checks

- The landing and demo had no console or page errors and only same-origin
  requests in fresh desktop and phone contexts. Phone document width was
  exactly 390px at a 390px viewport.
- The first-screen action opened a populated `/demo/` with eleven realistic
  findings, the persistent “Demo — sample data, nothing is saved” label,
  Reset demo, and Start for real. Reset retained a newly seeded `demo:` key
  and wrote no normal plan key. The leaving-demo defect is recorded above.
- A service-worker-controlled fresh context reloaded the landing page offline
  with its H1 and no errors. Reduced motion set transition and animation
  durations to 0.01ms. The offline notice and normal online recovery path are
  present.
- Axe WCAG 2 A/AA, 2.1 AA, and 2.2 AA found no violations at desktop, phone,
  or explicit dark theme. The mobile target-size finding is a separate
  contract check, not an Axe result.
- A mocked valid license return stripped the query token, stored it locally,
  and unlocked Planning Room. A mocked invalid response relocked it with an
  actionable message. A short token was rejected with an actionable message.
  Invalid report JSON produced the low-severity recovery defect above.
- `/privacy/` and `/terms/` returned 200 with route-specific titles and one
  main/H1. An unknown route returned the designed Page not found document with
  HTTP 404 and a home link; that deliberate 404 is not a defect. All linked
  internal pages and the source repository link returned 200, apart from the
  expected unknown-route 404.
- `verify-url.sh` passed against production: HTTP 200, 584ms measured load,
  title, `lang=en`, one H1, main landmark, complete image alt text, and zero
  console errors.
- Live hashed JS, CSS, WebP, and service-worker assets send
  `Cache-Control: public, max-age=31536000, immutable`, CSP, frame denial,
  permissions policy, `nosniff`, and referrer policy headers. Initial JS is
  6,944 bytes plus a 711-byte shared module; CSS is 12,189 bytes.

## Earlier findings

| Earlier finding | Current disposition |
| --- | --- |
| No CLI/browser one-click demo | Fixed: the installed `--demo` artifact and `/demo/` both use a realistic eleven-finding sample. The new leave-demo cleanup defect remains. |
| No claims inventory | Improved: 14 declared commands run and pass. Two public promises still lack complete observable coverage. |
| Broken checkout link | Fixed as a user path: checkout registration is explicitly pending and no broken checkout link is published. |
| First screen lacked job, audience, and sample action | Fixed. |
| Missing canonical/social/touch metadata | Fixed on the reviewed routes. |
| No real 404 | Fixed: live unknown route is designed and returns HTTP 404. |
| Offline reload, mobile overflow, asset caching, and response headers | Fixed and rechecked live. |
| Lighthouse attachment limitation | No Lighthouse score is claimed. Direct browser, Axe, byte-budget, and loading checks were completed. |

## Evidence

Screenshots and `verify-url.sh` output are in
`/work/.evidence/supabase-exit-map-verify-4/`, including fresh desktop,
phone, populated-demo, reset-demo, and verifier captures.
