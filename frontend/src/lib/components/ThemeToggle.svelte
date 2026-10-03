<script lang="ts">
	import Icon from '@iconify/svelte';
	import { toggleTheme, getCurrentTheme } from '#lib/helpers/theme.js';

	interface Props {
		ariaLabel?: string;
	}

	const { ariaLabel = 'Toggle theme' }: Props = $props();

	let theme = $state<'dark' | 'light'>(getCurrentTheme());
	const isDark = $derived(theme === 'dark');

	function handleToggle() {
		theme = toggleTheme();
	}
</script>

<button
	type="button"
	onclick={handleToggle}
	class="relative flex h-8 w-8 items-center justify-center border border-neutral-300 bg-neutral-100 text-neutral-800 transition-colors hover:border-neutral-900 hover:bg-neutral-200 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-200 dark:hover:border-neutral-100 dark:hover:bg-neutral-700"
	aria-label={ariaLabel}
	title={isDark ? 'Switch to light mode' : 'Switch to dark mode'}
>
	{#if isDark}
		<Icon icon="lucide:sun" width="15" height="15" />
	{:else}
		<Icon icon="lucide:moon" width="15" height="15" />
	{/if}
</button>
