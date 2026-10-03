<script lang="ts">
	import Icon from '@iconify/svelte';
	import { canvasStore } from '#lib/state/canvas.svelte';
	import ChoiceConfig from '#lib/components/ChoiceConfig.svelte';
	import ScoreConfig from '#lib/components/ScoreConfig.svelte';
	import NoulConfig from '#lib/components/NoulConfig.svelte';
	import ImageUploader from '#lib/components/ImageUploader.svelte';
	import CornerDecorations from '#lib/components/CornerDecorations.svelte';

	let instructionValue = $derived.by(() => {
		if (canvasStore.activeType === 'choice') return canvasStore.choiceState.instruction;
		if (canvasStore.activeType === 'score') return canvasStore.scoreState.instruction;
		return canvasStore.noulState.instruction;
	});

	let promptValue = $derived.by(() => {
		if (canvasStore.activeType === 'choice') return canvasStore.choiceState.prompt;
		if (canvasStore.activeType === 'score') return canvasStore.scoreState.prompt;
		return canvasStore.noulState.prompt;
	});

	const statusConfig = $derived.by(() => {
		switch (canvasStore.status) {
			case 'loading':
				return {
					text: 'Loading...',
					dotClass: 'bg-yellow-400 animate-pulse',
					textClass: 'text-yellow-600 dark:text-yellow-400 font-medium'
				};
			case 'done':
				return {
					text: 'Done',
					dotClass: 'bg-emerald-500',
					textClass: 'text-emerald-600 dark:text-emerald-400 font-medium'
				};
			case 'idle':
			default:
				return {
					text: 'IDLE',
					dotClass: 'bg-neutral-400 dark:bg-neutral-500',
					textClass: 'text-neutral-400 dark:text-neutral-500'
				};
		}
	});

	function updateInstruction(text: string) {
		if (canvasStore.activeType === 'choice') canvasStore.choiceState.instruction = text;
		else if (canvasStore.activeType === 'score') canvasStore.scoreState.instruction = text;
		else canvasStore.noulState.instruction = text;

		canvasStore.saveToStorage();
		canvasStore.scheduleEvaluate();
	}

	function updatePrompt(text: string) {
		if (canvasStore.activeType === 'choice') canvasStore.choiceState.prompt = text;
		else if (canvasStore.activeType === 'score') canvasStore.scoreState.prompt = text;
		else canvasStore.noulState.prompt = text;

		canvasStore.saveToStorage();
		canvasStore.scheduleEvaluate();
	}
</script>

<CornerDecorations class="bg-neutral-50/40 p-4 sm:p-5 dark:bg-neutral-900/30">
	<div class="flex flex-col gap-5">
		<!-- Section Header -->
		<div class="flex items-center justify-between border-b border-neutral-200 pb-3 dark:border-neutral-800">
			<div class="flex items-center gap-2">
				<Icon icon="lucide:terminal" width="16" height="16" class="text-neutral-500" />
				<span class="font-mono text-xs font-bold uppercase tracking-wider text-neutral-900 dark:text-neutral-100">
					Decision Input Schema
				</span>
			</div>
			<div class="flex items-center gap-1.5 font-mono text-[10px]">
				<span class="inline-block h-1.5 w-1.5 rounded-full {statusConfig.dotClass}"></span>
				<span class={statusConfig.textClass}>{statusConfig.text}</span>
			</div>
		</div>

		<!-- 1. Instruction Input -->
		<div class="flex flex-col gap-1.5">
			<div class="flex items-center justify-between">
				<label class="flex items-center gap-1.5 font-mono text-xs font-semibold tracking-wider text-neutral-800 uppercase dark:text-neutral-200">
					<Icon icon="lucide:help-circle" width="14" height="14" />
					<span>1. Instruction / Question</span>
				</label>
				<span class="font-mono text-[10px] text-neutral-400">
					{instructionValue.length} chars
				</span>
			</div>
			<textarea
				rows="2"
				placeholder="What decision or judgment should the model make? (e.g. Which team should handle this?)"
				value={instructionValue}
				oninput={(e) => updateInstruction(e.currentTarget.value)}
				class="w-full resize-y border border-neutral-300 bg-white p-3 font-mono text-xs text-neutral-900 placeholder:text-neutral-400 focus:border-neutral-900 focus:outline-none dark:border-neutral-700 dark:bg-neutral-900/80 dark:text-neutral-100 dark:placeholder:text-neutral-500 dark:focus:border-neutral-100"
			></textarea>
		</div>

		<!-- 2. Dynamic Primitive Config (Choice / Score / Noul) -->
		<div class="border-t border-neutral-200/80 pt-4 dark:border-neutral-800/80">
			<div class="mb-3">
				<span class="font-mono text-[11px] font-bold text-neutral-500 uppercase tracking-wider">
					2. Output Structure Definition
				</span>
			</div>
			{#if canvasStore.activeType === 'choice'}
				<ChoiceConfig />
			{:else if canvasStore.activeType === 'score'}
				<ScoreConfig />
			{:else if canvasStore.activeType === 'noul'}
				<NoulConfig />
			{/if}
		</div>

		<!-- 3. Prompt / State Textarea -->
		<div class="border-t border-neutral-200/80 pt-4 dark:border-neutral-800/80">
			<div class="flex flex-col gap-1.5">
				<div class="flex items-center justify-between">
					<label class="flex items-center gap-1.5 font-mono text-xs font-semibold tracking-wider text-neutral-800 uppercase dark:text-neutral-200">
						<Icon icon="lucide:file-text" width="14" height="14" />
						<span>3. State / Context To Judge</span>
					</label>
					<span class="font-mono text-[10px] text-neutral-400">
						{promptValue.length} chars
					</span>
				</div>
				<textarea
					rows="4"
					placeholder="Enter the passage, customer message, issue description, or context to evaluate..."
					value={promptValue}
					oninput={(e) => updatePrompt(e.currentTarget.value)}
					class="w-full resize-y border border-neutral-300 bg-white p-3 text-xs leading-relaxed text-neutral-900 placeholder:text-neutral-400 focus:border-neutral-900 focus:outline-none dark:border-neutral-700 dark:bg-neutral-900/80 dark:text-neutral-100 dark:placeholder:text-neutral-500 dark:focus:border-neutral-100"
				></textarea>
			</div>
		</div>

		<!-- 4. Images Upload -->
		<div class="border-t border-neutral-200/80 pt-4 dark:border-neutral-800/80">
			<ImageUploader />
		</div>
	</div>
</CornerDecorations>
