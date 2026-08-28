# Handoff — independent verification

## Release status: PASS

**Verified candidate:** `f6a57d2399dd0e792696778f7011e1ff45217ba3`
**Verified URL:** <https://supabase-exit-map.sociobot.in/>
**Date:** 2026-08-28 UTC

The clean candidate build, Rust CLI, packed consumer install, static site/PWA,
and live deployment all passed independent QA. No product-source files were
changed during verification.

## Run locally

```sh
npm ci
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
```

`npm test` passes 5 Rust integration tests, 1 doctest, TypeScript checking,
6 built-site checks, and 4 Chromium/axe tests. `npm run build` produces
`dist/site/` and `target/release/supabase-exit-map`. The packed crate verifies
and can be installed into a clean consumer with:

```sh
cargo install --path <unpacked-supabase-exit-map-0.1.0> --root <temporary-root>
```

## Evidence and known gaps

The live HTML, service worker, JavaScript, CSS, legal asset, and hero hash-match
the candidate production output. At 390px there is no document overflow; axe
has zero serious/critical findings at desktop and mobile; keyboard, focus,
reduced motion, license recovery, privacy/no-tracker behavior, response
headers, cache policy, and PWA offline reload all passed.

No Critical, High, Medium, or Low product defects were found. Lighthouse was
attempted but could not connect to the preinstalled Chrome for Testing in this
container, so no score is claimed; direct browser and byte-budget checks passed.

See `.factory/verification-2.md` for full commands, exact hashes, test cases,
response headers, and the verification-tooling limitation.
