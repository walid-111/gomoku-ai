<script lang="ts">
	import Modal from "$lib/components/Modal.svelte";
	import Select from "$lib/components/Select.svelte";
	import IconComputer from "$lib/icons/game-mode/IconComputer.svelte";
	import IconLocal from "$lib/icons/game-mode/IconLocal.svelte";
	import IconPlay from "$lib/icons/IconPlay.svelte";
	import { storage } from "$lib/storage.svelte";
	import * as wasm from "$lib/ai/game-rs/game_rs";
	import { getGameState, PLAYER_TYPE, type PlayerType } from "../../routes/+page.svelte";

	const PLAYERS_OPTIONS = $state([
		PLAYER_TYPE.HUMAN,
		PLAYER_TYPE.COMPUTER,
		PLAYER_TYPE.PENTE_ORG,
	]);

	const OPENING_OPTIONS = new Map([
		[ 'None', {
			id: wasm.Opening.None,
			emoji: '🚫',
			description: undefined
		} ],
		[ 'Pro', {
			id: wasm.Opening.Pro,
			emoji: '3️⃣',
			description: '(center, any, >=3 from center)'
		} ],
		[ 'LongPro', {
			id: wasm.Opening.LongPro,
			emoji: '4️⃣',
			description: '(center, any, >=4 from center)'
		} ],
	]);

	const gameState = getGameState();

	const formatPlayer = {
		parse: (data: string) => PLAYERS_OPTIONS[parseInt(data)],
		stringify: (data: PlayerType) => PLAYERS_OPTIONS.findIndex((player) => player === data).toString(),
	}
	let customPlayer1 = storage<PlayerType>('customPlayer1', PLAYERS_OPTIONS[0], formatPlayer);
	let customPlayer2 = storage<PlayerType>('customPlayer2', PLAYERS_OPTIONS[0], formatPlayer);
	let customOpening = storage<string>('customOpening', 'None');

	function reset() {
		gameState.players[1] = PLAYER_TYPE.HUMAN;
		gameState.players[2] = PLAYER_TYPE.HUMAN;
		gameState.opening = wasm.Opening.None;
		gameState.reset();
	}
</script>

<Modal bind:open={gameState.selectionScreen} transient={false}>
{#snippet children({ close })}
	<!-- Quick mode -->
	<div class="button-actions">
		<button class="button button-bg"
			onclick={() => {
				reset();
				gameState.players[Math.round(Math.random()) + 1] = PLAYER_TYPE.COMPUTER;
				close();
			}}
		><IconComputer /> Computer</button>

		<button class="button button-bg"
			onclick={() => {
				reset();
				close();
			}}
		><IconLocal /> Local</button>
	</div>

	<div class="hr">
		<hr>
		or
		<hr>
	</div>

	<!-- Custom mode -->
	<div class="button-actions">

		<Select bind:value={customPlayer1.value} options={PLAYERS_OPTIONS}>
			{#snippet option(option: PlayerType)}
				<img src={option.avatar(1)} alt="" class="mini-avatar" />
				<span>{option.name}</span>
			{/snippet}
		</Select>
		<Select bind:value={customPlayer2.value} options={PLAYERS_OPTIONS}>
			{#snippet option(option: PlayerType)}
				<img src={option.avatar(2)} alt="" class="mini-avatar" />
				<span>{option.name}</span>
			{/snippet}
		</Select>
	</div>

	<Select bind:value={customOpening.value} options={[...OPENING_OPTIONS.keys()]}>
		{#snippet option(option: string)}
			<div class="opening-icon">
				{OPENING_OPTIONS.get(option)?.emoji}
			</div>
			<span>{option}
				{#if OPENING_OPTIONS.get(option)?.description}
					<small>{OPENING_OPTIONS.get(option)?.description}</small>
				{/if}
			</span>
		{/snippet}
	</Select>

	<div class="button-actions">
		<button class="button button-bg-primary"
			onclick={() => {
				gameState.players[1] = customPlayer1.value;
				gameState.players[2] = customPlayer2.value;
				gameState.opening = OPENING_OPTIONS.get(customOpening.value)?.id ?? wasm.Opening.None;
				gameState.reset();
				close();
			}}
		>Play <IconPlay /></button>
	</div>
{/snippet}
</Modal>

<style>
	.mini-avatar {
		width: 2rem;
		height: 2rem;
		object-fit: cover;
		border-radius: 0.5rem;
	}

	.opening-icon {
		width: 2rem;
		height: 2rem;
		display: grid;
		place-items: center;
		font-size: 1.25rem;
	}

	.hr {
		display: flex;
		align-items: center;
		gap: 0.5rem;

		hr {
			flex: 1;
			border-top: 1px solid var(--border);
		}
	}
</style>
