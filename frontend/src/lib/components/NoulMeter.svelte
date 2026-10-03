<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { NoulResult } from '#lib/types';

	interface Props {
		result: NoulResult;
	}

	let { result }: Props = $props();

	const trueProb = $derived(result.noul);
	const falseProb = $derived(1 - result.noul);
	const isTrue = $derived(trueProb >= 0.5);

	// Distance from 0.5 scaled to 0..100%
	const certaintyScore = $derived(Math.abs(trueProb - 0.5) * 200);

	function getCertaintyInterpretation(c: number): { text: string; colorClass: string } {
		if (c >= 70) return { text: 'High Certainty', colorClass: 'text-emerald-600 dark:text-emerald-400 border-emerald-500/50 bg-emerald-50 dark:bg-emerald-950/40' };
		if (c >= 30) return { text: 'Moderate Certainty', colorClass: 'text-amber-600 dark:text-amber-400 border-amber-500/50 bg-amber-50 dark:bg-amber-950/40' };
		return { text: 'Maximum Uncertainty / Ambiguous', colorClass: 'text-rose-600 dark:text-rose-400 border-rose-500/50 bg-rose-50 dark:bg-rose-950/40' };
	}

	const certaintyInfo = $derived(getCertaintyInterpretation(certaintyScore));
</script>

<div class="flex flex-col gap-6">
	<!-- Top Outcome Banner -->
	<div
		class="flex flex-col gap-2 border p-4 transition-colors {isTrue
			? 'border-emerald-600 bg-emerald-600 text-white dark:border-emerald-500 dark:bg-emerald-950 dark:text-emerald-100'
			: 'border-rose-600 bg-rose-600 text-white dark:border-rose-500 dark:bg-rose-950 dark:text-rose-100'}"
	>
		<div class="flex items-center justify-between">
			<span class="font-mono text-[10px] font-bold tracking-widest uppercase opacity-85">
				Calibrated Decision Outcome
			</span>
			<span class="border border-white/40 px-2 py-0.5 font-mono text-[11px] font-semibold dark:border-current">
				P(True) = {trueProb.toFixed(4)}
			</span>
		</div>
		<div class="flex items-baseline gap-3">
			<span class="font-mono text-3xl font-bold tracking-tight uppercase sm:text-4xl">
				{isTrue ? 'YES' : 'NO'}
			</span>
			<span class="font-mono text-sm opacity-90">
				({(Math.max(trueProb, falseProb) * 100).toFixed(1)}% likelihood)
			</span>
		</div>
	</div>

	<!-- Certainty from Distance to 0.5 -->
	<div class="flex flex-col gap-2 border border-dashed border-neutral-300 bg-white p-3.5 dark:border-neutral-800 dark:bg-neutral-900">
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-1.5">
				<Icon icon="lucide:activity" width="14" height="14" class="text-neutral-500" />
				<span class="font-mono text-xs font-semibold uppercase tracking-wider text-neutral-800 dark:text-neutral-200">
					Certainty Metric (|P - 0.5| × 2)
				</span>
			</div>
			<span class="border px-2 py-0.5 font-mono text-[11px] font-bold {certaintyInfo.colorClass}">
				{certaintyInfo.text} ({certaintyScore.toFixed(1)}%)
			</span>
		</div>
		<div class="h-2 w-full overflow-hidden bg-neutral-100 dark:bg-neutral-800">
			<div
				class="h-full bg-neutral-900 transition-all duration-300 ease-out dark:bg-neutral-100"
				style="width: {certaintyScore}%;"
			></div>
		</div>
	</div>

	<!-- Dual Gauge Comparison (No vs Yes) -->
	<div class="flex flex-col gap-3">
		<span class="font-mono text-xs font-bold uppercase tracking-wider text-neutral-800 dark:text-neutral-200">
			Calibrated Probability Distribution
		</span>

		<!-- Split Bar -->
		<div class="flex h-10 w-full overflow-hidden border border-neutral-300 bg-neutral-100 font-mono text-xs font-bold dark:border-neutral-700 dark:bg-neutral-800">
			<!-- False Segment -->
			<div
				class="flex items-center justify-start bg-rose-500/80 px-3 text-white transition-all duration-300 ease-out dark:bg-rose-600/80"
				style="width: {falseProb * 100}%;"
			>
				{#if falseProb > 0.12}
					<span>False ({(falseProb * 100).toFixed(1)}%)</span>
				{/if}
			</div>

			<!-- True Segment -->
			<div
				class="flex items-center justify-end bg-emerald-500 px-3 text-white transition-all duration-300 ease-out dark:bg-emerald-600"
				style="width: {trueProb * 100}%;"
			>
				{#if trueProb > 0.12}
					<span>True ({(trueProb * 100).toFixed(1)}%)</span>
				{/if}
			</div>
		</div>

		<!-- Details Table -->
		<div class="grid grid-cols-2 gap-3">
			<div class="border border-neutral-200 bg-white p-3 dark:border-neutral-800 dark:bg-neutral-900/40">
				<span class="font-mono text-[10px] text-neutral-400 uppercase">False (No) Probability</span>
				<div class="mt-1 font-mono text-lg font-bold text-neutral-800 dark:text-neutral-200">
					{(falseProb * 100).toFixed(2)}%
				</div>
			</div>
			<div class="border border-neutral-200 bg-white p-3 dark:border-neutral-800 dark:bg-neutral-900/40">
				<span class="font-mono text-[10px] text-neutral-400 uppercase">True (Yes) Probability</span>
				<div class="mt-1 font-mono text-lg font-bold text-neutral-800 dark:text-neutral-200">
					{(trueProb * 100).toFixed(2)}%
				</div>
			</div>
		</div>
	</div>
</div>
