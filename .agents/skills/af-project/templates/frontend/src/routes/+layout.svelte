<script lang="ts">
	import '../app.css';
	import type { Snippet } from 'svelte';
	import ToastViewport from '#lib/components/ToastViewport.svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<div class="min-h-screen bg-(--bg) text-(--fg) font-sans antialiased">
	{@render children()}
	<ToastViewport />
</div>
