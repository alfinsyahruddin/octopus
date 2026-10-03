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
	class="btn-interactive flex size-9 items-center justify-center rounded-lg transition-colors duration-150 hover:bg-(--bg-card-hover)"
	style="color: var(--fg-muted);"
	aria-label={ariaLabel}
	title={isDark ? 'Switch to light mode' : 'Switch to dark mode'}
>
	{#if isDark}
		<Icon icon="lucide:sun" width="18" height="18" />
	{:else}
		<Icon icon="lucide:moon" width="18" height="18" />
	{/if}
</button>
