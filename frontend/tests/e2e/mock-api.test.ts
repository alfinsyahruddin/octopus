import { expect, test } from "@playwright/test";
import { installApiMocks } from "./mock-api.js";

test("reports an unexpected background API request", async ({ page }) => {
  const api = await installApiMocks(page, {
    "GET /canvas/models": async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({
          data: [],
          status: 200,
          message: null,
          timestamp: new Date().toISOString(),
        }),
      });
    },
  });

  await page.goto("/");
  await page.evaluate(async () => {
    try {
      await fetch("/api/background-poll");
    } catch {
      // The mock guard aborts unknown API requests by design.
    }
  });

  expect(() => api.assertNoUnexpectedRequests()).toThrow(
    "Unexpected API requests: GET /api/background-poll",
  );
});
