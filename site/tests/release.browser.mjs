import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

async function expectNoSeriousAxeViolations(page) {
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(results.violations.filter((violation) => ["serious", "critical"].includes(violation.impact))).toEqual([]);
}

test("the installed shell reloads offline without module MIME errors", async ({ browser }) => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const errors = [];
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });

  await page.goto("/", { waitUntil: "networkidle" });
  await expect(page.locator("h1")).toHaveCount(1);
  await page.waitForFunction(() => navigator.serviceWorker?.controller !== null);
  await context.setOffline(true);
  await page.reload({ waitUntil: "domcontentloaded" });
  await expect(page.locator("h1")).toHaveText("Know what has to move before you move it.");
  expect(errors.filter((error) => /module script|MIME type/i.test(error))).toEqual([]);
  await context.close();
});

test("mobile keeps command overflow inside the terminal block", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/", { waitUntil: "networkidle" });
  await expect.poll(() => page.evaluate(() => document.documentElement.scrollWidth)).toBe(390);
  const commandBlocks = await page.locator(".code-block").evaluateAll((blocks) => blocks.map((block) => ({
    right: block.getBoundingClientRect().right,
    overflowX: getComputedStyle(block).overflowX,
    scrolls: block.scrollWidth > block.clientWidth
  })));
  expect(commandBlocks.every((block) => block.right <= 390 && block.overflowX === "auto")).toBe(true);
  expect(commandBlocks.some((block) => block.scrolls)).toBe(true);
});

test("desktop keyboard tabs and accessibility baseline work", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  await page.goto("/", { waitUntil: "networkidle" });
  await page.locator("#tab-all").focus();
  await page.keyboard.press("ArrowRight");
  await expect(page.locator("#tab-database")).toHaveAttribute("aria-selected", "true");
  await expectNoSeriousAxeViolations(page);
});

test("mobile accessibility baseline has no serious or critical violations", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/", { waitUntil: "networkidle" });
  await expectNoSeriousAxeViolations(page);
});
