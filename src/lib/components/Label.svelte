<script lang="ts">
	import type { Snippet } from "svelte";
	import type { HTMLLabelAttributes } from 'svelte/elements';

	let { children, ...restProps }: {
		children: Snippet
	} & HTMLLabelAttributes = $props();
</script>

<label {...restProps}>
	{@render children()}
</label>

<style>
	label {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		border-radius: 1rem;
		background: var(--layer-1);
		padding: 1rem;
	}
</style>
