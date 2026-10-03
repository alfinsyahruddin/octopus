<script lang="ts">
	import Icon from '@iconify/svelte';
	import { canvasStore } from '#lib/state/canvas.svelte';
</script>

<div class="flex flex-col gap-3">
	<div class="flex items-center justify-between">
		<label class="flex items-center gap-1.5 font-mono text-xs font-semibold tracking-wider text-neutral-800 uppercase dark:text-neutral-200">
			<Icon icon="lucide:sliders-horizontal" width="14" height="14" />
			<span>Ordered Scale Levels (Min 2)</span>
		</label>
		<button
			type="button"
			onclick={() => canvasStore.addScoreLevel()}
			class="flex items-center gap-1 border border-neutral-300 bg-neutral-50 px-2 py-1 font-mono text-[11px] font-medium text-neutral-700 transition-colors hover:border-neutral-900 hover:bg-neutral-100 hover:text-neutral-900 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-300 dark:hover:border-neutral-100 dark:hover:bg-neutral-700 dark:hover:text-neutral-100"
		>
			<Icon icon="lucide:plus" width="12" height="12" />
			<span>Add Level</span>
		</button>
	</div>

	<div class="flex flex-col gap-2">
		{#each canvasStore.scoreState.levels as level, index (index)}
			<div
				class="group relative flex items-center gap-2 border border-neutral-200 bg-white p-2.5 dark:border-neutral-800 dark:bg-neutral-900/60"
			>
				<span class="flex h-6 w-12 items-center justify-center border border-neutral-300 bg-neutral-100 font-mono text-[11px] font-bold text-neutral-700 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-300">
					lvl {index}
				</span>

				<div class="flex-1">
					<input
						type="text"
						placeholder="Level {index} criteria description (e.g. Low / Medium / High)"
						value={level}
						oninput={(e) => canvasStore.updateScoreLevel(index, e.currentTarget.value)}
						class="w-full border border-neutral-300 bg-transparent px-2.5 py-1.5 text-xs text-neutral-900 placeholder:text-neutral-400 focus:border-neutral-900 focus:outline-none dark:border-neutral-700 dark:text-neutral-100 dark:placeholder:text-neutral-500 dark:focus:border-neutral-100"
					/>
				</div>

				<button
					type="button"
					onclick={() => canvasStore.removeScoreLevel(index)}
					disabled={canvasStore.scoreState.levels.length <= 2}
					class="flex h-7 w-7 items-center justify-center border border-neutral-200 text-neutral-400 transition-colors hover:border-red-400 hover:bg-red-50 hover:text-red-600 disabled:cursor-not-allowed disabled:opacity-30 dark:border-neutral-800 dark:text-neutral-500 dark:hover:border-red-700 dark:hover:bg-red-950/40 dark:hover:text-red-400"
					title="Remove level"
				>
					<Icon icon="lucide:trash-2" width="12" height="12" />
				</button>
			</div>
		{/each}
	</div>
</div>
