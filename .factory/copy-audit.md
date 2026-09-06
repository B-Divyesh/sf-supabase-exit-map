# Landing copy audit

Audited 2026-09-06. Counts use visible words. Navigation labels, commands, table labels, and headings are included because screen-reader users encounter them. No visible sentence exceeds 22 words. No banned plain-words term appears.

| Visible copy | Words | Result |
| --- | ---: | --- |
| Skip to main content | 4 | pass |
| You’re offline. | 2 | pass |
| The guide is available. | 4 | pass |
| License checks resume when you reconnect. | 6 | pass |
| Supabase Exit Map | 3 | pass |
| Demo | 1 | pass |
| Install | 1 | pass |
| Sample report | 2 | pass |
| Privacy | 1 | pass |
| Local CLI v0.1.0 | 4 | pass |
| Map Supabase dependencies before you migrate | 6 | pass |
| For indie builders facing managed-backend costs who need a safe exit plan. | 12 | pass |
| Try it with sample data | 5 | pass |
| Runs the bundled project and shows a populated dependency report. | 10 | pass |
| Privacy: no site analytics. | 4 | pass |
| Offline: the guide works after its first visit. | 8 | pass |
| Price: the CLI is free; Planning Room is $19 once when checkout opens. | 13 | pass |
| Inspect each dependency before choosing a target. | 7 | pass |
| How it works | 3 | pass |
| Inspect local project files | 4 | pass |
| The CLI reads Supabase config, SQL migrations, and Edge Function source from the path you choose. | 16 | pass |
| Run the scan | 3 | pass |
| Point the CLI at a project root or its supabase directory. | 11 | pass |
| Review each dependency | 3 | pass |
| See file evidence, effort, replacement choices, and hosted-service limits. | 9 | pass |
| Plan the work | 3 | pass |
| Use the checklist before a restore test or cutover rehearsal. | 10 | pass |
| Install and run | 3 | pass |
| Run the CLI on your project | 6 | pass |
| Install from source, then write a Markdown or JSON report for your planning work. | 14 | pass |
| Install from source | 3 | pass |
| Install the CLI from source on a machine with Rust. | 10 | pass |
| Copy | 1 | pass |
| cargo install --git https://github.com/B-Divyesh/sf-supabase-exit-map | 3 | pass |
| Run the bundled sample | 4 | pass |
| --demo copies the sample into a new temporary directory and prints the report path. | 14 | pass |
| supabase-exit-map --demo --format markdown | 5 | pass |
| The recording uses the bundled sample. | 6 | pass |
| Read the same output on the interactive demo page. | 9 | pass |
| Review populated findings | 3 | pass |
| The sample identifies dependencies that change the order and cost of an exit. | 13 | pass |
| All 6; Database 2; Managed edges 4 | 7 | pass |
| Authentication | 1 | pass |
| High effort; replace | 3 | pass |
| Evidence config.toml:18 enables GitHub login. | 7 | pass |
| List providers, sessions, redirects, and mail. | 6 | pass |
| Choose an auth service or application auth. | 7 | pass |
| Object storage | 2 | pass |
| Evidence files.sql:4 creates a bucket and policies. | 9 | pass |
| Count and hash objects with read-only access. | 7 | pass |
| Test signed URLs and restore behavior. | 6 | pass |
| Realtime | 1 | pass |
| Evidence notes.sql:39 adds a publication. | 7 | pass |
| Measure fan-out and connection count. | 5 | pass |
| Test a replacement websocket route. | 5 | pass |
| Edge Function | 2 | pass |
| Evidence send-mail/index.ts:1 uses secrets and outbound fetch. | 9 | pass |
| Record secret names and rotate values. | 6 | pass |
| Add retry and timeout tests. | 5 | pass |
| Row-level security | 2 | pass |
| Evidence: policies call auth.uid(). | 5 | pass |
| Keep native Postgres RLS. | 4 | pass |
| Replace Supabase claim shapes and roles. | 6 | pass |
| Functions and triggers | 3 | pass |
| Evidence: two routines and one trigger use PL/pgSQL. | 8 | pass |
| Restore SQL in dependency order. | 5 | pass |
| Audit owners and search paths. | 5 | pass |
| What self-hosting does not provide | 5 | pass |
| Open-source compatibility does not include managed backups, scaling, security response, or support. | 12 | pass |
| Limits and privacy | 3 | pass |
| What the CLI does not do | 6 | pass |
| It helps you plan an exit. | 7 | pass |
| It does not migrate a live project or copy authentication data. | 11 | pass |
| Check the live system separately | 5 | pass |
| Dashboard settings, dynamic SQL, data size, sessions, and object bytes need read-only checks before cutover. | 15 | pass |
| Planning Room | 2 | pass |
| Assign exit work locally | 4 | pass |
| $19 one time | 3 | pass |
| Planning Room adds local report import, replacement choices, owners, statuses, and a printable cutover sheet. | 15 | pass |
| Import an Exit Map JSON report. | 6 | pass |
| Assign a replacement, owner, and status. | 6 | pass |
| Save the plan in this browser. | 6 | pass |
| Print the completed plan. | 4 | pass |
| Checkout registration is pending. | 4 | pass |
| A paid license cannot be purchased yet. | 7 | pass |
| Restore a purchase | 3 | pass |
| License token | 2 | pass |
| Paste a token from your receipt. | 6 | pass |
| It stays on this device. | 5 | pass |
| Verify license | 2 | pass |
| No license saved. | 3 | pass |
| Sociobot and Dodo are the merchant of record. | 8 | pass |
| Read our privacy notice and terms. | 7 | pass |
| Local plan | 2 | pass |
| Assign each dependency | 3 | pass |
| Import a CLI JSON report, make choices, then print or save the plan in this browser. | 16 | pass |
| Exit Map JSON report | 4 | pass |
| Build planning table | 3 | pass |
| Load example | 2 | pass |
| Paste a report or load the example. | 7 | pass |
| Save on this device | 4 | pass |
| Print plan | 2 | pass |
| Dependency; Replacement; Owner; Status | 4 | pass |
| Local Supabase dependency planning. | 4 | pass |
| Version 0.1.0. | 2 | pass |
| Source code; Privacy; Terms | 4 | pass |
| Built by Param Factory | 4 | pass |

## Terminology

| Concept | One term used |
| --- | --- |
| Files examined by the tool | local project files |
| Tool result | report |
| Report item | dependency |
| Migration planning workspace | Planning Room |
| Interactive sample | demo |
| Paid token | license |
| New target option | replacement |
| Moving from Supabase | exit |

## Flags

None. The longest visible sentence has 16 words. Commands and source URLs are retained as literal user instructions.
