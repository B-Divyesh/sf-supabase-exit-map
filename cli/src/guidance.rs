use crate::model::{Category, Effort, Finding, Portability};

pub(crate) fn finding(id: &str, name: &str) -> Finding {
    let (category, portability, effort, compatibility, guarantee, replacements, steps) = match id {
        "auth.config" | "auth.schema" => (
            Category::Auth, Portability::Replace, Effort::High,
            "Supabase Auth is open source and can be self-hosted, but application sessions, provider settings, hooks, and mail delivery must move together.",
            "Hosted availability, abuse controls, OAuth secrets, SMTP delivery, and operational recovery do not follow a database dump.",
            vec!["Self-hosted Supabase Auth (GoTrue)", "Keycloak", "Lucia-compatible application auth"],
            vec!["Inventory providers, redirect URLs, templates, hooks, and MFA", "Design a supported user/session transfer procedure", "Run dual-environment login and rollback tests"],
        ),
        "storage" => (
            Category::Storage, Portability::Replace, Effort::High,
            "Storage metadata and policies are Postgres objects; object bytes live outside the database.",
            "Managed object durability, CDN behavior, signed URL semantics, image transforms, and backups are service guarantees, not SQL portability.",
            vec!["S3-compatible storage (MinIO)", "Cloudflare R2", "AWS S3"],
            vec!["Export bucket inventory and object counts with read-only tools", "Copy objects and verify hashes", "Port policy and signed-URL behavior before cutover"],
        ),
        "realtime" => (
            Category::Realtime, Portability::Replace, Effort::Medium,
            "Postgres logical replication is portable; Supabase channel authorization and client protocol require a compatible server.",
            "Managed fan-out capacity, websocket uptime, and replication-slot monitoring are not provided by plain Postgres.",
            vec!["Self-hosted Supabase Realtime", "Postgres LISTEN/NOTIFY with an application websocket", "ElectricSQL"],
            vec!["List subscribed tables, events, presence, and broadcast channels", "Measure connection and fan-out needs", "Load-test the replacement and monitor replication lag"],
        ),
        "edge.functions" => (
            Category::EdgeFunction, Portability::Adapt, Effort::Medium,
            "TypeScript/JavaScript logic is portable, but Deno runtime APIs, injected secrets, auth context, and regional execution need adaptation.",
            "Managed deployment, geographic placement, logs, scaling, and secret injection do not move with source files.",
            vec!["Deno Deploy or self-hosted Deno", "Hono on Node.js", "Containerized functions"],
            vec!["Document secrets without copying their values", "Replace platform URLs and auth context", "Add runtime, timeout, retry, and observability tests"],
        ),
        "database.extensions" => (
            Category::Database, Portability::Verify, Effort::Medium,
            "Extension declarations are standard SQL, but each target host controls which binaries and versions are installed.",
            "Supabase extension availability, upgrades, and maintenance windows are hosted-service guarantees.",
            vec!["Target Postgres packaged extensions", "Application-layer equivalent", "Self-hosted extension image"],
            vec!["Compare extension names and versions on the target", "Check dump/restore ordering", "Test upgrade and backup compatibility"],
        ),
        "database.rls" => (
            Category::Database, Portability::Adapt, Effort::Medium,
            "Row-level security is native Postgres and policies migrate as SQL.",
            "JWT claim shape and database roles supplied by Supabase Auth are not created by plain Postgres.",
            vec!["Native Postgres RLS with application-set claims", "Application authorization with restricted database roles"],
            vec!["Map anon/authenticated/service roles", "Replace auth.uid() and JWT claim helpers", "Run allow-and-deny policy regression tests"],
        ),
        "database.routines" => (
            Category::Database, Portability::Portable, Effort::Low,
            "Postgres functions and triggers are dumpable SQL when their languages and dependencies exist on the target.",
            "Execution monitoring, statement timeouts, and operational tuning must be recreated.",
            vec!["Native Postgres functions and triggers"],
            vec!["Order routines before dependent triggers", "Audit SECURITY DEFINER search paths", "Run behavior tests under target roles"],
        ),
        "platform.cron" => (
            Category::Platform, Portability::Adapt, Effort::Medium,
            "pg_cron is open source but not universally available on managed Postgres.",
            "Scheduler monitoring, retries, and incident response are operational responsibilities.",
            vec!["pg_cron on the target", "systemd timers", "Application job runner"],
            vec!["List schedules, time zones, owners, and overlap behavior", "Recreate secrets safely", "Add failure alerts and idempotency tests"],
        ),
        "platform.vault" => (
            Category::Platform, Portability::Replace, Effort::High,
            "Vault schema calls depend on a Supabase-supported extension and should not be treated as a general Postgres secret store.",
            "Encryption keys, rotation, access control, and recovery procedures are service-specific.",
            vec!["SOPS-encrypted configuration", "OpenBao", "Container/runtime secrets"],
            vec!["Inventory secret names only; never print values", "Rotate secrets into the replacement", "Remove database-readable secrets after cutover"],
        ),
        "platform.network" => (
            Category::Platform, Portability::Adapt, Effort::Medium,
            "Outbound database HTTP extensions require target packages and explicit network policy.",
            "Egress availability, DNS, retries, and abuse protection vary by host.",
            vec!["Application worker HTTP calls", "pg_net if supported", "Durable job queue"],
            vec!["Inventory endpoints without logging credentials", "Move retryable side effects to a queue", "Set egress policy, timeouts, and alerts"],
        ),
        _ => (
            Category::Database, Portability::Verify, Effort::Low,
            "This project-local signal is likely portable but should be verified against the target environment.",
            "Runtime availability and operations are not implied by SQL compatibility.",
            vec!["Plain Postgres or an application-layer equivalent"],
            vec!["Verify behavior on a restored staging copy"],
        ),
    };
    Finding {
        id: id.into(),
        name: name.into(),
        category,
        portability,
        effort,
        evidence: Vec::new(),
        compatibility: compatibility.into(),
        hosted_guarantee: guarantee.into(),
        replacements: replacements.into_iter().map(str::to_string).collect(),
        next_steps: steps.into_iter().map(str::to_string).collect(),
    }
}
