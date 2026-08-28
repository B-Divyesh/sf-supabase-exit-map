# Supabase Exit Map

Supabase Exit Map is a local-only CLI for builders deciding how to leave a managed Supabase project. It inspects project config, SQL migrations, and Edge Function source to produce a dependency inventory, portability checklist, replacement choices, and effort flags. It never contacts Supabase and never reads live user or auth data.

## Install

From a release archive, put the `supabase-exit-map` binary on your `PATH`. From source:

```sh
cargo install --path cli
```

## Usage

Scan a project and print the readable report:

```sh
supabase-exit-map ./my-supabase-project
```

Write a Markdown plan for your issue tracker:

```sh
supabase-exit-map ./my-supabase-project --format markdown --output exit-plan.md
```

Feed a stable, versioned shape into other tools:

```sh
supabase-exit-map ./my-supabase-project --json > exit-map.json
```

Treat incomplete or unreadable project inputs as a CI failure:

```sh
supabase-exit-map . --strict --json
```

The CLI scans `supabase/config.toml` (or `config.toml` when the supplied path is already the Supabase directory), `migrations/**/*.sql`, `functions/**/*.{ts,js,tsx,jsx}`, and common seed SQL. It intentionally does not connect to a live project. Dynamic SQL and dashboard-only settings cannot be inferred, so the report always includes manual verification steps.

Exit codes are `0` for a complete scan, `2` for invalid arguments/input paths, and `3` when `--strict` finds scan warnings. Run `supabase-exit-map --help` for every option.

## What it recognizes

- Postgres extensions, row-level security policies, functions, triggers, scheduled jobs, and managed schemas
- Auth references and provider configuration
- Storage buckets/policies and object API usage
- Realtime publications and client channels
- Edge Functions, secret access, Deno/npm imports, and network calls
- Platform-specific signals such as Vault, `pg_net`, and Supabase client calls

Every finding carries evidence, portability, effort, a plain-Postgres or self-hosted replacement route, and concrete next actions. Hosted service guarantees are described separately from open-source component compatibility.

## Develop and verify

Requirements: Rust 1.78+ and Node.js 20+.

```sh
npm ci
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm test
npm run build
```

`npm test` runs Rust unit/integration tests, strict TypeScript checking, built-site checks, and Chromium browser/axe tests. Outside the factory environment, install the browser once with `npx playwright install chromium`. `npm run build` creates the static site in `dist/site/` and a release CLI in `target/release/`. Run the site locally with `npm run dev`; build only the deployable site with `npm run build:site`.

Create the ready-to-publish Rust package without publishing it:

```sh
cargo package --manifest-path cli/Cargo.toml
```

## Deploy

The factory deploys this as a static Azure Static Web Apps artifact. Build `dist/site/` with `npm ci && npm run build:site`; `site/public/staticwebapp.config.json` is copied into that artifact and supplies the cache and response-security policy. The factory owns deployment credentials, so do not publish or deploy it manually outside the work order.

## Privacy and limits

Scanning is local and telemetry-free. The documentation site has no analytics. The optional one-time Planning Room license is stored in your browser and verified against Sociobot at most once per day; see `/privacy` and `/terms` on the site. Supabase is a trademark of Supabase, Inc.; this independent tool is not affiliated with or endorsed by Supabase.

## License

[MIT](LICENSE)
