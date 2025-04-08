import { browser } from '$app/environment';
import { SETTINGS } from '$lib/storage.svelte.ts';

function sound(src: string, volume: number = 1) {
	if (!browser) {
		return {
			play() { }
		};
	}

	let audio = new Audio();

	fetch(src)
		.then(response => response.blob())
		.then(blob => {
			audio.src = URL.createObjectURL(blob);
		})
		.catch(console.error);

	return {
		play() {
			if (SETTINGS.sound.value) {
				let newAudio = (audio.cloneNode(false) as HTMLAudioElement);
				newAudio.volume = volume;
				newAudio.play();
			}
		}
	};
}

export const SOUND_PLACING = sound('sounds/placing.mp3', 0.3);
export const SOUND_LOSS = sound('sounds/loss.mp3', 0.3);
export const SOUND_WIN = sound('sounds/win.mp3', 0.3);
export const SOUND_END = sound('sounds/enigma.mp3', 0.5);
