import { storage } from '$lib/storage.svelte';
import { type DebugOverlayCell, type DebugOverlaySettings } from "$lib/ai/game-rs/game_rs";
import { SvelteMap } from 'svelte/reactivity';

type Overlay = ({
	text: string,
	fac: number,
	value: number,
} | undefined)[];

export class DebuggingSession {
	startMs = $state(performance.now());
	public durationMs = $state(0);


	public logs = $state('');


	#timingsMap = new Map<string, number>();
	public timings: {
		name: string,
		startMs: number,
		durationMs: number
	}[] = $state([]);


	public overlays = new SvelteMap<string, Overlay>();
	public selectedOverlayName = storage<string>('selected-overlay', 'none');
	get selectedOverlay(): Overlay | undefined {
		return this.overlays.get(this.selectedOverlayName.value);
	}

	constructor() { }

	reset() {
		this.startMs = performance.now();
		this.durationMs = 0;
		this.#timingsMap.clear();
		this.timings = [];
		this.overlays.clear();
	}

	logTimings() {
		this.durationMs = this.ms();
	}

	ms() {
		return performance.now() - this.startMs;
	}



	debugLog(message: string) {
		this.logs += `[${this.ms().toFixed(1)}] ${message}\n`;
	}

	debugTimeStart(label: string) {
		if (this.#timingsMap.has(label)) {
			this.debugLog(`Timing "${label}" already started`);
			return ;
		}

		this.#timingsMap.set(label, this.ms());
	}

	debugTimeEnd(label: string) {
		const startMs = this.#timingsMap.get(label);

		if (startMs === undefined) {
			this.debugLog(`Timing "${label}" not started`);
			return ;
		}

		this.#timingsMap.delete(label);

		this.timings.push({
			name: label,
			startMs,
			durationMs: this.ms() - startMs
		})
	}

	debugOverlay(name: string, settings: DebugOverlaySettings, values: DebugOverlayCell[]) {
		const overlay = values.map(value => {
			if (value.value !== undefined) {
				return {
					text: value.value.toFixed(settings.precision),
					fac: Math.abs(value.value) / settings.max,
					value: value.value,
				};
			}
		});

		this.overlays.set(name, overlay);
	}
}


export const debuggingSession = new DebuggingSession();

declare global {
	interface Window {
		debugLog:       (message: string) => void;
		debugTimeStart: (label: string)    => void;
		debugTimeEnd:   (label: string)    => void;
		debugOverlay:   (name: string, settings: DebugOverlaySettings, values: DebugOverlayCell[]) => void;
	}
}

globalThis.debugLog =       (...args) => debuggingSession.debugLog      (...args);
globalThis.debugTimeStart = (...args) => debuggingSession.debugTimeStart(...args);
globalThis.debugTimeEnd =   (...args) => debuggingSession.debugTimeEnd  (...args);
globalThis.debugOverlay =   (...args) => debuggingSession.debugOverlay  (...args);
