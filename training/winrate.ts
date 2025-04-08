import { CurrentSpecies, PenteORGSpecies } from "./species/index.ts";
import Piscina from 'piscina';
import os from 'node:os';

const GAMES = 256;

const individual = new CurrentSpecies();
const penteOrg = new PenteORGSpecies();


let GENES;

// gen 20574
GENES = [
	0.07311465591192245,
    0.00029573231586255133,
    0.7708396911621094,
    -0.055403370410203934,
    0.006159722339361906,
    -0.2296372652053833,
    -0.8280090689659119,
    -0.4332255721092224,
    0.07784324139356613,
    0.21065062284469604,
    -0.15054847300052643,
    -0.38377872109413147,
    -0.9225600361824036
]

for (const key in GENES) {
	individual.genes.genome[key] = GENES[key];
}


const POOL_SIZE = os.availableParallelism() * 2;
const pool = new Piscina({
	filename: import.meta.resolve('./worker.ts'),
	execArgv: ['--experimental-strip-types'],
	minThreads: POOL_SIZE,
	maxThreads: POOL_SIZE
});

let stats = new Map([
	[0, 0],
	[0.5, 0],
	[1, 0],
	[null, 0],
]);

const results = new Array(GAMES)
	.fill(0)
	.map(async (_, i) => {
		const genomes = [individual.genes.genome, penteOrg.genes.genome];
		const species = [individual.id, penteOrg.id];

		if (i % 2 === 0) {
			genomes.reverse();
			species.reverse();
		}

		const result = await pool.run({
			genomes,
			species
		});

		if (result !== null && i % 2 === 0) {
			return 1 - result;
		}

		return result;
	});

const red = '\x1b[1;91m';
const yellow = '\x1b[1;93m';
const green = '\x1b[1;92m';
const gray = '\x1b[1;90m';
const reset = '\x1b[0m';

function display(stat: number, label: string, color: string) {
	const total = stats.get(1) + stats.get(0) + stats.get(0.5);
	return `${label} ${color}${(stat / total * 100)
		.toFixed(1)
		.padStart(5, ' ')
	}%${reset} (${stat})`;
}

for await (let result of results) {
	stats.set(result, stats.get(result) + 1);
	console.log(
		display(stats.get(1), 'Won', green),
		display(stats.get(0), 'Lost', red),
		display(stats.get(0.5), 'Draw', yellow),
		'    ' + display(stats.get(null), 'Forfeit', gray)
	);
}
