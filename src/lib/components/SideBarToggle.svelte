<script lang="ts">
	import IconChevron from "$lib/icons/IconChevron.svelte";

	let { open = $bindable(true) }: {
		open?: boolean
	} = $props();
</script>

<div>
	<button
		class="button button-bg"
		onclick={() => open = !open}
		class:open
	>
		<IconChevron />
	</button>
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
		justify-content: center;
	}

	button {
		padding-left: 0;
		padding-right: 0;
		width: min-content;
	}

	button :global(svg) {
		transition: transform var(--snappy);
		transform: rotate(90deg);
	}
	button.open :global(svg) {
		transform: rotate(-90deg);
	}
</style>
