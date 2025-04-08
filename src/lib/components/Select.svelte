<script lang="ts" generics="T">
	import IconChevron from "$lib/icons/IconChevron.svelte";
	import type { Snippet } from "svelte";

	let {
		value = $bindable(),
		options,
		option
	}: {
		value: T;
		options: Iterable<T>;
		option: Snippet<[T]>
	} = $props();

	let open = $state(false);
</script>

<div
	class:open
	onfocusout={({ relatedTarget, currentTarget }) => {
		if (relatedTarget instanceof HTMLElement && currentTarget.contains(relatedTarget)) {
			return ;
		}
		open = false
	}}
>
	<button
		class="button button-bg"
		onclick={() => open = !open}
	>
		{@render option(value)}
		<IconChevron />
	</button>

	<ul
		class="card bordered"
	>
		{#each options as current}
			<li>
				<button
					class="button"
					tabindex={open ? 0 : -1}
					class:selected={value === current}
					onclick={() => {
						value = current;
						open = false;
					}}
				>
					{@render option(current)}
				</button>
			</li>
		{/each}
	</ul>
</div>

<style>
	div, button {
		position: relative;
		width: 100%;
	}

	button :global(img) {
		flex: 0 0 auto;
	}

	button :global(span) {
		flex: 1 1 auto;
		text-overflow: ellipsis;
		overflow: hidden;
		text-align: left;
	}

	div :global(svg) {
		transition: transform var(--snappy);
	}

	div.open :global(svg) {
		transform: rotate(180deg);
	}

	ul {
		transform: scaleY(0);
		transition: transform var(--snappy);
		position: absolute;
		width: 100%;
		transform-origin: top;
		z-index: 99;
		background: var(--layer-0);
		border-radius: 1.5rem;

		.button {
			min-height: auto;
			transition: background var(--snappy), color var(--snappy);

			&:hover {
				background: var(--layer-1);
				color: var(--text);
			}

			&.selected {
				background: rgb(var(--orange));
				color: var(--layer-0);
			}
		}
	}

	div.open ul {
		transform: scaleY(1);
	}
</style>
