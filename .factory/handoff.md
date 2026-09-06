# Handoff — repair 2

## Release status

Implementation SHA: d8a61fb3d596eae0953735dca6591d32cf481e5b
Documentation report SHA: 4178f6b082be196d6a922398473fbb430529aa07

The implementation is deployed at https://supabase-exit-map.sociobot.in/.

This repair closes the review findings in .factory/review-1.md:

- The installed CLI now supports --demo. It embeds a realistic Supabase project, copies it into a unique temporary directory, writes a report there, prints the paths, and keeps JSON valid on standard output.
- Both root examples/demo-project and the crate-packaged cli/examples/demo-project ship the sample input.
- The landing page first screen now states the job, audience, sample action, and privacy, offline, and price facts in plain words.
- /demo/ is an isolated one-click browser sandbox with eleven populated findings, the required persistent banner, Reset demo, Start for real, and a separate demo: local-storage key.
- .factory/claims.json declares 14 outcomes and each command passed from the documented setup.
- Canonical, Open Graph, Twitter, social image, and Apple touch metadata now ship on landing, demo, legal, and 404 routes.
- Static Web Apps now serves a designed 404.html for unknown paths with HTTP 404.
- The broken checkout URL is no longer linked. Planning Room remains a USD $19 one-time paid deliverable, but shows that checkout registration is pending.

## What the product does

Supabase Exit Map is for indie builders facing managed-backend costs. It inventories local Supabase configuration, migrations, and Edge Function source before an exit to Postgres or self-hosted components. It reports evidence, effort, replacement choices, and a cutover checklist. It does not migrate a live project.

## Verification

Clean setup and full suite:

~~~sh
npm ci
PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers npm test
npm run build
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo package --manifest-path cli/Cargo.toml
~~~

All passed. The full suite ran 6 Rust integration tests, one compiling doctest, TypeScript checks, 12 production-site and CLI outcome checks, and 8 browser checks with Axe WCAG 2.0/2.1/2.2 A/AA tags. No serious or critical Axe violations were found.

Every one of the 14 commands in .factory/claims.json was also run separately. They covered the isolated CLI demo, output formats, paths, error codes, no-credential path, proxy-recorded no-network scan, read-only inputs, offline reload, demo sandbox, no third-party demo requests, license restore fixture, browser-local plan fixture, and checkout status.

The ready-to-publish crate passed cargo package verification. Its tarball includes the embedded demo input. It was installed in a fresh temporary consumer root; --version, --help, and --demo --json worked, with eleven findings and a local-only report.

## Deployment and live checks

Deployment used /opt/fleet/lib/deploy-static.sh with the existing sf-supabase-exit-map Static Web App and its repository staticwebapp.config.json. It preserved the static product setup and deployed successfully as Azure deployment 91e44ef3-e1dc-45a1-84ed-63b80eaa4a22.

The live landing HTML, main JavaScript, and service worker hash-match dist/site. The landing and demo routes return 200. An unknown route returns HTTP 404, the Page not found title, and a home link.

verify-url.sh passed against production: HTTP 200, 694 ms measured load, title, lang, one H1, main landmark, image alt text, and zero console errors. Fresh desktop and phone Chromium contexts found:

- Job: Map Supabase dependencies before you migrate.
- Audience: indie builders facing managed-backend costs.
- First action: Try it with sample data.
- Phone document width: 390 px at a 390 px viewport.
- No serious or critical Axe violations and no console errors.

The live demo was entered through the first-screen action. It displayed eleven findings, kept the demo banner after Reset demo, wrote only the demo storage key, left the normal plan key absent, and made no third-party requests. A fresh live service-worker context reloaded the landing page offline with its H1 and zero errors.

Live assets return immutable cache headers. HTML, assets, and the service worker return CSP, frame protection, permissions policy, nosniff, and referrer policy headers. The landing includes canonical, Open Graph, Twitter, social image, and Apple touch metadata.

Performance bytes: initial JavaScript is 6,944 bytes plus a 711-byte shared chunk; CSS is 12,189 bytes; hero WebPs are 37,304 and 14,154 bytes; social WebP is 19,390 bytes. Lighthouse was attempted twice, once with CHROME_PATH and once through a manually started headless Chrome. The container could not attach to Chrome, and the manual browser tab crashed. No Lighthouse score is claimed.

## Billing dependency

The external endpoint https://api.sociobot.in/api/v1/products/supabase-exit-map/checkout still returns HTTP 404 with enabled factory product. Registration is owned by the separate billing operator and cannot be completed from this product repository. The paid Planning Room code, restore path, terms, USD $19 price, and public offer metadata remain in place. The site does not send visitors to that broken endpoint. Public registration metadata is at /work/.evidence/billing-offer.json.

## Evidence

Live evidence is in /work/.evidence/supabase-exit-map-repair-2, including desktop, phone, and demo screenshots; verify-url output; live HTML; 404 response; service-worker and bundle hashes; and the Lighthouse limitation logs. The catalog description is copied to /work/.evidence/catalog-description.txt.

## Known gaps and next steps

1. The billing-registration operator must register the stated Sociobot one-time offer. After registration, restore the canonical checkout link, exercise a real purchase-return, and verify entitlement against the live endpoint.
2. Lighthouse remains unmeasured in this container because Chrome could not be attached. Direct performance budgets, browser checks, and live cold load evidence passed.
