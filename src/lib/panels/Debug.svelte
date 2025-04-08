<script lang="ts">
	import TimingLogs from "$lib/components/TimingLogs.svelte";

	import { debuggingSession } from '$lib/ai/debug.svelte.ts';
	import Chart from "$lib/components/Chart.svelte";
	import MovesHistory from "$lib/components/MovesHistory.svelte";
	import Radio from "$lib/components/Radio.svelte";
	import RadioGroup from "$lib/components/RadioGroup.svelte";
	import IconCopied from "$lib/icons/IconCopied.svelte";
	import IconCopy from "$lib/icons/IconCopy.svelte";
	import { BOARD_SIZE, getGameState } from "../../routes/+page.svelte";
	import Switch from "$lib/components/Switch.svelte";
	import IconEdit from "$lib/icons/IconEdit.svelte";
	import IconSave from "$lib/icons/IconSave.svelte";
	import { tick } from "svelte";
	import { MoveResult } from "$lib/ai/game-rs/game_rs";

	const gameState = getGameState();

	let logsHTMLElement: HTMLPreElement;

	$effect(() => {
		if (!logsHTMLElement) return;
		debuggingSession.logs;
		logsHTMLElement.scrollTo({ top: logsHTMLElement.scrollHeight, behavior: 'smooth' })
	});

	let copied = $state(false);

	let editMoves = $state(false);
	let editMovesElement: HTMLInputElement;
	let editMovesValue = '';
	function editMovesSave() {
		editMoves = false;
		try {
			const moves = JSON.parse(editMovesValue).map((pos: any) => {
				if (typeof pos !== 'number') {
					throw new Error('Invalid move');
				}
				return { pos };
			});
			gameState.reset();
			gameState.playedMoves = moves;
			if (gameState.updateDisplayedMoves().result === MoveResult.Invalid) {
				throw new Error('Invalid moves');
			}
		} catch (e) {
			alert('Invalid moves');
		}
	}
	$effect(() => {
		// close edit moves when moves are updated
		gameState.playedMoves.length;
		editMoves = false;
	})



	const getCells = () => [...document.querySelectorAll('#game rect.cell')];

	const clickCell = async (cell: Element) => {
		cell.dispatchEvent(new MouseEvent('click', {
			bubbles: true,
			cancelable: true,
			view: window,
		}));
		await new Promise(requestAnimationFrame);
	};
	const clickAt = async (cells: Element[], x: number, y: number) => {
		await clickCell(cells[+y * 19 + +x]);
	};
</script>

<script module lang="ts">
	import { storage } from "$lib/storage.svelte";

	export let collectDebugInfo = storage('settings.enable_debug', false);
</script>

<section>
	<h2>Timing</h2>
	<TimingLogs debuggingSession={debuggingSession} />
</section>

<section>
	<Switch bind:checked={collectDebugInfo.value} id="settings.enable_debug">
		<span>Collect Debug Info</span>
	</Switch>
</section>

<div class="button-actions">
	<button class="button button-bg" onclick={async () => {
		const PATTERN = [0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0];
		const cells = getCells();

		for (let y in new Array(18/2).fill(null)) {
			for (let x in PATTERN) {
				if (PATTERN[x]) {
					await clickAt(cells, +x, +y*2 +1);
					await clickAt(cells, +x, +y*2);
				} else {
					await clickAt(cells, +x, +y*2);
					await clickAt(cells, +x, +y*2 +1);
				}
			}
		}

		for (let x of [15,14,13,9,10,12,8,4,5,6,3,2,7,16,18,17,11,0,1]) {
			await clickAt(cells, x, 18);
		}
	}}>Full Fill</button>

	<button class="button button-bg" onclick={async () => {
		for (let i = 0; i < BOARD_SIZE * BOARD_SIZE * 2; ++i) {
			let empty = getCells().filter(cell => !cell.classList.contains('not-allowed'));
			if (empty.length === 0) return;
			await clickCell(empty[Math.floor(Math.random() * empty.length)]);
		}
	}}>Random Fill</button>
</div>

<section>
	<h2 class="action-title">
		<span class="grow">History</span>
		{#if editMoves}
			<button onclick={editMovesSave}>
				<IconSave />
			</button>
		{:else}
			<button onclick={async () => {
				editMoves = true;
				editMovesValue = JSON.stringify(gameState.playedMoves.map(move => move.pos));
				await tick();
				editMovesElement.focus();
				editMovesElement.select();
			}}>
				<IconEdit />
			</button>
		{/if}
		<button
			onclick={async () => {
				try {
					await navigator.clipboard.writeText(JSON.stringify(
						gameState.playedMoves.map(move => move.pos)
					));
					copied = true;
					setTimeout(() => copied = false, 1000);
				} catch (error) {
					alert('Copy Failed');
				}
			}}
		>
			{#if copied}
				<IconCopied />
			{:else}
				<IconCopy />
			{/if}
		</button>
	</h2>
	{#if editMoves}
		<form onsubmit={editMovesSave} style="display:contents">
			<input
				type="text"
				bind:value={editMovesValue}
				bind:this={editMovesElement}
			>
		</form>
	{:else}
		<MovesHistory
			rootMoveIndex={-1}
			moves={gameState.playedMoves}
			onclick={(index) => {
				gameState.playedMoves = gameState.playedMoves.slice(0, index);
				gameState.debugMoves = [];
				gameState.updateDisplayedMoves();
			}}
		/>
	{/if}
</section>

{#if 0 < debuggingSession.overlays.size}
	<section>
		<h2>Overlay</h2>
		<RadioGroup bind:value={debuggingSession.selectedOverlayName.value}>
			{#each debuggingSession.overlays.keys() as key}
				<Radio value={key}>{key}</Radio>
			{/each}
		</RadioGroup>
	</section>

	<section>
		<h2>Debug Moves</h2>
		<MovesHistory
			rootMoveIndex={gameState.playedMoves.length - 2}
			moves={gameState.debugMoves}
			onclick={(index) => {
				gameState.debugMoves = gameState.debugMoves.slice(0, index);
				gameState.updateDisplayedMoves();
			}}
		/>
	</section>
{/if}


<section>
	<h2>Predicated Score</h2>
	<Chart values={
		gameState.playedMoves
			.map(move => move.score)
			.filter(x => x !== undefined)
	} />

	<h2>Alignments Score</h2>
	<Chart values={
		gameState.playedMoves
			.map(move => move.alignmentsScore)
	} />
</section>

<section>
	<h2>Logs</h2>
	<pre class="card bordered" bind:this={logsHTMLElement}>{debuggingSession.logs}</pre>
</section>

<style>
	pre {
		overflow: auto;
		font-family: monospace;
		max-height: 50vh;
	}

	.action-title {
		display: flex;
		align-items: center;
		gap: 1rem;

		button {
			display: grid;
			place-items: center;
		}
	}
</style>
