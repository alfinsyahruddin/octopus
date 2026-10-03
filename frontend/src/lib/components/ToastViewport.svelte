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
