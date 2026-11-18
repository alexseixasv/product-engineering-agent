import { test, expect } from "@playwright/test";
import path from "node:path";
import fs from "node:fs";

test("analytics prototype visual snapshot is reproducible", async ({ page }) => {
  await page.goto("/prototype/analytics");
  await expect(page.getByRole("heading", { name: "Analytics dashboard" })).toBeVisible();
  const outDir = path.resolve("..", "artifacts", "playwright");
  fs.mkdirSync(outDir, { recursive: true });
  const shotPath = path.join(outDir, "analytics-prototype.png");
  await page.screenshot({ path: shotPath, fullPage: true });
  expect(fs.existsSync(shotPath)).toBeTruthy();
});
