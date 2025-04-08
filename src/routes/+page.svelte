<script lang="ts" module>
	import { debuggingSession } from '$lib/ai/debug.svelte.ts';
	import { getAIMove } from "$lib/ai/pente.org/lib.ts";
	import { DEBUG_PANEL_INDEX } from '$lib/NavBar.svelte';
	import SidePanel, { selectedPanel } from "$lib/SidePanel.svelte";
	import { SOUND_END, SOUND_LOSS, SOUND_PLACING, SOUND_WIN } from "$lib/sound.ts";
	import * as wasm from "$lib/ai/game-rs/game_rs";
	import { getContext, setContext } from 'svelte';

	import avatarComputer from '$lib/icons/avatar/computer.webp';
	import avatarHuman1 from '$lib/icons/avatar/human1.webp';
	import avatarHuman2 from '$lib/icons/avatar/human2.webp';
	import avatarPenteOrg from '$lib/icons/avatar/pente.org.svg';

	// TODO Higher&dynamic depth in web mode than training

	export type Move = {
		pos: number;
		score?: number;
		shadowMoveIndex?: number;
	};

	export type DetailedMove = Move & {
		alignmentsScore: number;
	};

	export type PlayerType = {
		avatar: (player: number) => string;
		readonly name: string;
		pickMove?: (gameState: ReactiveGameState) => Promise<Move>;
	};

	export const PLAYER_TYPE: {
		HUMAN: PlayerType;
		COMPUTER: PlayerType;
		PENTE_ORG: PlayerType;
	} = {
		HUMAN: {
			avatar: (player: number) => [,avatarHuman1,avatarHuman2][player] ?? '',
			name: 'Human',
		},
		COMPUTER: {
			// id: 0,
			avatar: () => avatarComputer,
			name: 'Computer',
			pickMove: async (gameState: ReactiveGameState) => {
				let { inner } = gameState.inner(gameState.playedMoves);
				gameState.aiResult = await wasm.pickMove(
					new Uint16Array(gameState.playedMoves.map((move) => move.pos)),
					inner,
					selectedPanel.current.index == DEBUG_PANEL_INDEX
						&& collectDebugInfo.value
				);

				return {
					pos: gameState.aiResult.pickedMove,
					score: gameState.aiResult.pickedMoveScore
				};
			}
		},
		PENTE_ORG: {
			// id: 1,
			avatar: () => avatarPenteOrg,
			name: 'Pente.org',
			pickMove: async (gameState: ReactiveGameState) => {
				return {
					pos: getAIMove(gameState.playedMoves.map((move) => move.pos))
				};
			}
		}
	};

	export enum WinReason {
		None = wasm.MoveResult.Ok,
		Capture = wasm.MoveResult.WonByCapture,
		Alignment = wasm.MoveResult.WonByAlignment,
		Forfeit = 9
	};

	export class ReactiveGameState {
		public winner: number = $state()!;

		public board: number[] = $state()!;
		public capturesCount: Map<number, number> = $state()!;
		public allowedMovesBitMask: Uint32Array = $state()!;
		public winningAlignmentBitMask: Uint32Array = $state()!;

		public winReason: WinReason = $state()!;
		public selectionScreen: boolean = $state(true);

		public playedMoves: DetailedMove[] = $state([]);
		public debugMoves: Move[] = $state([]);
		public displayedMoves: Move[] = $state([]);

		public opening = wasm.Opening.None;

		public players: Record<number, PlayerType> = $state({
			1: PLAYER_TYPE.HUMAN,
			2: PLAYER_TYPE.HUMAN
		});

		public aiResult?: wasm.AIResult;
		public ghost: {
			x: number, y: number,
			allowed?: boolean,
			turn?: number,
		} | null = $state(null);


		constructor() {
			this.reset();
		}

		reset() {
			this.playedMoves = [];
			this.debugMoves = [];
			this.winner = wasm.Player.None;
			this.winReason = WinReason.None;
			this.updateDisplayedMoves();
		}

		updateDisplayedMoves() {
			if (0 < this.debugMoves.length) {
				return this.displayMoves([
					...this.playedMoves.slice(0, -1),
					...this.debugMoves
				]);
			} else {
				return this.displayMoves(this.playedMoves);
			}
		}

		displayMoves(moves: Move[]) {
			let result = this.inner(moves);
			if (result.result == wasm.MoveResult.Invalid) {
				return result;
			}

			this.displayedMoves = moves;

			// Update board
			this.board = result.inner.board;
			this.capturesCount = new Map([
				[1, result.inner.getPlayerCapturesCount(1)],
				[2, result.inner.getPlayerCapturesCount(2)]
			]);
			this.allowedMovesBitMask = result.inner.allowedMovesBitMask();
			let lastMove = this.playedMoves.at(-1);
			this.winningAlignmentBitMask = (result.inner.winner !== wasm.Player.None && lastMove !== undefined)
				? result.inner.winningAlignmentBitMask(lastMove.pos)
				: new Uint32Array();

			// update overlays
			debuggingSession.overlays.clear();
			if (this.playedMoves.length-1 <= moves.length) {
				wasm.updateOverlays(result.inner);
				this.aiResult?.updateOverlays(
					new Uint16Array(
						moves
							.slice(Math.max(0, this.playedMoves.length - 1))
							.map((move) => move.pos)
					)
				)
			}

			return result;
		}

		inner(moves: Move[]) {
			let inner = new wasm.PackedGameState();
			inner.opening = this.opening;
			let result = inner.makeMoves(new Uint32Array(moves.map((move) => move.pos)));
			return { inner, result };
		}

		makeMove(move: Move) {
			this.playedMoves.push({
				...move,
				// TODO Fix alignmentsScore graph
				alignmentsScore: 0
			});
			let debugMoves = this.debugMoves;
			this.debugMoves = [];

			let result = this.updateDisplayedMoves();
			if (result.result == wasm.MoveResult.Invalid) {
				this.playedMoves.pop();
				this.debugMoves = debugMoves;
				return false;
			}

			this.ghost = null;
			SOUND_PLACING.play();
			this.winner = result.inner.winner;
			this.winReason = result.result as unknown as WinReason;
			if (this.winner) {
				const isWinnerAI = this.players[this.winner].pickMove;
				const isLoserAI = this.players[this.winner == 1 ? 2 : 1].pickMove;
				if (isWinnerAI && !isLoserAI) {
					SOUND_LOSS.play();
				} else if (!isWinnerAI && isLoserAI) {
					SOUND_WIN.play();
				} else {
					SOUND_END.play();
				}
			}
			return true;
		}

		undoMove(count = 1) {
			this.playedMoves = this.playedMoves.slice(0, -count);
			this.debugMoves = [];
			this.updateDisplayedMoves();
			this.winner = wasm.Player.None;
			this.winReason = WinReason.None;
		}

		/**
		 * Return the player that will play at `turn`.
		 * If `turn`=0,2,4... then the first player (`1`) will be returned
		 * Else the second player (`2`) will be returned
		 */
		playerAtTurn(turn: number): wasm.Player {
			return (turn % 2) + 1;
		}

		playerTypeAtTurn(turn: number): PlayerType {
			return this.players[this.playerAtTurn(turn)];
		}

		forfeit() {
			this.winner = this.playerAtTurn(this.playedMoves.length + 1);
			this.winReason = WinReason.Forfeit;
		}

		allowed(x: number, y: number) {
			return this.winner === wasm.Player.None
				&& (this.allowedMovesBitMask[y] & (1 << x)) != 0
		}
	}

	export const BOARD_SIZE = 19;

	export function getGameState(): ReactiveGameState {
		return getContext('gameState');
	}
</script>

<script lang="ts">
	import ResultScreen from "$lib/screens/ResultScreen.svelte";
	import SelectionScreen from "$lib/screens/SelectionScreen.svelte";
	import { collectDebugInfo } from '$lib/panels/Debug.svelte';
	import SideBarToggle from '$lib/components/SideBarToggle.svelte';
	import { media } from '$lib/media.svelte';

	let gameState = new ReactiveGameState();
	setContext('gameState', gameState);

	let timer: NodeJS.Timeout;
	$effect(() => {
		let { pickMove } = gameState.playerTypeAtTurn(gameState.playedMoves.length);
		if (gameState.winner !== wasm.Player.None || pickMove === undefined) {
			return ;
		}


		clearTimeout(timer);
		timer = setTimeout(async () => {
			gameState.aiResult = undefined;
			debuggingSession.reset();
			let move = await pickMove(gameState);
			debuggingSession.logTimings();

			if (!gameState.makeMove(move)) {
				debuggingSession.debugLog(`AI made an invalid move: ${move.pos}`);
				gameState.forfeit();
			}
		}, 30);
	});

	let ctrlPressed = $state(false);
	let ghostTurn = $derived(
		gameState.ghost?.turn
		?? (ctrlPressed
			? (Math.max(0, gameState.playedMoves.length - 1) + gameState.debugMoves.length)
			: gameState.playedMoves.length)
	);
	let ghostAllowed = $derived(
		gameState.ghost && (
			gameState.ghost.allowed ?? gameState.allowed(gameState.ghost.x, gameState.ghost.y)
		));


	let smallScreen = media('(max-width:1000px)');
	let sideBarOpen = $state(!smallScreen.matches);
</script>

<svelte:head>
	<title>Pente</title>
</svelte:head>

<svelte:window
	onkeydown={(e) => ctrlPressed = e.ctrlKey}
	onkeyup={(e) => ctrlPressed = e.ctrlKey}
	onmouseenter={(e) => ctrlPressed = e.ctrlKey}
/>

<ResultScreen />
<SelectionScreen />

<div id="game-container" class:hide={sideBarOpen && smallScreen.matches}>
	<div class="card" id="game">
		{#if selectedPanel.current.index == DEBUG_PANEL_INDEX && gameState.ghost}
			{@const overlayValue = debuggingSession.selectedOverlay?.[gameState.ghost.y * BOARD_SIZE + gameState.ghost.x]}

			<span class="debug-text">
				x:{gameState.ghost.x} y:{gameState.ghost.y} ({gameState.ghost.y * BOARD_SIZE + gameState.ghost.x})
				{#if overlayValue}
					=> {overlayValue?.text}
				{/if}
			</span>
		{/if}

		<svg viewBox="0 0 {BOARD_SIZE} {BOARD_SIZE}">
			<pattern id="grid" width="1" height="1" patternUnits="userSpaceOnUse">
				<line x1="0.5" y1="0" x2="0.5" y2="1" stroke="var(--border)" stroke-width="2" vector-effect="non-scaling-stroke" />
				<line x1="0" y1="0.5" x2="1" y2="0.5" stroke="var(--border)" stroke-width="2" vector-effect="non-scaling-stroke" />
			</pattern>
			<rect fill="url(#grid)" width="100%" height="100%" />

			<pattern id="diagonal-hatch" width="0.25" height="0.25" patternTransform="rotate(45 0 0)">
				<line x1="0" y1="0" x2="0" y2="10" stroke="rgb(var(--red))" stroke-width="0.15" />
				<rect x="0" y="0" width="0.25" height="0.25" fill="rgba(var(--red), 0.2)" />
			</pattern>

			{#each gameState.board as cell, i}
				{@const y = Math.floor(i / BOARD_SIZE)}
				{@const x = i % BOARD_SIZE}
				{@const overlayCell = selectedPanel.current.index == DEBUG_PANEL_INDEX ? debuggingSession.selectedOverlay?.[i] : undefined}
				{@const allowed = gameState.allowed(x, y)}
				{@const isInWinningAlignment = gameState.winningAlignmentBitMask[y] & (1 << x)}

				<rect
					{x} {y}
					width="1" height="1"
					class="cell"
					class:not-allowed={!allowed}
					style:fill={overlayCell && `rgba(${overlayCell.value < 0 ? 'var(--red)' : 'var(--green)'}, ${overlayCell.fac})`}
					onclick={(e: MouseEvent) => {
						if (ctrlPressed) {
							// Debug/shadow move mode
							if (gameState.playedMoves.length <= gameState.displayedMoves.length) {
								gameState.debugMoves.push({ pos: i, shadowMoveIndex: gameState.debugMoves.length });
								if (gameState.updateDisplayedMoves().result == wasm.MoveResult.Invalid) {
									gameState.debugMoves.pop();
								}
							}
							return ;
						}

						// Play move
						if (gameState.playerTypeAtTurn(gameState.playedMoves.length).pickMove === undefined) {
							gameState.aiResult = undefined;
							gameState.makeMove({ pos: i });
						}
					}}
					onmouseenter={() => gameState.ghost = { x, y }}
					onmouseleave={() => gameState.ghost = null}
					aria-hidden="true"
				/>

				{#if cell == wasm.Player.None && gameState.winner === wasm.Player.None && !allowed}
					<rect
						{x} {y}
						width="1" height="1"
						fill="url(#diagonal-hatch)"
						class="diagonal-hatch"
					/>
				{/if}

				{#if cell !== wasm.Player.None}
					<circle
						cx={x + 0.5} cy={y + 0.5}
						r="0.4"
						style:fill="rgb(var(--stone-player-{cell}))"
						class="stone"
						class:won-blink={isInWinningAlignment}
					/>

					{#if gameState.displayedMoves.at(-1)?.pos === i}
						<circle
							cx={x + 0.5} cy={y + 0.5}
							r="0.2"
							fill="var(--layer-0)"
							class:won-blink={isInWinningAlignment}
						/>
					{/if}

					{@const lastMoveOnCell = gameState.displayedMoves.findLast(move => move.pos === i)}
					{#if lastMoveOnCell?.shadowMoveIndex !== undefined}
						<text class="shadow-move" x={x + 0.5} y={y + 0.5} dominant-baseline="middle" text-anchor="middle">
							{lastMoveOnCell.shadowMoveIndex + 1}
						</text>
					{/if}
				{/if}

				{#if overlayCell}
					<text x={x + 0.5} y={y + 0.5} dominant-baseline="middle" text-anchor="middle">{overlayCell.text}</text>
				{/if}
			{/each}

			<circle
				class="ghost"
				cx={0.5 + (gameState.ghost?.x??0)}
				cy={0.5 + (gameState.ghost?.y??0)}
				r="0.4"
				style:--stone-player-color="var(--stone-player-{gameState.playerAtTurn(ghostTurn)})"
				style:display={ghostAllowed ? "unset" : "none"}
				style="fill:rgb(var(--stone-player-color))"
				/>

			<style>
				@keyframes grow {
					0% { transform: scale(0); }
					to { transform: scale(1); }
				}

				@keyframes won-blink {
					0%, 100% { opacity: 1; }
					50% { opacity: 0.3; }
				}

				.cell {
					cursor: pointer;
					fill: #0000;

				}
				.not-allowed {
					cursor: not-allowed;
				}
				.diagonal-hatch, circle, text {
					pointer-events: none;
				}

				circle {
					animation: grow var(--snappy);
					transform-origin: center;
					transform-box: fill-box;
				}

				text {
					font-size: 0.25px;
					text-shadow: none;
					fill: white;
					mix-blend-mode: difference;

					&.shadow-move {
						font-size: 0.7px;
					}
				}

				.ghost {
					transition: cx var(--snappy), cy var(--snappy);
					opacity: 0.25;
				}

				.won-blink {
					animation: won-blink .5s infinite;
				}
			</style>
		</svg>

		<div class="captures-count">
			{#snippet capturesCount(player: number, capturesCount: number)}
				<div title="Player {player} Captures Count">
					{#each new Array(5).fill(null) as _, i}
						<div
							style="position:relative"
							class:ghost={i >= capturesCount}
							class:won-blink={capturesCount >= 5}
						>
							<div
								class="stone"
								style:background="rgb(var(--stone-player-{player}))"
							></div>
						</div>
					{/each}
				</div>
			{/snippet}

			{@render capturesCount(1, gameState.capturesCount.get(1)!)}
			<span title="Turn">
				{Math.floor(gameState.playedMoves.length / 2)}
			</span>
			{@render capturesCount(2, gameState.capturesCount.get(2)!)}
		</div>
	</div>
</div>

<SideBarToggle bind:open={sideBarOpen} />

{#if sideBarOpen}
<SidePanel />
{/if}

<style>
	#game-container {
		display: flex;
		flex-direction: column;
		justify-content: center;
		aspect-ratio: 1;
	}

	#game {
		box-shadow: 0 0 4rem inset #0b095b2e;
		position: relative;

		& > svg {
			display: flex;
		}
	}

	.stone {
		position: absolute;
		inset: 5%;
		border-radius: 50%;
		transition: opacity var(--snappy);
	}


	.captures-count {
		display: flex;
		justify-content: space-between;
		position: absolute;
		width: 100%;
		left: 0;
		top: 100%;

		& > div {
			display: flex;
			align-items: center;

			&:last-child {
				flex-direction: row-reverse;
			}

			& > div {
				width: 1rem;
				height: 1rem;
			}
		}
	}

	.debug-text {
		font-size: 0.75rem;
		position: absolute;
		font-family: monospace;
		pointer-events: none;
		top: -1rem;
	}
</style>

<!--
TODO(important) Select if debug enabled ? / show response time in all cases??
TODO Show ai reflecting ??
TODO Thanos Snap Disintegration Effect on restart + animate place
https://elgoog.im/thanos/
-->
