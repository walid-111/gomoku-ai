export class Genes {
	public genome: Float32Array;

	constructor(genome: Float32Array) {
		this.genome = genome;
	}

	static random(genomeSize: number): Genes {
		return new Genes(Float32Array.from(
			{ length: genomeSize },
			() => Math.random()*2-1
		));
	}

	mutate(mutationRate: number): Genes {
		for (let i = 0; i < this.genome.length; ++i) {
			if (Math.random() < mutationRate) {
				if (Math.random() < 0.5) {
					// New random gene
					this.genome[i] = Math.random() * 2 - 1;
				} else {
					// Small random change
					this.genome[i] += (Math.random() - 0.5) * 0.05;
				}
			}
		}
		return this;
	}

	static crossover(genes1: Genes, genes2: Genes, crossoverRate: number): Genes {
		if (Math.random() >= crossoverRate) {
			// No crossover
			return Math.random() < 0.5 ? genes1 : genes2;
		}
		return new Genes(Float32Array.from(
			{ length: genes1.genome.length },
			(_, i) => {
				if (Math.random() < 0.5) {
					// Randomly select one of the genes
					return Math.random() < 0.5 ? genes1.genome[i] : genes2.genome[i]
				} else {
					// Mix of both genes
					const fac = Math.random();
					return genes1.genome[i] * fac + genes2.genome[i] * (1 - fac);
				}
			}
		));
	}
}
