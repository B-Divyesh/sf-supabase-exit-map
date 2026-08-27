# Handoff — Supabase Exit Map v0.1.0

## What shipped

- A Rust single-binary CLI that recursively scans local Supabase config, SQL migrations/seeds, and TypeScript/JavaScript Edge Functions without credentials or network access.
- Findings for Auth, Storage, Realtime, Edge Functions, Postgres extensions, RLS/claim helpers, routines/triggers, cron, Vault, and outbound database networking. Every finding includes file/line evidence, portability, effort, open-source compatibility, hosted-service boundaries, replacement choices, and next actions.
- Terminal, versioned JSON, and issue-ready Markdown output; `--output`; useful `--help`; stable exit codes; and `--strict` for warning-sensitive CI.
- Secret-aware evidence redaction and explicit analysis limitations/manual checks for dashboard-only and live state.
- A responsive Vite landing/docs site in the required glacial minimal ceramics direction, with an original generated hero, live sample report filters, install/usage docs, dark mode, offline shell, privacy notice, terms, and no analytics or third-party runtime assets.
- A $19 one-time Planning Room unlock using the Sociobot billing contract. The full CLI and all core exports remain free. The unlock accepts return/pasted tokens, strips tokens from URLs, verifies against the slug endpoint, caches verdicts for at most one day, reconciles offline state, and adds local JSON import, replacement/owner/status planning, saving, and printing.

## Run and verify

Requirements: Rust 1.78+, Node.js 20+.

```sh
npm install
npm test
npm run build
```

`npm run build` is the work-order build command. It produces the deployable static root at `dist/site/` (including `dist/site/index.html`) and the release binary at `target/release/supabase-exit-map`.

Useful checks performed on 2026-08-27:

- `npm test` — passed: 5 Rust integration tests, 1 compiling doctest, and 4 site contract tests.
- `npm run build` — passed from installed lockfiles; Vite and release Rust builds completed.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo package --manifest-path cli/Cargo.toml --allow-dirty` — packaged and verified 12 files; 51.4 KiB unpacked / 15.3 KiB compressed. After checkout, use the same command without `--allow-dirty`; do not publish from the worker.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:5173 /tmp/sem-evidence-2` — HTTP 200, 621 ms local load, no console/page errors, title/lang/one h1/main/alt/button names all passed at desktop and 390×844.
- Playwright + axe-core WCAG 2 A/AA, 2.1 AA, and 2.2 AA scan at 390×844 — zero violations.
- Lighthouse mobile against the production build — performance 100, accessibility 100, best practices 100, SEO 100; FCP 1.0 s, LCP 1.0 s, CLS 0, TBT 0 ms.
- Browser flow with a mocked Sociobot verification response — license query stripped, token stored, one verify request across reloads within a day, Planning Room unlocked, 3 example rows created, and plan saved locally.

Production asset budgets:

- Initial app JS: 6.94 KiB plus 0.71 KiB shared (uncompressed), below 200 KiB.
- CSS: 10.38 KiB uncompressed, below 50 KiB.
- Fonts: 0 bytes; system stacks only.
- Hero: 40 KiB desktop / 16 KiB mobile WebP, explicit 3:2 dimensions, below 300 KiB.
- Release binary: 1.3 MiB on this Linux build.

## Known boundaries and next steps

- Static local analysis cannot see dashboard-only settings, live row counts, object bytes, active sessions, remote-only functions, or dynamic SQL. Reports state this and include read-only manual verification steps; adding a credentialed live mode is intentionally out of scope.
- The factory still needs to register `supabase-exit-map` with Sociobot billing before checkout/verification can succeed in production. No product ID, provider SDK, infrastructure, DNS, secret, or billing configuration was added here.
- Release archives and registry publication are factory responsibilities. The crate is ready for `cargo package`; no package was published.
- Lighthouse numbers are local production-build measurements and can vary on deployed infrastructure.
- Supabase is a trademark of Supabase, Inc.; the site clearly states that the product is independent and not endorsed by Supabase.
