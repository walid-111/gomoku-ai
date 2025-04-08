<script module lang="ts">
	export type Tab = {
		icon?: any,
		text?: string,
		title: string,
	};

	export type SelectedTab<T extends Tab> = {
		readonly current: T,
		readonly previous: T,
		changeTab: (tab: T) => void
	}

	export function createSelectedTabState<T extends Tab>(defaultTab: T, onUpdate: ((newTab: T) => void) | undefined = undefined): SelectedTab<T> {
		let current = $state(defaultTab);
		let previous = $state(defaultTab);

		return {
			get current()  { return current },
			get previous() { return previous },
			changeTab(tab: T) {
				previous = current;
				current = tab;
				onUpdate?.(tab);
			}
		};
	}
</script>

<script lang="ts" generics="T extends Tab">
	import { crossfade } from 'svelte/transition';

	let { tabs, selectedTab = $bindable() }: {
		tabs: T[],
		selectedTab: SelectedTab<T>
	} = $props();

	const TABS_CROSSFADE_SYMBOL = Symbol();
	const [send, receive] = crossfade({
		duration: 200,
	});
</script>

{#each tabs as tab}
	<button
		class="button"
		onclick={() => selectedTab.changeTab(tab)}
		title={tab.title}
	>
		{#if tab.icon}
			<tab.icon></tab.icon>
		{/if}

		{tab.text}

		{#if selectedTab.current === tab}
			<div
				in:send={{ key: TABS_CROSSFADE_SYMBOL }} out:receive={{ key: TABS_CROSSFADE_SYMBOL }}
				class="button button-bg selected-tab"
			></div>
		{/if}
	</button>
{/each}

<style>
	.selected-tab {
		position: absolute;
		z-index: -1;
		inset: 0;
	}
</style>
