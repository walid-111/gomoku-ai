import { Genes } from "./genes.ts";

export abstract class Individual {
	public elo = 1000;
	public genes: Genes;

	constructor(
		genomeSize: number,
		genes = Genes.random(genomeSize)
	) {
		this.genes = genes;
	}

	abstract id: number;
	abstract makeMoves(moves: Uint32Array): void;
	abstract pickMove(): Promise<number>;
	abstract free(): void;
}
