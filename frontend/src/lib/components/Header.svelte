<script lang="ts">
	import Icon from '@iconify/svelte';
	import ThemeToggle from '#lib/components/ThemeToggle.svelte';
	import { canvasStore } from '#lib/state/canvas.svelte';

	function handleReset() {
		canvasStore.resetCurrentType();
	}
</script>

<header class="border-b border-neutral-200 bg-white/80 backdrop-blur-md dark:border-neutral-800 dark:bg-neutral-950/80">
	<div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
		<!-- Left: Logo & Title -->
		<div class="flex items-center gap-3">
			<a href="/" class="group flex items-center gap-2.5">
				<img src="/logo.svg" alt="Octopus Logo" class="h-8 w-8 object-contain dark:invert" />
				<div class="flex flex-col">
					<div class="flex items-center gap-2">
						<span class="font-mono text-base font-bold tracking-widest text-neutral-900 uppercase dark:text-neutral-100">
							Octopus
						</span>
						<span class="border border-neutral-300 px-1.5 py-0.2 text-[10px] font-medium tracking-wider text-neutral-600 uppercase dark:border-neutral-700 dark:text-neutral-400">
							System 1
						</span>
					</div>
					<span class="text-[11px] text-neutral-500 dark:text-neutral-400">
						Decision Model Playground
					</span>
				</div>
			</a>
		</div>

		<!-- Right: Controls -->
		<div class="flex items-center gap-2 sm:gap-3">
			<!-- Model Selector -->
			<div class="flex items-center gap-1.5 border border-neutral-300 bg-neutral-50 px-2 py-1 dark:border-neutral-700 dark:bg-neutral-900">
				<Icon icon="lucide:cpu" width="14" height="14" class="text-neutral-500" />
				<select
					value={canvasStore.selectedModel}
					onchange={(e) => canvasStore.setSelectedModel(e.currentTarget.value)}
					class="cursor-pointer bg-transparent font-mono text-xs font-medium text-neutral-800 focus:outline-none dark:text-neutral-200"
					aria-label="Select Decision Model"
				>
					{#if canvasStore.availableModels.length > 0}
						{#each canvasStore.availableModels as m}
							<option value={m.id} class="bg-white text-neutral-900 dark:bg-neutral-900 dark:text-neutral-100">
								{m.id}
							</option>
						{/each}
					{:else}
						<option value="clef-flash" class="bg-white text-neutral-900 dark:bg-neutral-900 dark:text-neutral-100">
							clef-flash
						</option>
					{/if}
				</select>
			</div>

			<!-- Reset Button -->
			<button
				type="button"
				onclick={handleReset}
				class="flex h-8 items-center gap-1.5 border border-neutral-300 bg-neutral-100 px-2.5 font-mono text-xs font-medium text-neutral-700 transition-colors hover:border-neutral-900 hover:bg-neutral-200 hover:text-neutral-900 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-300 dark:hover:border-neutral-100 dark:hover:bg-neutral-700 dark:hover:text-neutral-100"
				title="Reset current model inputs to clean slate"
			>
				<Icon icon="lucide:rotate-ccw" width="13" height="13" />
				<span class="hidden sm:inline">Reset</span>
			</button>

			<!-- Theme Toggle -->
			<ThemeToggle />
		</div>
	</div>
</header>
