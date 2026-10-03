<script lang="ts">
	import Icon from '@iconify/svelte';
	import { canvasStore } from '#lib/state/canvas.svelte';
</script>

<div class="flex flex-col gap-3">
	<label class="flex items-center gap-1.5 font-mono text-xs font-semibold tracking-wider text-neutral-800 uppercase dark:text-neutral-200">
		<Icon icon="lucide:check-circle-2" width="14" height="14" />
		<span>Boundary Criteria (Optional)</span>
	</label>

	<div class="grid grid-cols-1 gap-2.5 sm:grid-cols-2">
		<div class="border border-neutral-200 bg-white p-3 dark:border-neutral-800 dark:bg-neutral-900/60">
			<div class="mb-1.5 flex items-center justify-between">
				<span class="font-mono text-[11px] font-bold text-emerald-600 uppercase dark:text-emerald-400">
					True Condition (Yes)
				</span>
				<span class="font-mono text-[10px] text-neutral-400">P → 1.0</span>
			</div>
			<input
				type="text"
				placeholder="When should the model answer Yes/True?"
				value={canvasStore.noulState.trueDesc}
				oninput={(e) => {
					canvasStore.noulState.trueDesc = e.currentTarget.value;
					canvasStore.saveToStorage();
					canvasStore.scheduleEvaluate();
				}}
				class="w-full border border-neutral-300 bg-transparent px-2.5 py-1.5 text-xs text-neutral-900 placeholder:text-neutral-400 focus:border-neutral-900 focus:outline-none dark:border-neutral-700 dark:text-neutral-100 dark:placeholder:text-neutral-500 dark:focus:border-neutral-100"
			/>
		</div>

		<div class="border border-neutral-200 bg-white p-3 dark:border-neutral-800 dark:bg-neutral-900/60">
			<div class="mb-1.5 flex items-center justify-between">
				<span class="font-mono text-[11px] font-bold text-rose-600 uppercase dark:text-rose-400">
					False Condition (No)
				</span>
				<span class="font-mono text-[10px] text-neutral-400">P → 0.0</span>
			</div>
			<input
				type="text"
				placeholder="When should the model answer No/False?"
				value={canvasStore.noulState.falseDesc}
				oninput={(e) => {
					canvasStore.noulState.falseDesc = e.currentTarget.value;
					canvasStore.saveToStorage();
					canvasStore.scheduleEvaluate();
				}}
				class="w-full border border-neutral-300 bg-transparent px-2.5 py-1.5 text-xs text-neutral-900 placeholder:text-neutral-400 focus:border-neutral-900 focus:outline-none dark:border-neutral-700 dark:text-neutral-100 dark:placeholder:text-neutral-500 dark:focus:border-neutral-100"
			/>
		</div>
	</div>
</div>
