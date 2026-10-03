<script lang="ts">
	import Icon from '@iconify/svelte';
	import { canvasStore } from '#lib/state/canvas.svelte';

	const winningChoice = $derived(
		canvasStore.choiceResult && canvasStore.choiceResult.result.type === 'choice'
			? canvasStore.choiceResult.result.choice
			: null
	);
</script>

<div class="flex flex-col gap-3">
	<div class="flex items-center justify-between">
		<label class="flex items-center gap-1.5 font-mono text-xs font-semibold tracking-wider text-neutral-800 uppercase dark:text-neutral-200">
			<Icon icon="lucide:list-plus" width="14" height="14" />
			<span>Choice Options (Min 2)</span>
		</label>
		<button
			type="button"
			onclick={() => canvasStore.addChoice()}
			class="flex items-center gap-1 border border-neutral-300 bg-neutral-50 px-2 py-1 font-mono text-[11px] font-medium text-neutral-700 transition-colors hover:border-neutral-900 hover:bg-neutral-100 hover:text-neutral-900 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-300 dark:hover:border-neutral-100 dark:hover:bg-neutral-700 dark:hover:text-neutral-100"
		>
			<Icon icon="lucide:plus" width="12" height="12" />
			<span>Add Option</span>
		</button>
	</div>

	<div class="flex flex-col gap-2">
		{#each canvasStore.choiceState.choices as choice, index (index)}
			{@const isWinner = winningChoice === choice.id && choice.id.trim().length > 0}
			<div
				class="group relative flex flex-col gap-2 border p-2.5 transition-all duration-150 sm:flex-row sm:items-center {isWinner
					? 'border-emerald-600 bg-emerald-50/40 dark:border-emerald-500/80 dark:bg-emerald-950/20'
					: 'border-neutral-200 bg-white dark:border-neutral-800 dark:bg-neutral-900/60'}"
			>
				<!-- Index number indicator -->
				<div class="flex items-center gap-2 sm:w-auto">
					<span class="flex h-5 w-5 items-center justify-center font-mono text-[10px] text-neutral-400 dark:text-neutral-500">
						{index + 1}
					</span>
					<div class="relative flex-1 sm:w-40">
						<input
							type="text"
							placeholder="Option name (e.g. refund)"
							value={choice.id}
							oninput={(e) => canvasStore.updateChoiceId(index, e.currentTarget.value)}
							class="w-full border border-neutral-300 bg-transparent px-2.5 py-1.5 font-mono text-xs text-neutral-900 placeholder:text-neutral-400 focus:border-neutral-900 focus:outline-none dark:border-neutral-700 dark:text-neutral-100 dark:placeholder:text-neutral-500 dark:focus:border-neutral-100"
						/>
					</div>
				</div>

				<div class="flex-1">
					<input
						type="text"
						placeholder="Description / criteria (optional)"
						value={choice.description || ''}
						oninput={(e) => canvasStore.updateChoiceDesc(index, e.currentTarget.value)}
						class="w-full border border-neutral-300 bg-transparent px-2.5 py-1.5 text-xs text-neutral-900 placeholder:text-neutral-400 focus:border-neutral-900 focus:outline-none dark:border-neutral-700 dark:text-neutral-100 dark:placeholder:text-neutral-500 dark:focus:border-neutral-100"
					/>
				</div>

				{#if isWinner}
					<span class="inline-flex items-center gap-1 border border-emerald-600 bg-emerald-100 px-1.5 py-0.5 font-mono text-[10px] font-bold text-emerald-800 uppercase dark:border-emerald-500 dark:bg-emerald-900/60 dark:text-emerald-300">
						<Icon icon="lucide:check" width="10" height="10" />
						Top
					</span>
				{/if}

				<button
					type="button"
					onclick={() => canvasStore.removeChoice(index)}
					disabled={canvasStore.choiceState.choices.length <= 2}
					class="flex h-7 w-7 items-center justify-center border border-neutral-200 text-neutral-400 transition-colors hover:border-red-400 hover:bg-red-50 hover:text-red-600 disabled:cursor-not-allowed disabled:opacity-30 dark:border-neutral-800 dark:text-neutral-500 dark:hover:border-red-700 dark:hover:bg-red-950/40 dark:hover:text-red-400"
					title="Remove option"
				>
					<Icon icon="lucide:trash-2" width="12" height="12" />
				</button>
			</div>
		{/each}
	</div>
</div>
