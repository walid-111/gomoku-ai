<script module lang="ts">
	import Tabs, { type SelectedTab, type Tab } from '$lib/components/Tabs.svelte';

	import IconDebug from '$lib/icons/IconDebug.svelte';
	import IconRules from '$lib/icons/IconRules.svelte';
	import IconSettings from '$lib/icons/IconSettings.svelte';

	import Debug from '$lib/panels/Debug.svelte';
	import Rules from '$lib/panels/Rules.svelte';
	import Settings from '$lib/panels/Settings.svelte';

	export type Panel = Tab & {
		panel: any,
		index: number
	};

	export let PANELS: Panel[] = $state([
		{
			icon: IconRules,
			panel: Rules,
			title: 'Rules',
			index: 0
		},
		{
			icon: IconDebug,
			panel: Debug,
			title: 'Debug',
			index: 1
		},
		{
			icon: IconSettings,
			panel: Settings,
			title: 'Settings',
			index: 2
		}
	]);

	export const DEBUG_PANEL_INDEX = 1;
</script>

<script lang="ts">
	import IconGitHub from '$lib/icons/IconGitHub.svelte';
	import IconUndo from '$lib/icons/IconUndo.svelte';
	import { getGameState } from '../routes/+page.svelte';

	const gameState = getGameState();

	let { selectedPanel }: {
		selectedPanel: SelectedTab<Panel>
	} = $props();
</script>

<nav>
	<aside class="grow">
		<!-- <button
			title="Undo Move" class="button button-bg"
			disabled={gameState.playedMoves.length === 0}
			onclick={() => {
				gameState.undoMove(
					gameState.playerTypeAtTurn(gameState.playedMoves.length+1).pickMove
						? 2 // The opp is an AI (we undo 2 moves)
						: 1 // The opp is a human
				);
			}}
		><IconUndo /></button> -->
	</aside>

	<Tabs
		tabs={PANELS}
		selectedTab={selectedPanel}
	/>

	<aside class="grow">
		<!-- <a class="button" title="Source Code" href="https://github.com/walidmoovin/Gomoku" target="_blank"><IconGitHub /></a> -->
	</aside>
</nav>

<style>
	nav {
		display: flex;
		gap: 0.5rem;
		justify-content: center;

		aside {
			&:last-child {
				display: flex;
				justify-content: flex-end;
			}
		}
	}
</style>
