<script lang="ts">
	import Icon from '@iconify/svelte';
	import { canvasStore } from '#lib/state/canvas.svelte';
	import CornerDecorations from '#lib/components/CornerDecorations.svelte';
	import ChoiceChart from '#lib/components/ChoiceChart.svelte';
	import ScoreChart from '#lib/components/ScoreChart.svelte';
	import NoulMeter from '#lib/components/NoulMeter.svelte';

	const currentResult = $derived.by(() => {
		if (canvasStore.activeType === 'choice') return canvasStore.choiceResult;
		if (canvasStore.activeType === 'score') return canvasStore.scoreResult;
		return canvasStore.noulResult;
	});

	function formatDuration(ms: number): string {
		if (ms < 1000) {
			return `${ms.toFixed(1)} ms`;
		}
		return `${(ms / 1000).toFixed(2)} s`;
	}
</script>

<CornerDecorations class="bg-neutral-50/40 p-4 sm:p-5 dark:bg-neutral-900/30">
	<div class="flex flex-col gap-5">
		<!-- Output Header -->
		<div class="flex items-center justify-between border-b border-neutral-200 pb-3 dark:border-neutral-800">
			<div class="flex items-center gap-2">
				<Icon icon="lucide:sparkles" width="16" height="16" class="text-neutral-500" />
				<span class="font-mono text-xs font-bold uppercase tracking-wider text-neutral-900 dark:text-neutral-100">
					Interactive Decision Output
				</span>
			</div>

			<div class="flex items-center gap-2">
				{#if canvasStore.isEvaluating}
					<div class="flex items-center gap-1.5 border border-amber-300 bg-amber-50 px-2 py-0.5 font-mono text-[11px] text-amber-800 dark:border-amber-800 dark:bg-amber-950/60 dark:text-amber-300">
						<Icon icon="lucide:loader-2" width="12" height="12" class="animate-spin" />
						<span>Inferring...</span>
					</div>
				{:else if currentResult}
					<!-- AI Duration Badge -->
					<div class="flex items-center gap-1 border border-neutral-300 bg-white px-2 py-0.5 font-mono text-[11px] text-neutral-600 dark:border-neutral-700 dark:bg-neutral-900 dark:text-neutral-300">
						<Icon icon="lucide:zap" width="12" height="12" class="text-amber-500" />
						<span>AI: {formatDuration(currentResult.ai_duration_ms)}</span>
					</div>
				{:else}
					<span class="font-mono text-[10px] text-neutral-400">Waiting for input</span>
				{/if}
			</div>
		</div>

		<!-- Error Message Banner -->
		{#if canvasStore.error}
			<div class="flex items-start gap-2 border border-red-300 bg-red-50 p-3 text-xs text-red-800 dark:border-red-900 dark:bg-red-950/50 dark:text-red-300">
				<Icon icon="lucide:alert-triangle" width="16" height="16" class="mt-0.5 shrink-0 text-red-600" />
				<div class="flex flex-col gap-1">
					<span class="font-mono font-bold uppercase">Inference Notice</span>
					<span>{canvasStore.error}</span>
				</div>
			</div>
		{/if}

		<!-- Result View -->
		{#if currentResult}
			{#if currentResult.result.type === 'choice'}
				<ChoiceChart result={currentResult.result} />
			{:else if currentResult.result.type === 'score'}
				<ScoreChart result={currentResult.result} />
			{:else if currentResult.result.type === 'noul'}
				<NoulMeter result={currentResult.result} />
			{/if}
		{:else}
			<!-- Empty State -->
			<div class="flex flex-col items-center justify-center py-16 text-center">
				<div class="flex h-14 w-14 items-center justify-center border border-dashed border-neutral-300 bg-neutral-100 text-neutral-400 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-500">
					<Icon icon="lucide:brain" width="28" height="28" />
				</div>
				<h3 class="mt-4 font-mono text-sm font-bold uppercase tracking-wider text-neutral-800 dark:text-neutral-200">
					Awaiting Decision Schema
				</h3>
				<p class="mt-1.5 max-w-sm text-xs leading-relaxed text-neutral-500 dark:text-neutral-400">
					{#if canvasStore.activeType === 'choice'}
						Enter an instruction, at least 2 options, and a prompt to judge. Output updates automatically in real-time.
					{:else if canvasStore.activeType === 'score'}
						Enter an instruction, at least 2 score levels, and a prompt to calculate weighted continuous score.
					{:else}
						Enter an instruction condition and prompt to see calibrated binary likelihood.
					{/if}
				</p>

				<div class="mt-6 flex flex-wrap items-center justify-center gap-2 font-mono text-[11px] text-neutral-400">
					<span class="border border-neutral-200 px-2 py-0.5 dark:border-neutral-800">1s Debounce</span>
					<span class="border border-neutral-200 px-2 py-0.5 dark:border-neutral-800">Calibrated Distribution</span>
					<span class="border border-neutral-200 px-2 py-0.5 dark:border-neutral-800">Non-Autoregressive</span>
				</div>
			</div>
		{/if}
	</div>
</CornerDecorations>
