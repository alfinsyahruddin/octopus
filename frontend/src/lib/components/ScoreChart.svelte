<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { ScoreResult } from '#lib/types';

	interface Props {
		result: ScoreResult;
	}

	let { result }: Props = $props();

	const levelKeys = $derived(Object.keys(result.probabilities).sort((a, b) => Number(a) - Number(b)));
	const maxLevelIndex = $derived(Math.max(1, levelKeys.length - 1));

	// Percentage position along scale (0 to 100%)
	const scorePercentage = $derived(
		Math.min(100, Math.max(0, (result.score / maxLevelIndex) * 100))
	);

	function getConfidenceLabel(conf: number): { label: string; colorClass: string } {
		if (conf >= 0.8) return { label: 'High Confidence', colorClass: 'text-emerald-600 dark:text-emerald-400 border-emerald-500/50 bg-emerald-50 dark:bg-emerald-950/40' };
		if (conf >= 0.5) return { label: 'Moderate Confidence', colorClass: 'text-amber-600 dark:text-amber-400 border-amber-500/50 bg-amber-50 dark:bg-amber-950/40' };
		return { label: 'Ambiguous / Low Confidence', colorClass: 'text-rose-600 dark:text-rose-400 border-rose-500/50 bg-rose-50 dark:bg-rose-950/40' };
	}

	const confidenceInfo = $derived(getConfidenceLabel(result.confidence));
</script>

<div class="flex flex-col gap-6">
	<!-- Top Outcome Banner: Weighted Continuous Score -->
	<div class="flex flex-col gap-2 border border-neutral-900 bg-neutral-900 p-4 text-white dark:border-neutral-100 dark:bg-neutral-100 dark:text-neutral-900">
		<div class="flex items-center justify-between">
			<span class="font-mono text-[10px] font-bold tracking-widest uppercase opacity-75">
				Probability-Weighted Mean Score
			</span>
			<span class="border border-white/30 px-2 py-0.5 font-mono text-[11px] font-semibold dark:border-black/30">
				Scale: 0.0 – {maxLevelIndex}.0
			</span>
		</div>
		<div class="flex items-baseline gap-2">
			<span class="font-mono text-3xl font-bold tracking-tight sm:text-4xl">
				{result.score.toFixed(2)}
			</span>
			<span class="font-mono text-xs opacity-75">
				/ {maxLevelIndex}.00
			</span>
		</div>
	</div>

	<!-- Continuous Position Gauge -->
	<div class="flex flex-col gap-2 border border-dashed border-neutral-300 bg-white p-4 dark:border-neutral-800 dark:bg-neutral-900">
		<div class="flex items-center justify-between text-xs">
			<span class="font-mono font-semibold uppercase text-neutral-800 dark:text-neutral-200">
				Continuous Scale Position
			</span>
			<span class="font-mono text-neutral-500">
				{scorePercentage.toFixed(1)}% along spectrum
			</span>
		</div>

		<!-- Track with Level ticks and Marker -->
		<div class="relative my-3 h-3 w-full bg-neutral-100 dark:bg-neutral-800">
			<!-- Colored Fill -->
			<div
				class="h-full bg-neutral-900 transition-all duration-300 ease-out dark:bg-white"
				style="width: {scorePercentage}%;"
			></div>

			<!-- Score Needle/Marker -->
			<div
				class="absolute top-1/2 -ml-2 -mt-3.5 h-7 w-4 border-2 border-neutral-900 bg-white shadow-md transition-all duration-300 ease-out dark:border-white dark:bg-neutral-950"
				style="left: {scorePercentage}%;"
			></div>
		</div>

		<!-- Level labels below track -->
		<div class="flex justify-between text-[11px] text-neutral-500">
			{#each levelKeys as key}
				<span class="font-mono">
					{key}: {result.legend[key] || `Level ${key}`}
				</span>
			{/each}
		</div>
	</div>

	<!-- Confidence Metric -->
	<div class="flex items-center justify-between border border-neutral-200 bg-neutral-50/50 p-3 dark:border-neutral-800 dark:bg-neutral-900/40">
		<div class="flex items-center gap-1.5 font-mono text-xs text-neutral-700 dark:text-neutral-300">
			<Icon icon="lucide:shield-check" width="14" height="14" class="text-neutral-500" />
			<span>Confidence Metric</span>
		</div>
		<span class="border px-2 py-0.5 font-mono text-[11px] font-bold {confidenceInfo.colorClass}">
			{confidenceInfo.label} ({(result.confidence * 100).toFixed(1)}%)
		</span>
	</div>

	<!-- Level Probability Breakdown -->
	<div class="flex flex-col gap-3">
		<span class="font-mono text-xs font-bold uppercase tracking-wider text-neutral-800 dark:text-neutral-200">
			Level Probability Breakdown
		</span>

		<div class="flex flex-col gap-2.5">
			{#each levelKeys as key (key)}
				{@const prob = result.probabilities[key] || 0}
				{@const percentage = (prob * 100).toFixed(1)}
				{@const desc = result.legend[key] || `Level ${key}`}
				<div class="flex flex-col gap-1">
					<div class="flex items-center justify-between text-xs">
						<div class="flex items-center gap-2">
							<span class="font-mono font-bold text-neutral-900 dark:text-neutral-100">
								[{key}] {desc}
							</span>
						</div>
						<div class="flex items-center gap-2 font-mono text-xs">
							<span class="text-neutral-400">({prob.toFixed(4)})</span>
							<span class="font-bold text-neutral-800 dark:text-neutral-200">
								{percentage}%
							</span>
						</div>
					</div>

					<div class="h-2.5 w-full overflow-hidden border border-neutral-200 bg-neutral-100 dark:border-neutral-800 dark:bg-neutral-900">
						<div
							class="h-full bg-neutral-900 transition-all duration-300 ease-out dark:bg-white"
							style="width: {percentage}%;"
						></div>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
