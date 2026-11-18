import { test, expect } from "@playwright/test";
import path from "node:path";
import fs from "node:fs";

test("analytics prototype offers design themes and snapshots deal-feed", async ({
  page,
}) => {
  await page.goto("/prototype/analytics");
  await expect(
    page.getByRole("heading", { name: "Choose a design direction" }),
  ).toBeVisible();

  await page.getByTestId("theme-option-deal-feed").click();
  await expect(page.getByTestId("analytics-prototype-deal-feed")).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Analytics dashboard" }),
  ).toBeVisible();

  const outDir = path.resolve("..", "artifacts", "playwright");
  fs.mkdirSync(outDir, { recursive: true });
  const shotPath = path.join(outDir, "analytics-prototype-deal-feed.png");
  await page.screenshot({ path: shotPath, fullPage: true });
  expect(fs.existsSync(shotPath)).toBeTruthy();
});

test("pulseboard theme remains available", async ({ page }) => {
  await page.goto("/prototype/analytics?theme=pulseboard");
  await expect(page.getByTestId("analytics-prototype-pulseboard")).toBeVisible();
});
