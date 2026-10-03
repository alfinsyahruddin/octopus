# Frontend Vertical Slice Feature

This reference implementation demonstrates how to build a complete vertical slice through every frontend layer: Domain Types, API Client, Reactive State Module, Svelte 5 UI Component, Page Route, Unit Test, and Mocked E2E Browser Journey.

It matches the backend [`app_settings` vertical slice](../backend/feature.md).

---

## 1. Domain Types (`src/lib/types/settings.ts`)

Define strongly typed frontend interfaces matching the backend API contracts:

```ts
export interface AppSettings {
	maintenance_mode: boolean;
	system_announcement: string | null;
}

export interface UpdateAppSettingsInput {
	maintenance_mode: boolean;
	system_announcement: string | null;
}
```

---

## 2. API Client Methods (`src/lib/api/settings.ts`)

Encapsulate HTTP requests through the centralized transport in [`src/lib/api.ts`](foundations.md#1-centralized-typed-api-transport-srclibapits):

```ts
import { request } from '#lib/api.js';
import type { AppSettings, UpdateAppSettingsInput } from '#lib/types/settings.js';

export async function fetchSettings(): Promise<AppSettings> {
	return request<AppSettings>('/api/settings');
}

export async function updateSettings(input: UpdateAppSettingsInput): Promise<AppSettings> {
	return request<AppSettings>('/api/settings', {
		method: 'PATCH',
		body: JSON.stringify(input)
	});
}
```

---

## 3. Reactive State Module (`src/lib/helpers/settings.svelte.ts`)

Manage domain state, loading indicators, and error tracking using Svelte 5 runes and toast notifications:

```ts
import { fetchSettings, updateSettings } from '#lib/api/settings.js';
import { toast } from '#lib/helpers/toast.svelte.js';
import type { AppSettings, UpdateAppSettingsInput } from '#lib/types/settings.js';

class SettingsState {
	data = $state<AppSettings>({
		maintenance_mode: false,
		system_announcement: null
	});
	isLoading = $state(false);
	isSaving = $state(false);
	error = $state<string | null>(null);

	async load() {
		this.isLoading = true;
		this.error = null;
		try {
			this.data = await fetchSettings();
		} catch (err: unknown) {
			this.error = err instanceof Error ? err.message : 'Failed to load settings';
			toast.error(this.error);
		} finally {
			this.isLoading = false;
		}
	}

	async save(input: UpdateAppSettingsInput) {
		this.isSaving = true;
		this.error = null;
		try {
			this.data = await updateSettings(input);
			toast.success('Settings updated successfully');
			return true;
		} catch (err: unknown) {
			this.error = err instanceof Error ? err.message : 'Failed to update settings';
			toast.error(this.error);
			return false;
		} finally {
			this.isSaving = false;
		}
	}
}

export const settingsState = new SettingsState();
```

---

## 4. UI Component (`src/lib/components/SettingsCard.svelte`)

Implement the UI with Svelte 5 runes (`$props`, `$state`, `$derived`, `onclick`), scoped Tailwind utility styling, and accessibility attributes:

```svelte
<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { AppSettings, UpdateAppSettingsInput } from '#lib/types/settings.js';

	interface Props {
		settings: AppSettings;
		isSaving?: boolean;
		onSave: (updated: UpdateAppSettingsInput) => void;
	}

	let { settings, isSaving = false, onSave }: Props = $props();

	let maintenanceMode = $state(false);
	let systemAnnouncement = $state('');

	$effect(() => {
		maintenanceMode = settings.maintenance_mode;
		systemAnnouncement = settings.system_announcement ?? '';
	});

	const hasChanges = $derived(
		maintenanceMode !== settings.maintenance_mode ||
		(systemAnnouncement || null) !== settings.system_announcement
	);

	function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		onSave({
			maintenance_mode: maintenanceMode,
			system_announcement: systemAnnouncement.trim() ? systemAnnouncement.trim() : null
		});
	}
</script>

<div class="rounded-xl border border-(--border) bg-(--bg-card) p-6 shadow-xs">
	<div class="flex items-center gap-3 mb-6">
		<div class="flex size-10 items-center justify-center rounded-lg bg-(--color-accent)/10 text-(--color-accent)">
			<Icon icon="lucide:settings" width="22" height="22" />
		</div>
		<div>
			<h2 class="text-lg font-semibold text-(--fg)">System Settings</h2>
			<p class="text-sm text-(--fg-muted)">Manage platform maintenance mode and broadcast announcements.</p>
		</div>
	</div>

	<form onsubmit={handleSubmit} class="space-y-5">
		<!-- Maintenance Toggle -->
		<div class="flex items-center justify-between rounded-lg border border-(--border) p-4">
			<div>
				<span class="block font-medium text-(--fg)">Maintenance Mode</span>
				<span class="block text-sm text-(--fg-muted)">Block non-admin users from accessing platform APIs</span>
			</div>
			<label class="relative inline-flex cursor-pointer items-center">
				<input
					type="checkbox"
					bind:checked={maintenanceMode}
					class="sr-only peer"
					data-testid="maintenance-toggle"
				/>
				<div class="peer h-6 w-11 rounded-full bg-(--border) transition-colors peer-checked:bg-(--color-accent) peer-focus:ring-2 peer-focus:ring-(--color-accent)/20 after:absolute after:top-0.5 after:left-[2px] after:size-5 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:after:translate-x-full"></div>
			</label>
		</div>

		<!-- Announcement Textarea -->
		<div>
			<label for="announcement" class="block text-sm font-medium text-(--fg) mb-1.5">
				System Announcement
			</label>
			<textarea
				id="announcement"
				rows="3"
				bind:value={systemAnnouncement}
				placeholder="Enter public broadcast message..."
				data-testid="announcement-input"
				class="w-full rounded-lg border border-(--border) bg-(--bg) p-3 text-sm text-(--fg) placeholder:text-(--fg-muted) focus:border-(--color-accent) focus:outline-hidden"
			></textarea>
		</div>

		<!-- Action Footer -->
		<div class="flex justify-end pt-2">
			<button
				type="submit"
				disabled={!hasChanges || isSaving}
				data-testid="save-settings-btn"
				class="inline-flex items-center gap-2 rounded-lg bg-(--color-accent) px-5 py-2.5 text-sm font-medium text-white transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50"
			>
				{#if isSaving}
					<Icon icon="lucide:loader-2" class="animate-spin" width="16" height="16" />
					Saving...
				{:else}
					<Icon icon="lucide:check" width="16" height="16" />
					Save Settings
				{/if}
			</button>
		</div>
	</form>
</div>
```

---

## 5. Client Route Page (`src/routes/settings/+page.svelte`)

Mount the feature in a client route page and trigger the initial load:

```svelte
<script lang="ts">
	import { onMount } from 'svelte';
	import SettingsCard from '#lib/components/SettingsCard.svelte';
	import { settingsState } from '#lib/helpers/settings.svelte.js';

	onMount(() => {
		settingsState.load();
	});
</script>

<svelte:head>
	<title>System Settings | App</title>
</svelte:head>

<main class="mx-auto max-w-3xl px-4 py-8">
	{#if settingsState.isLoading}
		<div class="flex justify-center p-12 text-(--fg-muted)" data-testid="settings-loading">
			Loading system settings...
		</div>
	{:else if settingsState.error}
		<div class="rounded-lg bg-red-500/10 border border-red-500/20 p-4 text-sm text-red-500">
			{settingsState.error}
		</div>
	{:else}
		<SettingsCard
			settings={settingsState.data}
			isSaving={settingsState.isSaving}
			onSave={(updated) => settingsState.save(updated)}
		/>
	{/if}
</main>
```

---

## 6. Vitest Component Unit Test (`tests/unit/SettingsCard.test.ts`)

Test component rendering, reactive property binding, and save callbacks:

```ts
import { render, screen, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import SettingsCard from '#lib/components/SettingsCard.svelte';

describe('SettingsCard Component', () => {
	it('renders with initial values and disables save button when unchanged', () => {
		render(SettingsCard, {
			props: {
				settings: { maintenance_mode: false, system_announcement: 'Hello' },
				onSave: vi.fn()
			}
		});

		const toggle = screen.getByTestId('maintenance-toggle') as HTMLInputElement;
		const input = screen.getByTestId('announcement-input') as HTMLTextAreaElement;
		const saveBtn = screen.getByTestId('save-settings-btn') as HTMLButtonElement;

		expect(toggle.checked).toBe(false);
		expect(input.value).toBe('Hello');
		expect(saveBtn.disabled).toBe(true);
	});

	it('enables save button and fires onSave with modified values', async () => {
		const onSave = vi.fn();
		render(SettingsCard, {
			props: {
				settings: { maintenance_mode: false, system_announcement: null },
				onSave
			}
		});

		const toggle = screen.getByTestId('maintenance-toggle');
		await fireEvent.click(toggle);

		const saveBtn = screen.getByTestId('save-settings-btn') as HTMLButtonElement;
		expect(saveBtn.disabled).toBe(false);

		await fireEvent.click(saveBtn);
		expect(onSave).toHaveBeenCalledWith({
			maintenance_mode: true,
			system_announcement: null
		});
	});
});
```

---

## 7. Zero-Egress Playwright E2E Test (`tests/e2e/settings.test.ts`)

Playwright tests exercise user journeys against the frontend dev server. Every backend API interaction must be strictly intercepted and mocked. Tests must pass reliably without running a live API, PostgreSQL, or Redis instance.

### Network Boundary Invariants

| Practice | Strictly Required (DO) | Invariant Violation (DON'T) |
| :--- | :--- | :--- |
| **API Interception** | Intercept every `/api/**` route before `page.goto`. | **Never** let requests hit a real backend via `route.continue()` or `route.fetch()`. |
| **Unmocked Endpoints** | Record and abort the request, then assert the guard after the journey so background calls fail the test. | **Never** return generic fallback 200 `{}` envelopes to mask unmocked paths. |
| **DOM Selectors** | Use accessible queries (`getByRole`, `getByTestId`, `getByText`). | **Never** use brittle class or tag selectors (e.g. `div.container > button:nth-child(2)`). |
| **Assertions & Timing** | Use auto-waiting web assertions (`await expect(...).toBeVisible()`). | **Never** use arbitrary hardcoded sleep timers (`page.waitForTimeout(2000)`). |
| **Service Workers** | Block service workers in config (`serviceWorkers: 'block'`). | **Never** allow service workers to bypass Playwright route interception. |

### Starter Configuration and API Guard

Use [`playwright.config.ts`](../../templates/frontend/playwright.config.ts) for the dev-server-only setup and service-worker blocking. Install the reusable [`mock-api.ts`](../../templates/frontend/tests/e2e/mock-api.ts) guard before navigation, then call `assertNoUnexpectedRequests()` after the journey. Unknown `/api/**` calls are recorded and aborted, so background polling is reported even when the page handles the failed fetch.

The isolated [`mock-api.test.ts`](../../templates/frontend/tests/e2e/mock-api.test.ts) deliberately issues an unexpected background request and verifies that the guard reports it. Extend this pattern with endpoint fixtures for each product journey; mock known method/path pairs and never use a permissive fallback.

### Execution Command
```sh
bun run test:e2e
```
