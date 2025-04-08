import { Individual } from '../../individual.ts';
import Module from './ai.js';

await new Promise((resolve) => {
	Module.onRuntimeInitialized = resolve;
});

export function getAIMove(moves: number[]): number {
	const _arrayToHeap = function(typedArray: Int32Array){
		let numBytes = typedArray.length * typedArray.BYTES_PER_ELEMENT;
		let ptr = Module._malloc(numBytes);
		let heapBytes = new Uint8Array(Module.HEAPU8.buffer, ptr, numBytes);
		heapBytes.set(new Uint8Array(typedArray.buffer));
		return heapBytes;
	};
	let typedArray = new Int32Array(moves);
	let heapBytes = _arrayToHeap(typedArray);
	let move = Module.ccall('getAIMove', 'number', ['number','number','number','number','number'], [
		1 /*g*/,
		3 /*level*/,
		1 /*opening_book*/,
		heapBytes.byteOffset,
		typedArray.length
	]);
	Module._free(heapBytes.byteOffset);
	return move;
}

export class PenteORGSpecies extends Individual {
	moves: number[] = [];

	constructor() {
		super(0);
	}

	id = 0;

	makeMoves(moves: Uint32Array) {
		this.moves.push(...moves);
	}

	async pickMove() {
		return getAIMove(this.moves);
	}

	free() {}
}
