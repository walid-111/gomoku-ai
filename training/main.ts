/* Genetic Algorithm :
- 1: Have a genetic representation of the solution.
- 2: Have a function to generate new solutions.
- 3: Have a fitness function to evaluate the solution. (win/lose, number of captures, alignment scores)
- 4: Have a selection function to select the best solutions.
- 5: Have an elitism function to keep the best solutions.
- 5: Have a crossover function to generate new solutions from the selected ones.
- 6: Have a mutation function to introduce some randomness.
*/

import os from 'node:os';
import Piscina from 'piscina';
import { Genes } from './genes.ts';
import { Individual } from './individual.ts';
import { GENERATION_FILE, getStats, listGenerations, readJSON, saveJSON, STATS_FILE } from './io.ts';
import { CurrentSpecies } from './species/index.ts';
import { time } from './time.ts';

const MIN_NB_MATCHES = 21;
// const NB_MATCHES_PENTE_ORG = 5;
const POPULATION_SIZE = 37;

const POOL_SIZE = os.availableParallelism() * 2;

class GeneticAlgorithm {
	populationSize: number;
	crossoverRate: number;
	mutationRate: number;
	elitismCount: number;

	population: Individual[];
	pool: Piscina;

	constructor(
		populationSize: number,
		crossoverRate: number,
		mutationRate: number,
		elitismCount: number
	) {
		this.populationSize = populationSize;
		this.crossoverRate = crossoverRate;
		this.mutationRate = mutationRate;
		this.elitismCount = elitismCount;

		this.population = Array.from(
			{ length: populationSize },
			() => new CurrentSpecies()
		);
		this.pool = new Piscina({
			filename: import.meta.resolve('./worker.ts'),
			execArgv: ['--experimental-strip-types'],
			minThreads: POOL_SIZE,
			maxThreads: POOL_SIZE
		});
	}

	getRandomIndividual() {
		return this.population[Math.floor(Math.random() * this.population.length)];
	}

	getRandomOpponents(individual: Individual, numOpponents: number) {
		// Making same matches are useless as the algorithm is deterministic
		let opponents = new Set<Individual>();

		while (opponents.size < numOpponents) {
			let opponent = this.getRandomIndividual();
			if (opponent !== individual) {
				opponents.add(opponent);
			}
		}

		return [...opponents];
	}

	/* Tournament selection */
	selectParent() {
		const TOURNAMENT_SIZE = 3;
		let best = this.getRandomIndividual();
		for (let i = 0; i < TOURNAMENT_SIZE; i++) {
			let current = this.getRandomIndividual();
			if (current.elo > best.elo) {
				best = current;
			}
		}
		return best;
	}

	async eval() {
		// const penteIndividual = new PenteORGSpecies();

		const matches = (await Promise.all(this.population.map(individual =>
			Promise.all(/*[
				// Matches against random opponents
				...*/this.getRandomOpponents(individual, MIN_NB_MATCHES)
					.map(async (opponent) => ({
						a: individual,
						b: opponent,
						result: await this.pool.run({
							genomes: [
								individual.genes.genome,
								opponent.genes.genome
							]
						})
					})),

				// Matches against pente.org
				// ...Array.from({ length: NB_MATCHES_PENTE_ORG }, async (_, i) => {
				// 	let a: Individual = individual;
				// 	let b: Individual = penteIndividual;
				// 	if (i % 2 === 1) {
				// 		[a, b] = [b, a];
				// 	}

				// 	return {
				// 		a,
				// 		b,
				// 		result: await this.pool.run({
				// 			genomes: [ a.genes.genome, b.genes.genome ],
				// 			species: [ a.id, b.id ]
				// 		})
				// 	};
				// })
		/*]*/)))).flat();

		// TODO discard player with too many losses (ex: 10 losses) / run less matches

		// Compute elo, we do multiple pass to stabilize the ranking
		for (let i = 0; i < 21; ++i) {
			// Make wins against pente.org worth more elo ??
			// penteIndividual.elo = 4000;

			for (let match of shuffle(matches)) {
				if (typeof match.result !== 'number') {
					// The match was not successful
					continue;
				}
				let expected = 1 / (1 + 10**((match.b.elo - match.a.elo) / 400));
				let actual = match.result;
				const K = 20;
				match.a.elo += K * (actual - expected);
				match.b.elo += K * (expected - actual);
			}
		}

		// console.log('pente.org elo', penteIndividual.elo);

		return this.population.sort((a, b) => b.elo - a.elo);
	}

	evolve() {
		let newPopulation = this.population.slice(0, this.elitismCount)
			// Don't copy the elo
			.map(individual => new CurrentSpecies(individual.genes));

		while (newPopulation.length < this.populationSize) {
			let parent1 = this.selectParent();
			let parent2; // make sure parents are different
			do { parent2 = this.selectParent(); } while (parent1 === parent2);

			let child = new CurrentSpecies(
				Genes.crossover(parent1.genes, parent2.genes, this.crossoverRate)
					.mutate(this.mutationRate)
			);
			newPopulation.push(child);
		}
		this.population = newPopulation;
	}
}

function shuffle<T>(array: T[]) {
	for (let i = array.length - 1; i >= 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[array[i], array[j]] = [array[j], array[i]];
	}
	return array;
}

let ga = new GeneticAlgorithm(
	POPULATION_SIZE,
	0.75,
	0.05,
	10
);

// List all genetic data files
let generation = listGenerations()[0] ?? 0;

const backupData = readJSON(GENERATION_FILE(generation), []);
for (let i in backupData) {
	for (let j in backupData[i].genes.genome) {
		ga.population[i].genes.genome[j] = backupData[i].genes.genome[j];
	}
}

console.log(`Loaded backup ${generation}`);

let statsHistory = readJSON(STATS_FILE, []);

while (true) {
	generation += 1;

	const stop = time('gen');
	const best = (await ga.eval())[0];

	// Make a backup
	if (generation % 100 === 0) {
		saveJSON(GENERATION_FILE(generation), ga.population);
		console.log(`Saved backup ${generation}`);
	}

	// Update stats for plot
	const stats = getStats(generation, ga.population);
	statsHistory.push(stats);
	saveJSON(STATS_FILE, statsHistory);

	ga.evolve();

	console.log(stats);
	console.log(best.genes);
	stop();
	console.log('\n' + '-'.repeat(32) + '\n\n');
}
