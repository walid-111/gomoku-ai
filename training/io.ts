import fs from 'node:fs';

export const GENES_VERSION = 'current';
export const STATS_FILE = '/stats.json';
export const GENERATION_FILE = (generation: number) => `/gen-${generation}.json`;

export type Stats = ReturnType<typeof getStats>;

export function getStats(generation: number, data: { elo: number }[]) {
	return {
		x: generation,
		max: data[0].elo,
		median: data[Math.floor(data.length / 2)].elo,
		avg: data.reduce((acc, { elo }) => acc + elo, 0) / data.length,
		min: data.at(-1).elo,
	};
}

const resolve = (path: string) => new URL(import.meta.resolve(`./genetic-data/${GENES_VERSION}${path}`)).pathname;

export function readJSON<T>(file: string, init: T) {
	try {
		return JSON.parse(fs.readFileSync(resolve(file), 'utf-8')) as T ?? init;
	} catch (e) {
		console.error(`Error reading ${file}`, e);
		return init;
	}
}

export function saveJSON<T>(file: string, data: T) {
	fs.writeFileSync(resolve(file), JSON.stringify(data, null, '\t'));
}

export function listGenerations() {
	const DIR = resolve('/');

	if (!fs.existsSync(DIR)) {
		fs.mkdirSync(DIR);
	}

	return fs.readdirSync(DIR)
		.map(file => +file.match(/gen-(\d+)\.json/)?.[1])
		.filter((x) => !isNaN(x))
		.sort((a, b) => b - a);
}
