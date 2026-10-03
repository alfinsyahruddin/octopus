<script lang="ts">
	import Icon from '@iconify/svelte';
	import { canvasStore } from '#lib/state/canvas.svelte';

	let isDragging = $state<boolean>(false);
	let fileInputRef: HTMLInputElement | null = null;

	const currentImages = $derived.by(() => {
		if (canvasStore.activeType === 'choice') return canvasStore.choiceState.images;
		if (canvasStore.activeType === 'score') return canvasStore.scoreState.images;
		return canvasStore.noulState.images;
	});

	function handleFiles(files: FileList | null) {
		if (!files || files.length === 0) return;

		for (let i = 0; i < files.length; i++) {
			const file = files[i];
			if (!file.type.startsWith('image/')) continue;

			const reader = new FileReader();
			reader.onload = (e) => {
				const result = e.target?.result as string;
				if (result) {
					canvasStore.addImage(result);
				}
			};
			reader.readAsDataURL(file);
		}
	}

	function onDrop(e: DragEvent) {
		e.preventDefault();
		isDragging = false;
		if (e.dataTransfer?.files) {
			handleFiles(e.dataTransfer.files);
		}
	}

	function onDragOver(e: DragEvent) {
		e.preventDefault();
		isDragging = true;
	}

	function onDragLeave(e: DragEvent) {
		e.preventDefault();
		isDragging = false;
	}

	function formatBase64Size(b64: string): string {
		const stringLength = b64.length - (b64.indexOf(',') + 1);
		const sizeInBytes = 4 * Math.ceil(stringLength / 3) * 0.5624896334383687;
		const kb = sizeInBytes / 1024;
		return kb >= 1024 ? `${(kb / 1024).toFixed(1)} MB` : `${Math.round(kb)} KB`;
	}
</script>

<div class="flex flex-col gap-2">
	<div class="flex items-center justify-between">
		<label class="flex items-center gap-1.5 font-mono text-xs font-semibold tracking-wider text-neutral-800 uppercase dark:text-neutral-200">
			<Icon icon="lucide:image" width="14" height="14" />
			<span>Multimodal Vision Input (Optional)</span>
		</label>
		{#if currentImages.length > 0}
			<span class="font-mono text-[11px] text-neutral-500">
				{currentImages.length} image{currentImages.length > 1 ? 's' : ''} attached
			</span>
		{/if}
	</div>

	<!-- Drop zone -->
	<div
		role="region"
		aria-label="Image dropzone"
		ondrop={onDrop}
		ondragover={onDragOver}
		ondragleave={onDragLeave}
		class="relative flex flex-col items-center justify-center border border-dashed p-4 text-center transition-colors {isDragging
			? 'border-neutral-900 bg-neutral-100 dark:border-neutral-100 dark:bg-neutral-800'
			: 'border-neutral-300 bg-white hover:border-neutral-500 dark:border-neutral-700 dark:bg-neutral-900/60 dark:hover:border-neutral-500'}"
	>
		<input
			type="file"
			accept="image/png,image/jpeg,image/webp"
			multiple
			bind:this={fileInputRef}
			onchange={(e) => handleFiles(e.currentTarget.files)}
			class="hidden"
		/>

		<Icon icon="lucide:upload-cloud" width="24" height="24" class="mb-1 text-neutral-400 dark:text-neutral-500" />
		<p class="font-mono text-xs text-neutral-700 dark:text-neutral-300">
			Drag & drop images here, or
			<button
				type="button"
				onclick={() => fileInputRef?.click()}
				class="underline decoration-1 underline-offset-2 hover:text-neutral-900 dark:hover:text-neutral-100"
			>
				browse files
			</button>
		</p>
		<span class="mt-0.5 text-[10px] text-neutral-400">PNG, JPEG, WebP</span>
	</div>

	<!-- Image Thumbnails Grid -->
	{#if currentImages.length > 0}
		<div class="mt-1 flex flex-wrap gap-2.5">
			{#each currentImages as img, idx (idx)}
				<div class="group relative flex h-20 w-20 flex-col overflow-hidden border border-neutral-300 bg-neutral-100 dark:border-neutral-700 dark:bg-neutral-800">
					<img src={img} alt="Uploaded input {idx + 1}" class="h-full w-full object-cover" />

					<!-- Size indicator badge -->
					<div class="absolute bottom-0 inset-x-0 bg-black/60 px-1 py-0.5 text-center font-mono text-[9px] text-white">
						{formatBase64Size(img)}
					</div>

					<!-- Delete button -->
					<button
						type="button"
						onclick={() => canvasStore.removeImage(idx)}
						class="absolute top-1 right-1 flex h-5 w-5 items-center justify-center bg-black/75 text-white opacity-0 transition-opacity group-hover:opacity-100 hover:bg-red-600"
						title="Remove image"
					>
						<Icon icon="lucide:x" width="12" height="12" />
					</button>
				</div>
			{/each}
		</div>
	{/if}
</div>
