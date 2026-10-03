import { expect, test } from '@playwright/test';
import { installApiMocks } from './mock-api.js';

test('reports an unexpected background API request', async ({ page }) => {
	const api = await installApiMocks(page, {});

	await page.goto('/');
	await page.evaluate(async () => {
		try {
			await fetch('/api/background-poll');
		} catch {
			// The mock guard aborts unknown API requests by design.
		}
	});

	expect(() => api.assertNoUnexpectedRequests()).toThrow(
		'Unexpected API requests: GET /api/background-poll'
	);
});
