<script lang="ts">
	import Icon from '@iconify/svelte';
	import ThemeToggle from '#lib/components/ThemeToggle.svelte';
	import { toast } from '#lib/helpers/toast.svelte.js';
</script>

<svelte:head>
	<title>Welcome | App</title>
</svelte:head>

<div class="flex min-h-screen flex-col">
	<header class="flex h-16 items-center justify-between border-b border-(--border) px-6">
		<div class="flex items-center gap-2 font-bold text-lg">
			<Icon icon="lucide:layers" class="text-(--color-accent)" width="24" height="24" />
			<span>App</span>
		</div>
		<ThemeToggle />
	</header>

	<main class="flex flex-1 flex-col items-center justify-center p-6 text-center">
		<h1 class="text-4xl font-extrabold tracking-tight sm:text-5xl text-(--fg)">
			Svelte 5 + Bun + Tailwind v4
		</h1>
		<p class="mt-4 max-w-lg text-lg text-(--fg-muted)">
			Single-Page Application powered by pure CSR mode, Runes, and typed API client transport.
		</p>
		<div class="mt-8 flex gap-4">
			<button
				type="button"
				onclick={() => toast.success('Notification triggered!')}
				class="inline-flex items-center gap-2 rounded-lg bg-(--color-accent) px-5 py-2.5 text-sm font-medium text-white transition-opacity hover:opacity-90 cursor-pointer"
			>
				<Icon icon="lucide:bell" width="16" height="16" />
				Test Toast
			</button>
		</div>
	</main>
</div>
