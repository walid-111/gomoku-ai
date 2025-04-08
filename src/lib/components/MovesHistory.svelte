<script lang="ts">
	import { BOARD_SIZE, getGameState, type Move } from '../../routes/+page.svelte';

	let {
		rootMoveIndex,
		moves,
		onclick,
	}: {
		rootMoveIndex: number,
		moves: Move[];
		/// index = 0 for `root` and 1 for the first `moves`
		onclick: (index: number) => void;
	} = $props();

	const gameState = getGameState();

	let rootMove = $derived(gameState.playedMoves[rootMoveIndex]);

	function mod(n: number, m: number) {
		return ((n % m) + m) % m;
	}
</script>

{#snippet move(label: string, pos: number | undefined, index: number)}
	<button
		class="move"
		style:--stone-player-color="var(--stone-player-{mod(rootMoveIndex + index, 2) + 1})"
		onclick={() => onclick(index)}
		onmouseenter={() => {
			gameState.displayMoves([
				...gameState.displayedMoves.slice(0, rootMoveIndex + 1),
				...moves.slice(0, Math.max(index - 1, 0)),
			])

			gameState.ghost = (pos === undefined)
				? null
				: {
					x: pos % BOARD_SIZE,
					y: Math.floor(pos / BOARD_SIZE),
					allowed: true,
					turn: rootMoveIndex + index,
				};
		}}
	>{label}</button>
{/snippet}

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="moves" onmouseleave={() => {
		gameState.updateDisplayedMoves()
		gameState.ghost = null;
	}}>
	{@render move('root', rootMove?.pos, 0)}
	{#each moves as { pos }, index}
		{@render move(pos.toString(), pos, index + 1)}
	{/each}
</div>

<style>
	.moves {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
		align-items: center;
	}

	.move {
		color: rgb(var(--stone-player-color));
		background: rgba(var(--stone-player-color), 0.1);
		padding: 0.05rem 0.25rem;
		border-radius: 0.25rem;
	}
</style>
