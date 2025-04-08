<script lang="ts">
	import { createSelectedTabState } from "$lib/components/Tabs.svelte";
	import NavBar, { PANELS, type Panel } from "$lib/NavBar.svelte";
	import * as wasm from "$lib/ai/game-rs/game_rs";
	import { fly } from "svelte/transition";
	import { getGameState } from "../routes/+page.svelte";

	const gameState = getGameState();

	let offsetWidth: number = 0;

	const slideDirection = $derived((selectedPanel.previous.index < selectedPanel.current.index) ? 1 : -1);
</script>

<script module lang="ts">
	function storeSelectedPanel() {
		localStorage.setItem('panel', selectedPanel.current.index.toString());
	}
	function loadSelectedPanel(): Panel {
		let index = localStorage.getItem('panel');
		return PANELS[index === null ? 0 : parseInt(index)];
	}

	export const selectedPanel = createSelectedTabState(
		loadSelectedPanel(),
		storeSelectedPanel
	);
</script>

<aside class="grow">
	<NavBar {selectedPanel} />

	<div class="panel-container">
		{#key selectedPanel.current}
			<div class="panel-scrollarea">
				<div
					class="panel-content"
					bind:offsetWidth
					in:fly= {{ x: slideDirection *  offsetWidth, duration: 200, opacity: 1 }}
					out:fly={{ x: slideDirection * -offsetWidth, duration: 200, opacity: 1 }}
				>
					<div class="container">
						<selectedPanel.current.panel></selectedPanel.current.panel>
					</div>
				</div>
			</div>
		{/key}
	</div>

	{#if gameState.winner !== wasm.Player.None}
		<div class="button-actions">
			<button class="button button-bg" onclick={() => {
				gameState.selectionScreen = true;
			}}>New Game</button>
			<button class="button button-bg-primary" onclick={() => {
				gameState.reset();
			}}>Rematch</button>
		</div>
	{/if}
</aside>

<style>
	aside {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		max-width: 40ch;
	}

	.panel-container {
		flex: 1 0 0;
		position: relative;
	}

	.panel-scrollarea {
		overflow-x: hidden;
		overflow-y: auto;
		position: absolute;
		inset: 0 -1rem;
		padding: 0 1rem;
	}

	.panel-content {
		position: relative;

		.container {
			position: absolute;
			width: 100%;
			padding-bottom: 1rem;
		}
	}
</style>
