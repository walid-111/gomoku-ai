<script lang="ts">
	import Label from "$lib/components/Label.svelte";
	import type { Snippet } from "svelte";
	import type { HTMLInputAttributes } from 'svelte/elements';

	let {
		children,
		id,
		checked = $bindable(false),
		...restProps
	}: {
		children: Snippet,
		id: string,
	} & Omit<HTMLInputAttributes, 'type'> = $props();
</script>

<input type="checkbox" bind:checked {id} {...restProps} />

<Label for={id}>
	{@render children()}

	<div class="slider" class:checked></div>
</Label>


<style>
	input {
		opacity: 0;
		pointer-events: none;
		position: absolute;
	}

	.slider {
		background: var(--layer-2);
		width: 3rem;
		height: 2rem;
		border-radius: 1rem;
		position: relative;
		transition: background var(--snappy);
	}

	.slider::before {
		content: "";
		background: var(--text);
		width:  1.5rem;
		height: 1.5rem;
		border-radius: 50%;
		position: absolute;
		top: 0.25rem;
		left: 0.25rem;
		transition: background var(--snappy), transform var(--snappy);
	}

	.slider.checked {
		background: rgb(var(--orange));
	}
	.slider.checked::before {
		background: var(--layer-0);
		transform: translateX(1rem);
	}
</style>
