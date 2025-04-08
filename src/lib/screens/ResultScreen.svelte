<script lang="ts">
	import Modal from "$lib/components/Modal.svelte";
	import IconCrown from "$lib/icons/IconCrown.svelte";
	import { getGameState, WinReason } from "../../routes/+page.svelte";

	const gameState = getGameState();
</script>

<Modal open={gameState.winReason !== WinReason.None}>
{#snippet children({ close })}
	<div>
		<h2 style:color="rgb(var(--stone-player-{gameState.winner}))">
			{gameState.players[gameState.winner]?.name} Won
		</h2>
		<p>
			in {Math.floor(gameState.playedMoves.length / 2)} turns
			{#if gameState.winReason === WinReason.Capture}
				by capture
			{:else if gameState.winReason === WinReason.Alignment}
				by alignment
			{:else if gameState.winReason === WinReason.Forfeit}
				by forfeit
			{/if}
		</p>

	</div>

	{#snippet player(playerId: number)}
		{@const player = gameState.players[playerId]}
		{@const won = playerId === gameState.winner}

		<div
			class="player"
			style:--stone-player-color="var(--stone-player-{playerId})"
			class:won
		>
			<img src={player.avatar(playerId)} alt="">
			{#if won}
			<div class="crown"><IconCrown /></div>
			{/if}
			<span>{player.name}</span>
		</div>
	{/snippet}

	<div class="vs">
		{@render player(1)}
		<span>vs</span>
		{@render player(2)}
	</div>

	<div class="button-actions">
		<button class="button button-bg" onclick={close}>Close</button>
		<button class="button button-bg-primary" onclick={() => {
			gameState.reset();
			close();
		}} autofocus>Rematch</button>
	</div>
{/snippet}
</Modal>

<style>
	h2, p {
		text-align: center;
	}

	.vs {
		display: flex;
		gap: 1rem;
		align-items: center;
		justify-content: center;
	}

	@media (max-width: 30rem) {
		.vs {
			flex-direction: column;
		}
	}

	.player {
		position: relative;
		display: grid;
		place-items: center;

		img {
			width: 8rem;
			height: 8rem;
			object-fit: cover;
			border-radius: 1rem;
			border: 0.5rem solid var(--layer-1);
		}

		&.won img {
			border-color: rgb(var(--stone-player-color));
		}
	}

	.crown {
		position: absolute;
		font-size: 2rem;
		background: rgb(var(--stone-player-color));
		color: var(--layer-0);
		border-radius: 1rem;
		padding: 0.25rem;
		display: flex;
		top: 0;
		right: 0;
	}
</style>
