<script lang="ts">
	import '../app.css';
	import { onMount, type Snippet } from 'svelte';
	import Header from '#lib/components/Header.svelte';
	import ToastViewport from '#lib/components/ToastViewport.svelte';
	import { getCurrentTheme, applyTheme } from '#lib/helpers/theme';
	import { canvasStore } from '#lib/state/canvas.svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	onMount(() => {
		applyTheme(getCurrentTheme());
		canvasStore.initModels();
	});
</script>

<div class="flex min-h-screen flex-col bg-[var(--bg)] text-[var(--fg)] antialiased transition-colors duration-200">
	<Header />

	<main class="flex-1">
		{@render children()}
	</main>

	<footer class="border-t border-neutral-200/80 bg-neutral-50/50 py-6 text-center text-xs text-neutral-500 dark:border-neutral-800/80 dark:bg-neutral-950/50 dark:text-neutral-400">
		<div class="mx-auto flex max-w-7xl flex-col items-center justify-between gap-3 px-4 sm:flex-row sm:px-6 lg:px-8">
			<div class="flex items-center gap-2 font-mono text-[11px]">
				<span class="inline-block h-2 w-2 bg-neutral-900 dark:bg-neutral-100"></span>
				<span class="font-bold uppercase tracking-wider text-neutral-800 dark:text-neutral-200">Octopus</span>
				<span class="text-neutral-400">— System One Decision Model Playground</span>
			</div>
			<div class="flex items-center gap-4 font-mono text-[11px] text-neutral-400">
				<span>Non-Autoregressive</span>
				<span>•</span>
				<span>Calibrated Probabilities</span>
				<span>•</span>
				<span>Zero Hallucination</span>
			</div>
		</div>
	</footer>

	<ToastViewport />
</div>
