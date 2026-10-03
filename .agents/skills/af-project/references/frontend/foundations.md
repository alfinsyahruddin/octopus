# Frontend Foundations

Use the starter files as the source of truth for the centralized API client, CSS tokens, CSR shell, and SvelteKit/Vite configuration. This reference explains the integration decisions without repeating those implementations.

## API transport

All network calls go through [`src/lib/api.ts`](../../templates/frontend/src/lib/api.ts) and its `request<T>()` function. Keep transport concerns there: JSON headers, bearer token attachment, response-envelope parsing, typed `ApiError`s, and one deduplicated refresh attempt after `401`. UI components call domain API modules and should not call `fetch()` directly.

Set `PUBLIC_API_BASE_URL` for the target environment. The template fallback is intended for local development. Auth state and refresh policy are optional product choices; when auth is not used, remove refresh behavior together with its storage assumptions.

## CSR shell and styling

- [`+layout.ts`](../../templates/frontend/src/routes/+layout.ts) disables SSR and prerendering. Keep this application in pure CSR mode; do not add SvelteKit server routes.
- [`+layout.svelte`](../../templates/frontend/src/routes/+layout.svelte) imports the global stylesheet, renders the route snippet, and mounts the shared toast viewport.
- [`app.html`](../../templates/frontend/src/app.html) is the document shell. Keep app-specific metadata and shell attributes here.
- [`app.css`](../../templates/frontend/src/app.css) owns Tailwind CSS v4 setup, dark-mode variant, theme tokens, semantic color variables, and global resets. Put feature styling in component utility classes or scoped styles; avoid global feature abstractions and `@layer components`.

## Build and test configuration

The authoritative setup is in [`vite.config.ts`](../../templates/frontend/vite.config.ts), [`tsconfig.json`](../../templates/frontend/tsconfig.json), [`vitest.config.ts`](../../templates/frontend/vitest.config.ts), and [`playwright.config.ts`](../../templates/frontend/playwright.config.ts). The matching scripts and dependency versions live in [`package.json`](../../templates/frontend/package.json).

When adapting these files, preserve the `#lib/*` import mapping and `.js` extensions on TypeScript imports. Keep Vitest on jsdom and Playwright pointed at the frontend dev server only. Playwright must block service workers and intercept every API request with a known mock; see [the E2E feature guidance](feature.md#7-zero-egress-playwright-e2e-test).

## Adding a foundation module

1. Copy the required files from [`templates/frontend/`](../../templates/frontend/).
2. Align package scripts, imports, and configuration paths with the copied files.
3. Keep API types aligned with backend DTOs and add focused unit tests for parsing, auth refresh, and error behavior that the project uses.
4. Add an E2E mock for every endpoint the page needs before navigation. Unknown requests must be recorded, aborted, and asserted after the journey so background polling cannot go unnoticed.
