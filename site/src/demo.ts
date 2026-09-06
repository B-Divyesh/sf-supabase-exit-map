import "./styles.css";

type SampleFinding = { name: string; effort: string; evidence: string; next: string };

const DEMO_KEY = "demo:supabase-exit-map:sample-v1";
const sample: SampleFinding[] = [
  { name: "Authentication configuration", effort: "high · replace", evidence: "config.toml enables GitHub sign-in.", next: "List providers, redirects, sessions, and mail." },
  { name: "Auth schema dependencies", effort: "high · replace", evidence: "workspace.sql references auth.users.", next: "Rehearse the supported user transfer." },
  { name: "Object storage", effort: "high · replace", evidence: "workspace.sql creates the attachments bucket.", next: "Count objects and test signed URLs." },
  { name: "Realtime channels", effort: "medium · replace", evidence: "workspace.sql publishes public.notes.", next: "Measure fan-out and replication lag." },
  { name: "Row-level security", effort: "medium · adapt", evidence: "workspace.sql calls auth.uid() in a policy.", next: "Replace claim shapes and test allow and deny cases." },
  { name: "Functions and triggers", effort: "low · portable", evidence: "workspace.sql defines a security definer trigger.", next: "Audit owners and search paths." },
  { name: "Postgres extensions", effort: "medium · verify", evidence: "workspace.sql enables pg_cron.", next: "Confirm target extension support." },
  { name: "Scheduled database jobs", effort: "medium · replace", evidence: "workspace.sql schedules archive-notes.", next: "Choose a scheduler and retry policy." },
  { name: "Database-managed secrets", effort: "high · replace", evidence: "workspace.sql uses vault.create_secret.", next: "Record names and rotate secret values." },
  { name: "Outbound database calls", effort: "medium · replace", evidence: "workspace.sql calls net.http_post.", next: "Move the call behind an application service." },
  { name: "Edge Functions", effort: "medium · adapt", evidence: "send-mail/index.ts uses Deno environment access and fetch.", next: "Choose a runtime and timeout policy." }
];

const read = () => {
  try { return localStorage.getItem(DEMO_KEY); } catch { return null; }
};
const write = () => {
  try { localStorage.setItem(DEMO_KEY, JSON.stringify({ openedAt: Date.now(), findings: sample.length })); } catch { /* Demo still renders when storage is unavailable. */ }
};

function render() {
  write();
  const summary = document.getElementById("demo-summary") as HTMLElement;
  const output = document.getElementById("demo-findings") as HTMLOListElement;
  summary.textContent = String(sample.length) + " findings loaded from the bundled sample.";
  output.replaceChildren(...sample.map((finding, index) => {
    const item = document.createElement("li");
    const tone = finding.effort.startsWith("high") ? "high" : finding.effort.startsWith("medium") ? "medium" : "low";
    item.className = "demo-finding";
    item.innerHTML = '<div class="finding-index">' + String(index + 1).padStart(2, "0") + '</div><div><h3>' + finding.name + '</h3><span class="tag ' + tone + '">' + finding.effort + '</span></div><div><p><strong>Evidence</strong> ' + finding.evidence + '</p><p><strong>Next</strong> ' + finding.next + '</p></div>';
    return item;
  }));
}

document.getElementById("reset-demo")?.addEventListener("click", () => {
  try { localStorage.removeItem(DEMO_KEY); } catch { /* The reset is still an in-memory refresh. */ }
  render();
  (document.getElementById("demo-summary") as HTMLElement).textContent = String(sample.length) + " findings loaded again from the bundled sample.";
});

if (!read()) write();
render();
