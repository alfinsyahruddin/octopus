<script lang="ts">
	import ModelTypeSelector from '#lib/components/ModelTypeSelector.svelte';
	import PromptEditor from '#lib/components/PromptEditor.svelte';
	import OutputVisualizer from '#lib/components/OutputVisualizer.svelte';
</script>

<svelte:head>
	<title>Octopus — System One Decision Model Playground</title>
	<meta
		name="description"
		content="Interactive playground for System One Decision Models: Choice, Score, and Noul with real-time calibrated predictions."
	/>
</svelte:head>

<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
	<!-- Top Bar / Primitive Selector -->
	<div class="mb-6 flex flex-col gap-4">
		<div class="flex flex-col gap-1">
			<h1 class="font-mono text-xl font-bold tracking-tight text-neutral-900 uppercase sm:text-2xl dark:text-neutral-100">
				System One Decision Playground
			</h1>
			<p class="max-w-2xl text-xs text-neutral-500 sm:text-sm dark:text-neutral-400">
				Explore typed, calibrated decisions executed in a single forward pass without token-by-token text generation.
			</p>
		</div>

		<ModelTypeSelector />
	</div>

	<!-- Main Canvas Grid: Input Schema vs Interactive Output -->
	<div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
		<!-- Left: Input Schema & Prompt -->
		<div class="flex flex-col">
			<PromptEditor />
		</div>

		<!-- Right: Visualizer & Metrics -->
		<div class="flex flex-col">
			<OutputVisualizer />
		</div>
	</div>
</div>
