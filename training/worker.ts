console.log('worker.ts');

import { CurrentSpecies, SPECIES, wasm } from './species/index.ts';
import { Genes } from './genes.ts';

const MAX_TURNS = 128;

/// 1 = A won, -1 = B won, 0 = draw
export default async function run({
	genomes,
	species: speciesIds
}: {
	genomes: [Float32Array, Float32Array],
	species?: [number, number]
}): Promise<number> {
	const individuals = speciesIds
		? speciesIds.map((id, i) => new SPECIES[id](new Genes(genomes[i])))
		: [new CurrentSpecies(new Genes(genomes[0])), new CurrentSpecies(new Genes(genomes[1]))];

	// const stop = time('\x1b[1A\x1b[2Krun');

	let gameState = new wasm.PackedGameState();
	let movesCount = 0;
	let moveResult;

	while (gameState.winner === wasm.Player.None && movesCount < MAX_TURNS) {
		const player = gameState.currentPlayer === wasm.Player.Player1 ? 0 : 1;

		let move = new Uint32Array([await individuals[player].pickMove()]);

		moveResult = gameState.makeMoves(move);
		if (moveResult === wasm.MoveResult.Invalid) break;
		for (const individual of individuals) {
			individual.makeMoves(move);
		}
		movesCount += 1;
	}

	// stop();

	const { winner } = gameState;

	gameState.free();
	for (const individual of individuals) {
		individual.free();
	}

	// console.error(winner, ['forf', 'align', 'capt'][winReason]);
	if (moveResult === wasm.MoveResult.Invalid) return null;
	if (winner === wasm.Player.Player1) return 1;
	if (winner === wasm.Player.Player2) return 0;
	return 0.5;
}
