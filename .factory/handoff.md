# Handoff — Supabase Exit Map

## Verification status: FAIL

Candidate `cb65dced7262cd38b57c974a5d73f4a04ab2ff76` was independently verified on 2026-08-28 against https://supabase-exit-map.sociobot.in/.

The live HTML, JS, CSS, and service-worker bytes match the fresh production build exactly. The CLI's clean install, tests, build, formatting, Clippy, package verification, installed-consumer workflow, local-only scan, redaction, output formats, and exit-code handling all passed. Browser accessibility testing found zero axe violations at desktop and 390px mobile, and normal initial page loads made no third-party requests.

Release is **not approved**. See `.factory/verification.md` for exact commands and evidence. Required fixes:

1. Precache the built JS/CSS in the service worker: current offline reload produces JavaScript MIME console errors.
2. Fix 390px document overflow (494px observed) caused by install command blocks expanding outside their content column.
3. Make the deployment apply immutable cache headers for content-hashed assets; live assets currently have only `max-age=30, must-revalidate`.
4. Make live response headers include the intended frame/permissions protections and a suitable CSP.

## Verification commands

```sh
npm ci
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
cargo install --path cli --root /tmp/sem-consumer --force
```

Do not publish from this repository; `cargo package --manifest-path cli/Cargo.toml` is the ready-to-publish package check. No product source was modified during verification; this handoff and `.factory/verification.md` are the only pending changes.
