import pkg, { type Plot } from 'nodeplotlib';
import { readJSON, STATS_FILE } from './io.ts';
const { plot } = pkg;

const cachedStats = readJSON(STATS_FILE, []);

const stats = [
	...new Map(cachedStats
		.map((stat) => [stat.x, stat])
	).values()
].sort((a, b) => a.x - b.x);

const data: Plot[] = ['max', 'median', 'avg', 'min']
	.map(name => ({
		name,
		x: stats.map(g => g.x),
		y: stats.map(g => g[name]),
		type: 'scatter',
	}));

plot(data);
