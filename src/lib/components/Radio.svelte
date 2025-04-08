<script lang="ts">
	import { getContext, type Snippet } from "svelte";
	import type { HTMLInputAttributes } from "svelte/elements";

	let { children, ...restProps }: {
		children: Snippet,
	} & Omit<HTMLInputAttributes, 'type'> = $props();

	let context: { value: string } = getContext('radio-group');
</script>

<label>
	<input bind:group={context.value} type="radio" {...restProps} />
	{@render children()}
</label>
