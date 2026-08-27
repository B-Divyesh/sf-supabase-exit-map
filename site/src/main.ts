import "./styles.css";

const SLUG = "supabase-exit-map";
const API = `https://api.sociobot.in/api/v1/products/${SLUG}`;
const LICENSE_KEY = `sb_license:${SLUG}`;
const VERDICT_KEY = `sb_license_verdict:${SLUG}`;
const PLAN_KEY = `sb_plan:${SLUG}`;
const DAY = 86_400_000;

type Verdict = { token: string; valid: boolean; checkedAt: number; reason?: string };
type Finding = { id?: string; name: string; replacements?: string[]; effort?: string };
type ExitReport = { schema_version: string; findings: Finding[] };
type Plan = { report: ExitReport; choices: Array<{ replacement: string; owner: string; status: string }> };

const byId = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const safely = {
  get(key: string) { try { return localStorage.getItem(key); } catch { return null; } },
  set(key: string, value: string) { try { localStorage.setItem(key, value); return true; } catch { return false; } }
};

function setTheme(theme: "light" | "dark") {
  document.documentElement.dataset.theme = theme;
  const button = byId<HTMLButtonElement>("theme-toggle");
  button.setAttribute("aria-label", `Switch to ${theme === "dark" ? "light" : "dark"} theme`);
}

const savedTheme = safely.get("sem_theme");
setTheme(savedTheme === "dark" || savedTheme === "light" ? savedTheme : matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light");
byId<HTMLButtonElement>("theme-toggle").addEventListener("click", () => {
  const next = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
  setTheme(next);
  safely.set("sem_theme", next);
});

for (const button of document.querySelectorAll<HTMLButtonElement>(".copy")) {
  button.addEventListener("click", async () => {
    const previous = button.textContent;
    try {
      await navigator.clipboard.writeText(button.dataset.copy || "");
      button.textContent = "Copied";
    } catch {
      button.textContent = "Select text";
      button.closest(".code-block")?.querySelector("code")?.parentElement?.focus();
    }
    window.setTimeout(() => { button.textContent = previous; }, 1800);
  });
}

const tabs = Array.from(document.querySelectorAll<HTMLButtonElement>("[role=tab]"));
function selectTab(tab: HTMLButtonElement) {
  tabs.forEach((item) => { item.setAttribute("aria-selected", String(item === tab)); item.tabIndex = item === tab ? 0 : -1; });
  const filter = tab.dataset.filter;
  document.querySelectorAll<HTMLElement>(".finding").forEach((finding) => { finding.hidden = filter !== "all" && finding.dataset.group !== filter; });
  byId("findings").setAttribute("aria-labelledby", tab.id);
}
tabs.forEach((tab, index) => {
  tab.addEventListener("click", () => selectTab(tab));
  tab.addEventListener("keydown", (event) => {
    if (event.key !== "ArrowLeft" && event.key !== "ArrowRight" && event.key !== "Home" && event.key !== "End") return;
    event.preventDefault();
    const next = event.key === "Home" ? 0 : event.key === "End" ? tabs.length - 1 : (index + (event.key === "ArrowRight" ? 1 : -1) + tabs.length) % tabs.length;
    tabs[next].focus(); selectTab(tabs[next]);
  });
});

const offline = byId("offline-notice");
function updateNetwork() { offline.hidden = navigator.onLine; }
addEventListener("online", updateNetwork); addEventListener("offline", updateNetwork); updateNetwork();

const licenseStatus = byId("license-status");
const planner = byId("planner");
function status(message: string, kind = "") { licenseStatus.textContent = message; licenseStatus.className = `status ${kind}`.trim(); }
function unlock(message: string) { planner.hidden = false; status(message, "success"); }
function lock(message: string, kind = "") { planner.hidden = true; status(message, kind); }

async function verify(token: string, force = false) {
  const cached = parseJson<Verdict>(safely.get(VERDICT_KEY));
  if (!force && cached?.token === token && Date.now() - cached.checkedAt < DAY) {
    cached.valid ? unlock("Planning Room is unlocked on this device.") : lock("License no longer active. You can restore another token or buy a license.", "error");
    return;
  }
  if (cached?.token === token && cached.valid) unlock("Planning Room is unlocked. Refreshing the daily license check…");
  else status("Checking license…");
  try {
    const response = await fetch(`${API}/verify?license=${encodeURIComponent(token)}`, { headers: { Accept: "application/json" } });
    if (!response.ok) throw new Error(`verification returned ${response.status}`);
    const data = await response.json() as { valid: boolean; reason?: string };
    const verdict: Verdict = { token, valid: data.valid, reason: data.reason, checkedAt: Date.now() };
    safely.set(VERDICT_KEY, JSON.stringify(verdict));
    if (data.valid) unlock("License verified. Planning Room is unlocked.");
    else lock("License no longer active. You can restore another token or buy a license.", "error");
  } catch {
    if (cached?.token === token && cached.valid) unlock("Planning Room is available from the last check. We’ll verify again when you’re online.");
    else lock("Couldn’t verify the license. Check your connection and try again; the free CLI is unaffected.", "error");
  }
}

function acceptToken(token: string) {
  const clean = token.trim();
  if (clean.length < 8 || clean.length > 4096) { lock("That token does not look complete. Paste the token from your receipt.", "error"); return; }
  safely.set(LICENSE_KEY, clean);
  void verify(clean, true);
}

const params = new URLSearchParams(location.search);
const returned = params.get("license");
if (returned) {
  safely.set(LICENSE_KEY, returned);
  params.delete("license");
  const query = params.toString();
  history.replaceState({}, "", `${location.pathname}${query ? `?${query}` : ""}${location.hash}`);
  void verify(returned, true);
} else {
  const token = safely.get(LICENSE_KEY);
  if (token) void verify(token); else lock("No license saved.");
}

byId<HTMLFormElement>("license-form").addEventListener("submit", (event) => {
  event.preventDefault();
  const input = byId<HTMLInputElement>("license-token");
  if (!input.reportValidity()) return;
  acceptToken(input.value);
});

function parseJson<T>(raw: string | null): T | null { if (!raw) return null; try { return JSON.parse(raw) as T; } catch { return null; } }
function validateReport(value: unknown): ExitReport {
  if (!value || typeof value !== "object") throw new Error("The report must be a JSON object.");
  const candidate = value as Partial<ExitReport>;
  if (typeof candidate.schema_version !== "string" || !Array.isArray(candidate.findings)) throw new Error("This is not an Exit Map JSON report.");
  if (!candidate.findings.every((item) => item && typeof item.name === "string" && (!item.replacements || Array.isArray(item.replacements)))) throw new Error("One or more findings have an unsupported shape.");
  return candidate as ExitReport;
}

const example: ExitReport = { schema_version: "1.0", findings: [
  { id: "auth.config", name: "Authentication configuration", effort: "high", replacements: ["Self-hosted Supabase Auth (GoTrue)", "Keycloak", "Application auth"] },
  { id: "storage", name: "Object storage", effort: "high", replacements: ["S3-compatible storage (MinIO)", "Cloudflare R2", "AWS S3"] },
  { id: "realtime", name: "Realtime channels and replication", effort: "medium", replacements: ["Self-hosted Supabase Realtime", "Application websocket", "ElectricSQL"] }
] };

let currentReport: ExitReport | null = null;
const plannerStatus = byId("planner-status");
function plannerMessage(message: string, kind = "") { plannerStatus.textContent = message; plannerStatus.className = `status ${kind}`.trim(); }
function option(value: string, selected = false) { const node = document.createElement("option"); node.value = value; node.textContent = value; node.selected = selected; return node; }
function control<K extends "input" | "select">(kind: K, label: string) {
  const node = document.createElement(kind); node.setAttribute("aria-label", label); return node;
}

function buildPlan(report: ExitReport, choices: Plan["choices"] = []) {
  currentReport = report;
  const body = byId<HTMLTableSectionElement>("plan-body"); body.replaceChildren();
  report.findings.forEach((finding, index) => {
    const row = document.createElement("tr");
    const name = document.createElement("td"); name.dataset.label = "Dependency"; name.textContent = `${finding.name}${finding.effort ? ` · ${finding.effort}` : ""}`;
    const replacementCell = document.createElement("td"); replacementCell.dataset.label = "Replacement";
    const replacement = control("select", `Replacement for ${finding.name}`);
    const routes = finding.replacements?.length ? finding.replacements : ["Decide after target test"];
    routes.forEach((route) => replacement.append(option(route, route === choices[index]?.replacement))); replacementCell.append(replacement);
    const ownerCell = document.createElement("td"); ownerCell.dataset.label = "Owner";
    const owner = control("input", `Owner for ${finding.name}`); owner.placeholder = "Unassigned"; owner.value = choices[index]?.owner || ""; ownerCell.append(owner);
    const statusCell = document.createElement("td"); statusCell.dataset.label = "Status";
    const progress = control("select", `Status for ${finding.name}`);
    ["Not started", "Investigating", "Ready to rehearse", "Verified"].forEach((value) => progress.append(option(value, value === choices[index]?.status))); statusCell.append(progress);
    row.append(name, replacementCell, ownerCell, statusCell); body.append(row);
  });
  byId("plan-output").hidden = false;
  plannerMessage(`${report.findings.length} dependencies are ready to assign. Changes stay in this browser.`, "success");
}

function readChoices(): Plan["choices"] {
  return Array.from(byId("plan-body").rows).map((row) => ({
    replacement: (row.cells[1].querySelector("select") as HTMLSelectElement).value,
    owner: (row.cells[2].querySelector("input") as HTMLInputElement).value,
    status: (row.cells[3].querySelector("select") as HTMLSelectElement).value
  }));
}

byId<HTMLFormElement>("planner-import").addEventListener("submit", (event) => {
  event.preventDefault();
  try { buildPlan(validateReport(JSON.parse(byId<HTMLTextAreaElement>("report-json").value))); }
  catch (error) { plannerMessage(error instanceof Error ? error.message : "Couldn’t read that report.", "error"); }
});
byId("load-example").addEventListener("click", () => { byId<HTMLTextAreaElement>("report-json").value = JSON.stringify(example, null, 2); buildPlan(example); });
byId("save-plan").addEventListener("click", () => {
  if (!currentReport) return;
  const saved = safely.set(PLAN_KEY, JSON.stringify({ report: currentReport, choices: readChoices() } satisfies Plan));
  plannerMessage(saved ? "Plan saved on this device." : "This browser blocked local storage. Print or copy your decisions instead.", saved ? "success" : "error");
});
byId("print-plan").addEventListener("click", () => window.print());

const savedPlan = parseJson<Plan>(safely.get(PLAN_KEY));
if (savedPlan) { try { buildPlan(validateReport(savedPlan.report), savedPlan.choices); } catch { /* keep the import state */ } }

if ("serviceWorker" in navigator) addEventListener("load", () => { navigator.serviceWorker.register("/sw.js").catch(() => undefined); });
