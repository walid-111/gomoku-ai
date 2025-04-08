<script lang="ts">
	import { setContext, type Snippet } from 'svelte';

	let { value = $bindable(''), children }: {
		value: string,
		children: Snippet,
	} = $props();

	setContext('radio-group', {
		set value(v: string) {
			value = v;
		},
		get value() {
			return value;
		},
	});
</script>

<div>
	{@render children()}
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
	}
</style>
