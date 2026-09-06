# Handoff — verification 4

## Release status

**Verification verdict: FAIL.** The live implementation was not changed.

- Implementation reviewed: `d8a61fb3d596eae0953735dca6591d32cf481e5b`
- Documentation reviewed: `355dfe9075cd3d6f5c25a2cef4747bbc2e7e7aee`
- Live URL: <https://supabase-exit-map.sociobot.in/>

The live build hash-matches the implementation candidate. The verification
report is `.factory/verification-4.md`.

## What was verified

From a fresh clone, `npm ci`, `npm test`, `npm run build`, Rust formatting,
Clippy, and `cargo package` all passed. Every one of the 14 declared claim
commands was run separately and passed. The packed crate was installed into a
fresh consumer root; its help, version, and isolated `--demo --json` workflow
worked with eleven findings and a local report.

Fresh desktop and 390px phone visits confirmed the job, audience, and sample
action before scrolling. The live demo has the required label, reset control,
eleven realistic findings, and separate demo storage. Legal routes, metadata,
designed HTTP 404, offline reload, reduced motion, light/dark Axe checks,
security headers, immutable assets, and linked routes were checked.

## Known gaps / next steps

1. Make **Start for real** remove the `demo:supabase-exit-map:` storage key
   before leaving `/demo/`, and add it to the demo sandbox claim.
2. Make all mobile tap targets at least 44x44 CSS px, including header and
   footer links.
3. Replace the raw invalid-JSON parser message with a plain recovery message.
4. Add claim coverage for the landing/privacy no-third-party promise and for
   printing a completed Planning Room plan, or narrow/remove those promises.
5. Re-run independent verification after those repairs. No Lighthouse score
   is recorded because the container’s attachment limitation persists.
