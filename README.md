# Supabase Exit Map

Supabase Exit Map is a local CLI for indie builders planning a move away from Supabase. It scans project config, SQL migrations, and Edge Function source. The report lists dependencies, evidence, effort, replacement choices, and a cutover checklist.

The scanner does not make network requests or require a Supabase credential.

It does not migrate a live project, host replacements, or copy authentication data.

## Try the bundled demo

The demo needs no account or Supabase credential. It copies the shipped sample project into a new temporary directory, writes a report there, and prints the report path.

~~~sh
cargo run --bin supabase-exit-map -- --demo --format markdown
~~~

The sample input is in [examples/demo-project](examples/demo-project). The website demo is available at [https://supabase-exit-map.sociobot.in/demo/](https://supabase-exit-map.sociobot.in/demo/).

## Install

From source, with Rust 1.78 or newer:

~~~sh
cargo install --path cli
~~~

## Use

Scan a project and print a readable report:

~~~sh
supabase-exit-map ./my-project
~~~

The path can be a project root containing a supabase directory or the supabase directory itself.

Write a Markdown plan:

~~~sh
supabase-exit-map ./my-project --format markdown --output exit-plan.md
~~~

Write JSON for another planning tool:

~~~sh
supabase-exit-map ./my-project --json --output exit-map.json
~~~

Use strict mode in CI. It exits with code 3 if the scan has warnings.

~~~sh
supabase-exit-map . --strict --json
~~~

Exit code 2 means invalid arguments or an input path problem. Run supabase-exit-map --help for the full CLI reference.

## What the scan reports

The scanner recognizes local signals for:

- Postgres extensions, policies, functions, triggers, and scheduled jobs
- Auth configuration and auth-schema references
- Storage buckets and policies
- Realtime publications
- Edge Functions, environment access, and outbound calls
- Vault and pg_net

Every report labels the scan as local-only with live_project_queried set to false. Static analysis cannot see dashboard settings, dynamic SQL, live row counts, object bytes, or active sessions. Use the report’s manual checklist before a cutover.

## Planning Room

Planning Room is an optional USD $19 one-time license for local JSON import, assignment, status tracking, device-local saving, and printing. Checkout registration is pending, so purchases cannot be made yet. A previously issued license can still be restored from the site. See [Terms](https://supabase-exit-map.sociobot.in/terms/) and [Privacy](https://supabase-exit-map.sociobot.in/privacy/).

## Develop and verify

Requirements: Node.js 20+ and Rust 1.78+.

~~~sh
npm ci
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm test
npm run build
~~~

Outside the factory environment, install Chromium once:

~~~sh
npx playwright install chromium
~~~

npm test runs Rust unit and integration tests, TypeScript checks, production-site checks, browser checks, accessibility checks, and every claim test. npm run build creates dist/site/ and target/release/supabase-exit-map.

Run the site locally with npm run dev. Build only the deployable site with npm run build:site.

Create a ready-to-publish package without publishing:

~~~sh
cargo package --manifest-path cli/Cargo.toml
~~~

## Deploy

The factory deploys the static site from dist/site/. Build it with npm ci && npm run build:site. The repository includes the Static Web Apps cache, security, and designed 404 configuration.

## License

[MIT](LICENSE)
