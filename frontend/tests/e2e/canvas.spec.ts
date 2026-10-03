import { expect, test } from "@playwright/test";
import { installApiMocks } from "./mock-api.js";

test.describe("Octopus System One Decision Playground", () => {
  test("Choice decision flow, state persistence, and reset", async ({
    page,
  }) => {
    const api = await installApiMocks(page, {
      "GET /canvas/models": async (route) => {
        await route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify({
            data: [
              {
                id: "clef-flash",
                name: "Clef Flash 9B",
                description:
                  "Fast 9B multimodal System One decision model by Cloudflare",
                supported_types: ["choice", "score", "noul"],
                supports_vision: true,
              },
            ],
            status: 200,
            message: null,
            timestamp: new Date().toISOString(),
          }),
        });
      },
      "POST /canvas/evaluate": async (route) => {
        const req = route.request().postDataJSON();

        if (req.model_type === "choice") {
          await route.fulfill({
            status: 200,
            contentType: "application/json",
            body: JSON.stringify({
              data: {
                model: "clef-flash",
                model_type: "choice",
                result: {
                  type: "choice",
                  choice: "billing",
                  probabilities: {
                    billing: 0.98,
                    support: 0.02,
                  },
                  confidence: 0.92,
                },
                ai_duration_ms: 38.5,
              },
              status: 200,
              message: null,
              timestamp: new Date().toISOString(),
            }),
          });
        } else if (req.model_type === "score") {
          await route.fulfill({
            status: 200,
            contentType: "application/json",
            body: JSON.stringify({
              data: {
                model: "clef-flash",
                model_type: "score",
                result: {
                  type: "score",
                  score: 1.85,
                  legend: {
                    "0": "Low",
                    "1": "Medium",
                    "2": "Critical",
                  },
                  probabilities: {
                    "0": 0.05,
                    "1": 0.1,
                    "2": 0.85,
                  },
                  confidence: 0.81,
                },
                ai_duration_ms: 42.1,
              },
              status: 200,
              message: null,
              timestamp: new Date().toISOString(),
            }),
          });
        } else if (req.model_type === "noul") {
          await route.fulfill({
            status: 200,
            contentType: "application/json",
            body: JSON.stringify({
              data: {
                model: "clef-flash",
                model_type: "noul",
                result: {
                  type: "noul",
                  noul: 0.965,
                },
                ai_duration_ms: 29.4,
              },
              status: 200,
              message: null,
              timestamp: new Date().toISOString(),
            }),
          });
        }
      },
    });

    await page.goto("/");

    // Verify header and title
    await expect(page.locator("h1")).toContainText(
      "System One Decision Playground",
    );
    await expect(page.locator("select")).toHaveValue("clef-flash");
    await expect(page.locator("text=IDLE")).toBeVisible();

    // 1. Test Choice Flow
    // Fill in instruction
    const instructionTextarea = page.locator("textarea").first();
    await instructionTextarea.fill("Which team should handle this?");

    // Fill in choice 1 and 2
    const choiceInputs = page.locator('input[placeholder*="Option name"]');
    await choiceInputs.nth(0).fill("billing");
    await choiceInputs.nth(1).fill("support");

    // Fill in prompt
    const promptTextarea = page.locator("textarea").nth(1);
    await promptTextarea.fill(
      "I was charged twice for my subscription this month",
    );

    // Await real-time evaluated result
    await expect(page.locator("text=Done")).toBeVisible();
    await expect(page.locator("text=billing").first()).toBeVisible();
    await expect(page.locator("text=Selected Choice Winner")).toBeVisible();
    await expect(page.locator("text=AI: 38.5 ms")).toBeVisible();

    // Verify page reload preserves result in frontend
    await page.reload();
    await expect(page.locator("text=Done")).toBeVisible();
    await expect(page.locator("text=billing").first()).toBeVisible();
    await expect(page.locator("text=Selected Choice Winner")).toBeVisible();
    await expect(page.locator("text=AI: 38.5 ms")).toBeVisible();

    // 2. Test Tab Switching to Score
    await page.locator('button:has-text("Score")').click();
    await expect(page.locator("text=Ordered Scale Levels")).toBeVisible();

    // Fill in Score inputs
    await page
      .locator("textarea")
      .first()
      .fill("How critical is this incident?");
    const levelInputs = page.locator('input[placeholder*="Level"]');
    await levelInputs.nth(0).fill("Low");
    await levelInputs.nth(1).fill("Medium");
    // Add third level
    await page.locator('button:has-text("Add Level")').click();
    await page.locator('input[placeholder*="Level"]').nth(2).fill("Critical");

    await page
      .locator("textarea")
      .nth(1)
      .fill("Production database cluster is completely unreachable");

    // Verify Score result
    await expect(
      page.locator("text=Probability-Weighted Mean Score"),
    ).toBeVisible();
    await expect(page.locator("text=1.85")).toBeVisible();
    await expect(page.locator("text=AI: 42.1 ms")).toBeVisible();

    // 3. Switch back to Choice tab -> verify state persistence!
    await page.locator('button:has-text("Choice")').click();
    await expect(page.locator("textarea").first()).toHaveValue(
      "Which team should handle this?",
    );
    await expect(
      page.locator('input[placeholder*="Option name"]').nth(0),
    ).toHaveValue("billing");
    await expect(
      page.locator('input[placeholder*="Option name"]').nth(1),
    ).toHaveValue("support");
    await expect(page.locator("textarea").nth(1)).toHaveValue(
      "I was charged twice for my subscription this month",
    );
    await expect(page.locator("text=Selected Choice Winner")).toBeVisible();

    // 4. Test Noul Tab
    await page.locator('button:has-text("Noul")').click();
    await page
      .locator("textarea")
      .first()
      .fill("Is this request an escalation?");
    await page
      .locator("textarea")
      .nth(1)
      .fill("Immediate executive attention required");

    await expect(
      page.locator("text=Calibrated Decision Outcome"),
    ).toBeVisible();
    await expect(page.locator("text=YES")).toBeVisible();
    await expect(page.locator("text=AI: 29.4 ms")).toBeVisible();

    // 5. Test Reset Button
    await page.locator('button[title*="Reset"]').click();
    await expect(page.locator("textarea").first()).toHaveValue("");
    await expect(page.locator("textarea").nth(1)).toHaveValue("");
    await expect(page.locator("text=Awaiting Decision Schema")).toBeVisible();
    await expect(page.locator("text=IDLE")).toBeVisible();

    // 6. Test Theme Toggle
    const themeToggle = page.locator('button[aria-label="Toggle theme"]');
    await themeToggle.click();
    const hasDark = await page.evaluate(() =>
      document.documentElement.classList.contains("dark"),
    );
    expect(hasDark).toBe(true);

    api.assertNoUnexpectedRequests();
  });
});
