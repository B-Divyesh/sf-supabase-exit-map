import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

async function expectNoSeriousAxeViolations(page) {
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(results.violations.filter((violation) => ["serious", "critical"].includes(violation.impact))).toEqual([]);
}

test("@claim:offline-reload the guide reloads offline after its first visit", async ({ browser }) => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const errors = [];
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });

  await page.goto("/", { waitUntil: "networkidle" });
  await page.waitForFunction(() => navigator.serviceWorker?.controller !== null);
  await context.setOffline(true);
  await page.reload({ waitUntil: "domcontentloaded" });
  await expect(page.locator("h1")).toHaveText("Map Supabase dependencies before you migrate");
  expect(errors.filter((error) => /module script|MIME type/i.test(error))).toEqual([]);
  await context.close();
});

test("@claim:demo-sandbox sample mode shows populated output and keeps separate storage", async ({ page }) => {
  await page.goto("/demo/", { waitUntil: "networkidle" });
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
  await expect(page.locator(".demo-finding")).toHaveCount(11);
  await expect(page.locator("#demo-summary")).toContainText("11 findings loaded");
  expect(await page.evaluate(() => ({
    demo: localStorage.getItem("demo:supabase-exit-map:sample-v1"),
    real: localStorage.getItem("sb_plan:supabase-exit-map")
  }))).toEqual({ demo: expect.any(String), real: null });
  await page.getByRole("button", { name: "Reset demo" }).click();
  await expect(page.locator("#demo-summary")).toContainText("loaded again");
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
});

test("@claim:site-privacy demo load makes no third-party requests", async ({ browser }) => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const requests = [];
  page.on("request", (request) => requests.push(request.url()));
  await page.goto("/demo/", { waitUntil: "networkidle" });
  expect(requests.length).toBeGreaterThan(0);
  expect(requests.every((url) => new URL(url).origin === "http://127.0.0.1:4173")).toBe(true);
  await context.close();
});

test("@claim:license-restore valid license returns unlock the local Planning Room", async ({ page }) => {
  await page.route("https://api.sociobot.in/api/v1/products/supabase-exit-map/verify?license=demo-license-123", async (route) => {
    await route.fulfill({ contentType: "application/json", body: JSON.stringify({ valid: true, reason: "ok" }) });
  });
  await page.goto("/?license=demo-license-123", { waitUntil: "networkidle" });
  await expect(page.locator("#planner")).toBeVisible();
  await expect(page.locator("#license-status")).toContainText("License verified");
  expect(await page.evaluate(() => ({
    query: location.search,
    token: localStorage.getItem("sb_license:supabase-exit-map")
  }))).toEqual({ query: "", token: "demo-license-123" });
});

test("@claim:local-planning imported reports stay in browser storage", async ({ page }) => {
  await page.route("https://api.sociobot.in/api/v1/products/supabase-exit-map/verify?license=demo-license-456", async (route) => {
    await route.fulfill({ contentType: "application/json", body: JSON.stringify({ valid: true, reason: "ok" }) });
  });
  const outgoing = [];
  page.on("request", (request) => {
    if (request.url().startsWith("https://")) outgoing.push({ url: request.url(), body: request.postData() });
  });
  await page.goto("/?license=demo-license-456", { waitUntil: "networkidle" });
  await page.getByRole("button", { name: "Load example" }).click();
  await expect(page.locator("#plan-body tr")).toHaveCount(3);
  await page.getByRole("button", { name: "Save on this device" }).click();
  const plan = await page.evaluate(() => localStorage.getItem("sb_plan:supabase-exit-map"));
  expect(plan).toContain("Authentication configuration");
  expect(outgoing).toEqual([{ url: "https://api.sociobot.in/api/v1/products/supabase-exit-map/verify?license=demo-license-456", body: null }]);
});

test("@claim:planning-price shows the registered price without linking to unavailable checkout", async ({ page }) => {
  await page.goto("/#planning-room", { waitUntil: "networkidle" });
  await expect(page.locator(".price")).toContainText("$19 one time");
  await expect(page.locator("#checkout-status")).toContainText("Checkout registration is pending");
  await expect(page.locator('a[href*="/checkout"]')).toHaveCount(0);
});

test("first screen, keyboard controls, mobile layout, and accessibility work", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/", { waitUntil: "networkidle" });
  await expect(page.locator("h1")).toHaveText("Map Supabase dependencies before you migrate");
  await expect(page.getByRole("link", { name: /Try it with sample data/ })).toBeVisible();
  await expect(page.getByText("For indie builders facing managed-backend costs who need a safe exit plan.")).toBeVisible();
  await expect.poll(() => page.evaluate(() => document.documentElement.scrollWidth)).toBe(390);
  const commandBlocks = await page.locator(".code-block").evaluateAll((blocks) => blocks.map((block) => ({
    right: block.getBoundingClientRect().right,
    overflowX: getComputedStyle(block).overflowX
  })));
  expect(commandBlocks.every((block) => block.right <= 390 && block.overflowX === "auto")).toBe(true);
  await page.locator("#tab-all").focus();
  await page.keyboard.press("ArrowRight");
  await expect(page.locator("#tab-database")).toHaveAttribute("aria-selected", "true");
  await expectNoSeriousAxeViolations(page);
});

test("legal and demo pages have no serious or critical accessibility violations", async ({ page }) => {
  for (const path of ["/demo/", "/privacy/", "/terms/", "/404.html"]) {
    await page.goto(path, { waitUntil: "networkidle" });
    await expectNoSeriousAxeViolations(page);
  }
});
