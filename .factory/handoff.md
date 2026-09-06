# Handoff — review 1

## Release status: FAIL

**Reviewed implementation:** `7933c8eb95cd5fbbb2488edac9c60def935c6e52`
**Documentation HEAD:** `cf7025adb0a04618a3acc9d794044f532337f580`
**Live URL:** <https://supabase-exit-map.sociobot.in/>
**Date:** 2026-09-06 UTC

This was a review-only pass; no product code was changed. The complete result is in `.factory/review-1.md`. The verdict is FAIL: 6 findings (3 High, 3 Medium) and 19 untested public claims. The live Planning Room checkout link returns HTTP 404. The required CLI demo sandbox, claim inventory, social metadata, and real 404 page are also absent.

## Checks run

```sh
npm ci
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
```

All commands passed. The packed crate was installed into a clean temporary consumer root and its help, version, strict-empty, and missing-path behavior were exercised. `--demo` is not implemented (exit 2).

Fresh desktop and phone live-browser checks passed for the former offline, mobile-overflow, header, cache, keyboard, focus, axe, and normal-load privacy issues. `verify-url.sh` passed. Lighthouse could not connect to the available Chrome, so no score is claimed.

## Required repair before another review

1. Ship the documented isolated CLI demo and `--demo` sample flow.
2. Add and execute `.factory/claims.json` tests for every retained public claim.
3. Fix or remove the broken paid checkout link.
4. Correct the first screen, metadata, and real 404 route.
