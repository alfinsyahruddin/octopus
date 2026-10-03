import type { Page, Route } from "@playwright/test";

export type ApiMock = (route: Route) => Promise<void>;

/** Intercept known API calls and record every unexpected one, including background requests. */
export async function installApiMocks(
  page: Page,
  mocks: Record<string, ApiMock>,
) {
  const unexpectedRequests: string[] = [];

  await page.route(/(?:\/canvas\/|\/api\/)/, async (route) => {
    const request = route.request();
    const url = new URL(request.url());
    const key = `${request.method()} ${url.pathname}`;
    const mock = mocks[key];

    if (mock) {
      await mock(route);
      return;
    }

    unexpectedRequests.push(key);
    await route.abort("failed");
  });

  return {
    assertNoUnexpectedRequests() {
      if (unexpectedRequests.length > 0) {
        throw new Error(
          `Unexpected API requests: ${unexpectedRequests.join(", ")}`,
        );
      }
    },
  };
}
