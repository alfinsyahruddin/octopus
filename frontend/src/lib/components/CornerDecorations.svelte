<script lang="ts">
	interface Props {
		class?: string;
		dashed?: boolean;
		size?: number;
		children?: import('svelte').Snippet;
	}

	let { class: className = '', dashed = true, size = 10, children }: Props = $props();
</script>

<div class="relative {className} {dashed ? 'border border-dashed border-neutral-300 dark:border-neutral-800' : ''}">
	<!-- Top Left Right-Angled Triangle Line -->
	<svg
		class="pointer-events-none absolute -top-[1px] -left-[1px] text-neutral-900 dark:text-neutral-100"
		width={size}
		height={size}
		viewBox="0 0 12 12"
		fill="none"
	>
		<path d="M0 12V0H12" stroke="currentColor" stroke-width="2" />
	</svg>

	<!-- Top Right Right-Angled Triangle Line -->
	<svg
		class="pointer-events-none absolute -top-[1px] -right-[1px] text-neutral-900 dark:text-neutral-100"
		width={size}
		height={size}
		viewBox="0 0 12 12"
		fill="none"
	>
		<path d="M0 0H12V12" stroke="currentColor" stroke-width="2" />
	</svg>

	<!-- Bottom Left Right-Angled Triangle Line -->
	<svg
		class="pointer-events-none absolute -bottom-[1px] -left-[1px] text-neutral-900 dark:text-neutral-100"
		width={size}
		height={size}
		viewBox="0 0 12 12"
		fill="none"
	>
		<path d="M0 0V12H12" stroke="currentColor" stroke-width="2" />
	</svg>

	<!-- Bottom Right Right-Angled Triangle Line -->
	<svg
		class="pointer-events-none absolute -bottom-[1px] -right-[1px] text-neutral-900 dark:text-neutral-100"
		width={size}
		height={size}
		viewBox="0 0 12 12"
		fill="none"
	>
		<path d="M12 0V12H0" stroke="currentColor" stroke-width="2" />
	</svg>

	{#if children}
		{@render children()}
	{/if}
</div>
