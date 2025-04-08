import { PenteORGSpecies } from './pente.org/index.ts';

import { Genes } from '../genes.ts';
import { Individual } from '../individual.ts';

import fs from 'node:fs';
import path from 'node:path';

globalThis.debugLog = console.log;
globalThis.debugTimeStart = () => { };
globalThis.debugTimeEnd = () => { };
globalThis.debugOverlay = () => { };

import * as wasm from './current/game_rs.js';
wasm.initSync(fs.readFileSync(path.resolve(
	import.meta.dirname,
	'./current/game_rs_bg.wasm'
)));

export { wasm };

class CurrentSpecies extends Individual {
	gameState = new wasm.PackedGameState();

	constructor(genes?: Genes) {
		super(5, genes);
		// this.gameState.loadGenetic(this.genes.genome);
	}

	id = 1;

	makeMoves(moves: Uint32Array) {
		this.gameState.makeMoves(moves);
	}

	async pickMove() {
		const aiResult = await wasm.pickMoveDefault(this.gameState);
		const move = aiResult.pickedMove;

		aiResult.free();
		return move;
	}

	free() {
		this.gameState.free();
	}
};

export const SPECIES: (new (genes?: Genes) => Individual)[] = [
	PenteORGSpecies,
	CurrentSpecies,
];

export { CurrentSpecies, PenteORGSpecies };
