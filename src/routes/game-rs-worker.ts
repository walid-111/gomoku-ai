import * as wasm from "$lib/ai/game-rs/game_rs";

globalThis.debugLog = () => { };
globalThis.debugTimeStart = () => { };
globalThis.debugTimeEnd = () => { };
globalThis.debugOverlay = () => { };

self.onmessage = function (event) {
	if (event.data.type === 'init') {
		wasm.initSync({ module: event.data.module });
		return;
	}

	const { id, data } = event.data;

	setTimeout(() => {
		console.log('received task', id, data);
		const result = wasm.threadHandle(data);
		console.log('sending task result', id, result);
		self.postMessage({ id, result });
	}, 100);
};
