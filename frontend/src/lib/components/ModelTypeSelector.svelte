<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { DecisionModelType } from '#lib/types';
	import { canvasStore } from '#lib/state/canvas.svelte';

	interface TabDef {
		type: DecisionModelType;
		label: string;
		icon: string;
		badge: string;
		description: string;
	}

	const tabs: TabDef[] = [
		{
			type: 'choice',
			label: 'Choice',
			icon: 'lucide:list-ordered',
			badge: 'Categorical',
			description: 'Pick 1 from options with probability distribution'
		},
		{
			type: 'score',
			label: 'Score',
			icon: 'lucide:gauge',
			badge: 'Continuous',
			description: 'Probability-weighted mean on ordered scale'
		},
		{
			type: 'noul',
			label: 'Noul',
			icon: 'lucide:binary',
			badge: 'Binary Prob',
			description: 'Calibrated certainty for yes/no conditions'
		}
	];
</script>

<div class="grid grid-cols-1 gap-2 sm:grid-cols-3">
	{#each tabs as tab}
		{@const isActive = canvasStore.activeType === tab.type}
		<button
			type="button"
			onclick={() => canvasStore.setActiveType(tab.type)}
			class="group relative flex flex-col p-3.5 text-left transition-all duration-200 {isActive
				? 'border-2 border-neutral-900 bg-white shadow-sm dark:border-neutral-100 dark:bg-neutral-900'
				: 'border border-dashed border-neutral-300 bg-neutral-50/60 hover:border-neutral-500 hover:bg-white dark:border-neutral-800 dark:bg-neutral-900/40 dark:hover:border-neutral-600 dark:hover:bg-neutral-900'}"
		>
			<!-- Corner accent indicator for active tab -->
			{#if isActive}
				<div class="absolute -top-1 -left-1 h-2 w-2 bg-neutral-900 dark:bg-neutral-100"></div>
				<div class="absolute -bottom-1 -right-1 h-2 w-2 bg-neutral-900 dark:bg-neutral-100"></div>
			{/if}

			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<Icon
						icon={tab.icon}
						width="16"
						height="16"
						class={isActive ? 'text-neutral-900 dark:text-neutral-100' : 'text-neutral-400 group-hover:text-neutral-600 dark:text-neutral-500 dark:group-hover:text-neutral-300'}
					/>
					<span class="font-mono text-sm font-bold uppercase tracking-wider {isActive ? 'text-neutral-900 dark:text-neutral-100' : 'text-neutral-700 dark:text-neutral-300'}">
						{tab.label}
					</span>
				</div>
				<span
					class="border px-1.5 py-0.5 font-mono text-[10px] uppercase {isActive
						? 'border-neutral-900 bg-neutral-900 text-white dark:border-neutral-100 dark:bg-neutral-100 dark:text-neutral-900'
						: 'border-neutral-300 text-neutral-500 dark:border-neutral-700 dark:text-neutral-400'}"
				>
					{tab.badge}
				</span>
			</div>

			<p class="mt-2 text-xs leading-relaxed text-neutral-500 dark:text-neutral-400">
				{tab.description}
			</p>
		</button>
	{/each}
</div>
