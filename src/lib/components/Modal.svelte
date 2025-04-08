<script lang="ts">
	import type { Snippet } from "svelte";

	let { open = $bindable(), transient = true, children }: {
		open: boolean,
		transient?: boolean,
		children: Snippet<[{ close: () => void }]>
	} = $props();
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div role="dialog" class="dialog" class:open onclick={(e) => {
	if (transient && e.target === e.currentTarget) {
		open = false;
	}
}}>
	<div class="container card bordered">
		{#if open}
			{@render children({
				close: () => open = false
			})}
		{/if}
	</div>
</div>

<style>
	.dialog {
		position: fixed;
		inset: 0;
		backdrop-filter: blur(4.2px);
		background: #0008;
		display: grid;
		place-items: center;
		z-index: 9999;
		pointer-events: none;
		opacity: 0;
		transition: opacity var(--snappy);

		&.open {
			pointer-events: auto;
			opacity: 1;
		}

		.card {
			width: 100%;
			max-width: 30rem;
			background: var(--layer-0);
			border-radius: 2rem;
			padding: 1rem;
			box-shadow: 0 0 2rem 0 #000;

			transform: scale(0.5);
			transition: transform var(--snappy);
		}

		&.open .card {
			transform: scale(1);
		}
	}
</style>
