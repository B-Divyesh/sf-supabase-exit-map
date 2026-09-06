import assert from "node:assert/strict";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { createServer } from "node:http";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawn, spawnSync } from "node:child_process";
import { test } from "node:test";

const cwd = new URL("../..", import.meta.url).pathname;

function runCli(args) {
  return spawnSync("cargo", ["run", "--quiet", "--bin", "supabase-exit-map", "--", ...args], {
    cwd,
    encoding: "utf8"
  });
}

function runCliAsync(args, env) {
  return new Promise((resolve, reject) => {
    const child = spawn("cargo", ["run", "--quiet", "--bin", "supabase-exit-map", "--", ...args], { cwd, env });
    let stdout = "";
    let stderr = "";
    child.stdout.on("data", (chunk) => { stdout += chunk; });
    child.stderr.on("data", (chunk) => { stderr += chunk; });
    child.on("error", reject);
    child.on("close", (status) => resolve({ status, stdout, stderr }));
  });
}

test("@claim:cli-demo bundled demo creates an isolated report from sample files", async () => {
  const result = runCli(["--demo", "--json"]);
  assert.equal(result.status, 0, result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.scope.live_project_queried, false);
  assert.ok(report.findings.length >= 10);
  const outputLine = result.stderr.split("\n").find((line) => line.startsWith("Demo report written to "));
  assert.ok(outputLine, result.stderr);
  const reportPath = outputLine.slice("Demo report written to ".length);
  assert.match(reportPath, /supabase-exit-map-demo-/);
  const saved = JSON.parse(await readFile(reportPath, "utf8"));
  assert.equal(saved.findings.length, report.findings.length);
});

test("@claim:dependency-inventory reports evidence, effort, replacements, and checklist", () => {
  const result = runCli(["--demo", "--json"]);
  assert.equal(result.status, 0, result.stderr);
  const report = JSON.parse(result.stdout);
  const ids = new Set(report.findings.map((finding) => finding.id));
  for (const expected of ["auth.config", "storage", "realtime", "database.rls", "edge.functions", "platform.vault"]) {
    assert.ok(ids.has(expected), "missing " + expected);
  }
  assert.ok(report.findings.every((finding) => finding.evidence.length > 0 && finding.effort && finding.replacements.length > 0));
  assert.ok(report.checklist.length >= 3);
});

test("@claim:cli-paths scans both a project root and its Supabase directory", () => {
  for (const path of ["examples/demo-project", "examples/demo-project/supabase"]) {
    const result = runCli([path, "--json"]);
    assert.equal(result.status, 0, result.stderr);
    assert.ok(JSON.parse(result.stdout).findings.length >= 10);
  }
});

test("@claim:cli-output writes JSON and Markdown reports for planning tools", async () => {
  const folder = await mkdtemp(join(tmpdir(), "exit-map-claim-"));
  try {
    const jsonPath = join(folder, "report.json");
    const markdownPath = join(folder, "report.md");
    const json = runCli(["examples/demo-project", "--json", "--output", jsonPath]);
    const markdown = runCli(["examples/demo-project", "--format", "markdown", "--output", markdownPath]);
    assert.equal(json.status, 0, json.stderr);
    assert.equal(markdown.status, 0, markdown.stderr);
    assert.ok(JSON.parse(await readFile(jsonPath, "utf8")).findings.length >= 10);
    assert.match(await readFile(markdownPath, "utf8"), /## Portable SQL and cutover checklist/);
  } finally {
    await rm(folder, { recursive: true, force: true });
  }
});

test("@claim:cli-errors distinguishes invalid input from strict scan warnings", async () => {
  const missing = runCli(["this-path-is-not-present", "--json"]);
  assert.equal(missing.status, 2);
  const folder = await mkdtemp(join(tmpdir(), "exit-map-empty-"));
  try {
    const strict = runCli([folder, "--strict", "--json"]);
    assert.equal(strict.status, 3, strict.stderr);
    const warningReport = JSON.parse(strict.stdout);
    assert.equal(warningReport.findings.length, 0);
    assert.ok(warningReport.warnings.length >= 4);
  } finally {
    await rm(folder, { recursive: true, force: true });
  }
});

test("@claim:local-scan does not need a Supabase credential to inspect the bundled files", () => {
  const result = spawnSync("cargo", ["run", "--quiet", "--bin", "supabase-exit-map", "--", "--demo", "--json"], {
    cwd,
    encoding: "utf8",
    env: { ...process.env, SUPABASE_ACCESS_TOKEN: "" }
  });
  assert.equal(result.status, 0, result.stderr);
  assert.equal(JSON.parse(result.stdout).scope.live_project_queried, false);
});

test("@claim:no-cli-network bundled scans make no proxied network request", async () => {
  let requests = 0;
  const proxy = createServer((_, response) => {
    requests += 1;
    response.writeHead(502).end();
  });
  proxy.on("connect", (request, socket) => {
    requests += 1;
    socket.end("HTTP/1.1 502 Bad Gateway\r\n\r\n");
  });
  await new Promise((resolve) => proxy.listen(0, "127.0.0.1", resolve));
  try {
    const address = proxy.address();
    assert.ok(address && typeof address !== "string");
    const proxyUrl = "http://127.0.0.1:" + address.port;
    const result = await runCliAsync(["--demo", "--json"], {
      ...process.env,
      ALL_PROXY: proxyUrl,
      HTTP_PROXY: proxyUrl,
      HTTPS_PROXY: proxyUrl,
      SUPABASE_ACCESS_TOKEN: "ignored-test-value"
    });
    assert.equal(result.status, 0, result.stderr);
    assert.equal(JSON.parse(result.stdout).scope.live_project_queried, false);
    assert.equal(requests, 0);
  } finally {
    await new Promise((resolve, reject) => proxy.close((error) => error ? reject(error) : resolve()));
  }
  const tree = spawnSync("cargo", ["tree", "-p", "supabase-exit-map"], { cwd, encoding: "utf8" });
  assert.equal(tree.status, 0, tree.stderr);
  assert.doesNotMatch(tree.stdout, /reqwest|ureq|hyper|tokio|surf/i);
});

test("@claim:read-only-scan leaves input files unchanged and omits auth-data files", async () => {
  const folder = await mkdtemp(join(tmpdir(), "exit-map-read-only-"));
  const migration = join(folder, "supabase", "migrations", "001.sql");
  const authData = join(folder, "supabase", "auth", "users.csv");
  await mkdir(join(folder, "supabase", "migrations"), { recursive: true });
  await mkdir(join(folder, "supabase", "auth"), { recursive: true });
  await writeFile(migration, "create table public.notes (id uuid);\n");
  await writeFile(authData, "private-user-data-sentinel\n");
  try {
    const beforeMigration = await readFile(migration, "utf8");
    const beforeAuthData = await readFile(authData, "utf8");
    const result = runCli([folder, "--json"]);
    assert.equal(result.status, 0, result.stderr);
    assert.equal(await readFile(migration, "utf8"), beforeMigration);
    assert.equal(await readFile(authData, "utf8"), beforeAuthData);
    assert.doesNotMatch(result.stdout, /private-user-data-sentinel/);
  } finally {
    await rm(folder, { recursive: true, force: true });
  }
});
