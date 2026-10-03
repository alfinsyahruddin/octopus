# Frontend Architecture & Code Patterns

This guide defines the architectural standards, layer boundaries, and inviolable code rules for building SvelteKit single-page applications with Svelte 5 runes.

---

## 1. Architectural Layers & Unidirectional Data Flow

User actions flow through strictly segregated layers in one direction:

```text
┌─────────────────────────────────────────────────────────────────────┐
│                            User Action                              │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Client Routes (`src/routes/`)                                       │
│ └─ CSR page layouts, navigation guards, route parameter binding     │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│ UI Components (`src/lib/components/`)                               │
│ └─ Svelte 5 Runes ($props, $state, $derived), presentation, inputs  │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Reactive State Modules (`src/lib/helpers/*.svelte.ts`)              │
│ └─ Shared client stores, mutation methods, optimistic updates       │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Centralized API Transport (`src/lib/api.ts`)                        │
│ └─ Typed request wrapper, auto 401 token refresh, error mapping     │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Backend REST API (Actix-web)                                        │
│ └─ Unified API envelope: AppResponse<T>                             │
└─────────────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities
- **`src/routes/`**: Client-side routes (`+layout.svelte`, `+page.svelte`). Operates strictly in pure CSR mode (`export const ssr = false;` in root `+layout.ts`). Never create server routes (`+page.server.ts` or `+server.ts`).
- **`src/lib/components/`**: Reusable Svelte 5 components. Must use modern Runes exclusively (`$props`, `$state`, `$derived`, `$effect`, `onclick`). Focuses on presentation, user events, and scoped styling.
- **`src/lib/helpers/`**: Reactive state modules encapsulated in `.svelte.ts` files, client stores, and pure client utility functions.
- **`src/lib/api.ts`**: The single transport gateway for all HTTP calls. Handles authentication headers, 401 token refresh deduplication, and maps HTTP error statuses to typed `ApiError` instances.
- **`src/lib/types/`**: TypeScript interfaces defining domain request/response contracts, matching backend DTO schemas.

---

## 2. Inviolable Frontend Rules: Do's & Don'ts

### Svelte 5 Runes Cheat Sheet

> [!CAUTION]
> Never generate legacy Svelte 3/4 syntax. Always use Svelte 5 Runes:

| Pattern | Legacy Svelte 3/4 (FORBIDDEN) | Modern Svelte 5 Runes (MANDATORY) |
| :--- | :--- | :--- |
| **Component Props** | `export let title = 'Default';` | `let { title = 'Default' }: Props = $props();` |
| **Local State** | `let count = 0;` | `let count = $state(0);` |
| **Computed / Derived** | `$: doubled = count * 2;` | `const doubled = $derived(count * 2);` |
| **Side Effects** | `$: console.log(count);` | `$effect(() => { console.log(count); });` |
| **Event Listeners** | `<button on:click={handleClick}>` | `<button onclick={handleClick}>` |
| **Dynamic Snippets** | `<slot />` or `<slot name="icon" />` | `{#snippet children()}{/snippet}` and `render` |

### Guardrails: Do's & Don'ts

| Area | Strictly Required (DO) | Invariant Violation (DON'T) |
| :--- | :--- | :--- |
| **Reactivity Syntax** | Use Svelte 5 runes (`$props`, `$state`, `$derived`). | **Never** generate legacy Svelte 3/4 syntax (`export let`, `$:`, `on:click`). |
| **Render Mode** | Pure CSR (`ssr = false` in root `+layout.ts`). | **Never** create server routes (`+page.server.ts` or `+server.ts`). |
| **Component Styling** | Colocate utility classes in components or scoped `<style>`. | **Never** use `@layer components` or global `@apply` for feature styling. |
| **API Transport** | Call backend endpoints via centralized `request<T>()`. | **Never** invoke raw `fetch()` directly in UI components. |
| **E2E Testing** | Intercept 100% of network calls with Playwright mock fixtures. | **Never** allow E2E tests to egress to a live backend API. |
| **Package Manager** | Use `bun` for script running and package installation. | **Never** run `npm`, `yarn`, or `pnpm`. |
| **Library Imports** | Declare `#lib/*` in `package.json`; use `.js` extensions for TypeScript modules and `.svelte` for components. | **Never** rely on the removed `$lib` alias or extensionless library imports. |

### UI Component Reference Implementation (`src/lib/components/ToastViewport.svelte`)

Demonstrates reactive module state integration, keyed `{#each}` loops, enter/exit transitions (`fly`), Lucide vector iconography, and semantic theme styling (template available at [`templates/frontend/src/lib/components/ToastViewport.svelte`](../../templates/frontend/src/lib/components/ToastViewport.svelte)):

```svelte
<script lang="ts">
	import Icon from '@iconify/svelte';
	import { toasts, dismissToast } from '#lib/helpers/toast.svelte.js';
	import { fly } from 'svelte/transition';
	import { backOut, backIn } from 'svelte/easing';
</script>

<div
	class="pointer-events-none fixed inset-x-4 bottom-4 z-100 flex flex-col gap-2.5 sm:left-auto sm:w-88"
>
	{#each toasts as toast (toast.id)}
		<div
			in:fly={{ x: 150, duration: 400, easing: backOut }}
			out:fly={{ x: 150, duration: 300, easing: backIn }}
			class="pointer-events-auto flex items-center gap-3 rounded-2xl border px-4 py-3.5 shadow-xl backdrop-blur-xl"
			style="
				background-color: {toast.type === 'success'
				? 'rgba(34, 197, 94, 0.14)'
				: toast.type === 'error'
					? 'rgba(239, 68, 68, 0.14)'
					: 'rgba(48, 180, 201, 0.14)'};
				border-color: {toast.type === 'success'
				? 'rgba(34, 197, 94, 0.35)'
				: toast.type === 'error'
					? 'rgba(239, 68, 68, 0.35)'
					: 'rgba(48, 180, 201, 0.35)'};
				box-shadow: 0 12px 30px -4px rgba(0, 0, 0, 0.15), 0 4px 12px -2px rgba(0, 0, 0, 0.08);
			"
		>
			<!-- Icon -->
			<div
				class="shrink-0"
				style="color: {toast.type === 'success'
					? 'var(--success, #22c55e)'
					: toast.type === 'error'
						? 'var(--danger, #ef4444)'
						: 'var(--accent, #30b4c9)'}"
			>
				{#if toast.type === 'success'}
					<Icon icon="lucide:check-circle" width="18" height="18" />
				{:else if toast.type === 'error'}
					<Icon icon="lucide:alert-circle" width="18" height="18" />
				{:else}
					<Icon icon="lucide:info" width="18" height="18" />
				{/if}
			</div>

			<!-- Message -->
			<p class="font-600 flex-1 text-sm leading-snug" style="color: var(--fg)">
				{toast.message}
			</p>

			<!-- Dismiss button -->
			<button
				onclick={() => dismissToast(toast.id)}
				class="btn-interactive -mr-1 rounded-lg p-1 transition-colors duration-150 hover:bg-black/5 dark:hover:bg-white/10 cursor-pointer"
				style="color: var(--fg-muted);"
				aria-label="Dismiss notification"
			>
				<Icon icon="lucide:x" width="14" height="14" />
			</button>
		</div>
	{/each}
</div>
```

---

## 3. Sub-Rule Directory Map

Follow these detailed guides to implement each frontend layer:

| Focus Area | Guide | Key Topics |
| :--- | :--- | :--- |
| **Foundations** | [Frontend Foundations](foundations.md) | `api.ts`, `app.css`, `+layout.ts`, `+layout.svelte`, SvelteKit 3 `vite.config.ts`, `tsconfig.json`, `vitest.config.ts`, `nginx.conf` |
| **Vertical Slice** | [Vertical Slice Feature](feature.md) | Full end-to-end feature: Types → API client → Reactive helper → UI component → Route → Unit test → E2E test |
| **Optional Components** | [Frontend Optional Components](optional-components.md) | Auth state, navigation guards, `ThemeToggle.svelte`, toast system, complete `package.json` |
