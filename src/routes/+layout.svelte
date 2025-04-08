<script>
	import AIWorker from './game-rs-worker.ts?worker&inline';

	import * as wasm from "$lib/ai/game-rs/game_rs";
	import WASM_URL from "$lib/ai/game-rs/game_rs_bg.wasm?url";

	const wasmModule = fetch(WASM_URL)
		.then((response) => response.arrayBuffer())
		.then((bytes) => WebAssembly.compile(bytes));

	class RustThread {
		constructor() {
			this.worker = new AIWorker();
			this.promiseResolvers = new Map();
			this.id = 0;

			this.worker.onmessage = (event) => {
				const { id, result } = event.data;
				const resolver = this.promiseResolvers.get(id);
				if (resolver) {
					resolver(result);
					this.promiseResolvers.delete(id);
				} else {
					console.error('No resolver for', id);
				}
			};
		}

		send(data) {
			console.log('send', data);
			return new Promise((resolve) => {
				this.promiseResolvers.set(this.id, resolve);
				this.worker.postMessage({
					id: this.id,
					data
				});
				this.id += 1;
			});
		}
	}

	const pool = new Array(32).fill().map(() => new RustThread());
	let roundRobin = 0;

	globalThis.threadSend = async (data) => {
		const thread = pool[roundRobin];
		roundRobin = (roundRobin + 1) % pool.length;
		return thread.send(data);
	};

	wasmModule.then((module) => {
		for (let thread of pool) {
			thread.worker.postMessage({ type: 'init', module });
		}
	});


	import '$lib/style.postcss';

	import '$lib/ai/debug.svelte.ts';

	let { children } = $props();
</script>

{#await wasmModule.then((module) => wasm.initSync({ module })) then _}
	{@render children()}
{/await}

<svg width="100%" height="100%" class="overlay-noise">
	<filter id="noise">
		<feTurbulence type="fractalNoise" baseFrequency="0.45" numOctaves="1" stitchTiles="stitch"></feTurbulence>
	</filter>
	<rect width="100%" height="100%" filter="url(#noise)"></rect>
</svg>

<style>
	.overlay-noise {
		pointer-events: none;
		position: fixed;
		inset: 0;
		z-index: 999999;
		opacity: 0.15;
	}
</style>
