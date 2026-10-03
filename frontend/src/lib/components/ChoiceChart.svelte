<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { ChoiceResult } from '#lib/types';

	interface Props {
		result: ChoiceResult;
	}

	let { result }: Props = $props();

	// Sort options by probability descending
	const sortedEntries = $derived.by(() => {
		const entries = Object.entries(result.probabilities);
		return entries.sort(([, a], [, b]) => (b as number) - (a as number)) as [string, number][];
	});

	function getConfidenceLabel(conf: number): { label: string; colorClass: string } {
		if (conf >= 0.8) return { label: 'High Confidence', colorClass: 'text-emerald-600 dark:text-emerald-400 border-emerald-500/50 bg-emerald-50 dark:bg-emerald-950/40' };
		if (conf >= 0.5) return { label: 'Moderate Confidence', colorClass: 'text-amber-600 dark:text-amber-400 border-amber-500/50 bg-amber-50 dark:bg-amber-950/40' };
		return { label: 'Ambiguous / Low Confidence', colorClass: 'text-rose-600 dark:text-rose-400 border-rose-500/50 bg-rose-50 dark:bg-rose-950/40' };
	}

	const confidenceInfo = $derived(getConfidenceLabel(result.confidence));
</script>

<div class="flex flex-col gap-6">
	<!-- Top Outcome Banner -->
	<div class="flex flex-col gap-2 border border-neutral-900 bg-neutral-900 p-4 text-white dark:border-neutral-100 dark:bg-neutral-100 dark:text-neutral-900">
		<div class="flex items-center justify-between">
			<span class="font-mono text-[10px] font-bold tracking-widest uppercase opacity-75">
				Selected Choice Winner
			</span>
			<span class="border border-white/30 px-2 py-0.5 font-mono text-[11px] font-semibold dark:border-black/30">
				{(result.probabilities[result.choice] * 100 || 0).toFixed(1)}% prob
			</span>
		</div>
		<div class="flex items-baseline gap-3">
			<span class="font-mono text-2xl font-bold tracking-tight uppercase sm:text-3xl">
				{result.choice}
			</span>
		</div>
	</div>

	<!-- Confidence Metric Indicator -->
	<div class="flex flex-col gap-2 border border-dashed border-neutral-300 bg-white p-3.5 dark:border-neutral-800 dark:bg-neutral-900">
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-1.5">
				<Icon icon="lucide:shield-check" width="14" height="14" class="text-neutral-500" />
				<span class="font-mono text-xs font-semibold uppercase tracking-wider text-neutral-800 dark:text-neutral-200">
					Calibrated Confidence
				</span>
			</div>
			<span class="border px-2 py-0.5 font-mono text-[11px] font-bold {confidenceInfo.colorClass}">
				{confidenceInfo.label} ({(result.confidence * 100).toFixed(1)}%)
			</span>
		</div>
		<!-- Confidence Bar -->
		<div class="h-2 w-full overflow-hidden bg-neutral-100 dark:bg-neutral-800">
			<div
				class="h-full bg-neutral-900 transition-all duration-300 ease-out dark:bg-neutral-100"
				style="width: {Math.min(100, Math.max(0, result.confidence * 100))}%;"
			></div>
		</div>
	</div>

	<!-- Probability Distribution Bars -->
	<div class="flex flex-col gap-3">
		<div class="flex items-center justify-between">
			<span class="font-mono text-xs font-bold uppercase tracking-wider text-neutral-800 dark:text-neutral-200">
				Probability Distribution
			</span>
			<span class="font-mono text-[11px] text-neutral-400">Sum: 1.0</span>
		</div>

		<div class="flex flex-col gap-2.5">
			{#each sortedEntries as [option, prob] (option)}
				{@const isTop = option === result.choice}
				{@const percentage = (prob * 100).toFixed(1)}
				<div class="flex flex-col gap-1">
					<div class="flex items-center justify-between text-xs">
						<div class="flex items-center gap-2">
							<span class="font-mono font-bold {isTop ? 'text-emerald-600 dark:text-emerald-400' : 'text-neutral-800 dark:text-neutral-200'}">
								{option}
							</span>
							{#if isTop}
								<span class="border border-emerald-600 bg-emerald-100 px-1 py-0.2 font-mono text-[9px] font-bold text-emerald-800 uppercase dark:border-emerald-500 dark:bg-emerald-900/60 dark:text-emerald-300">
									Top
								</span>
							{/if}
						</div>
						<div class="flex items-center gap-2 font-mono text-xs">
							<span class="text-neutral-400">({prob.toFixed(4)})</span>
							<span class="font-bold {isTop ? 'text-neutral-900 dark:text-neutral-100' : 'text-neutral-600 dark:text-neutral-400'}">
								{percentage}%
							</span>
						</div>
					</div>

					<!-- Visual bar with soft color -->
					<div class="h-3 w-full overflow-hidden border border-neutral-200 bg-neutral-100 dark:border-neutral-800 dark:bg-neutral-900">
						<div
							class="h-full transition-all duration-300 ease-out {isTop
								? 'bg-emerald-500 dark:bg-emerald-400'
								: 'bg-neutral-400 dark:bg-neutral-600'}"
							style="width: {percentage}%;"
						></div>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
