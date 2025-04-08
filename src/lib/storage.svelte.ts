import { browser } from '$app/environment'

let listerners = new Map<string, (() => void)[]>();

if (browser) {
	window.addEventListener('storage', (e) => {
		listerners.get(e.key!)?.forEach((listener) => listener());
	});
}

export function storage<T>(
	key: string,
	defaultValue: T,
	format: { parse: (data: string) => T, stringify: (data: T) => string } = JSON
): { value: T } {
	let value = $state(defaultValue);

	const loadStorage = () => {
		if (!browser) {
			return ;
		}
		try {
			let newValue = format.parse(localStorage.getItem(key)!) ?? defaultValue;
			if (newValue !== value) {
				value = newValue;
			}
		} catch(e) {
			console.error('load storage', e);
		}
	}

	loadStorage();

	listerners.set(key, [
		...(listerners.get(key) ?? []),
		loadStorage
	]);

	return {
		get value() {
			return value;
		},
		set value(_value: T) {
			value = _value;
			try {
				const data = format.stringify(value);
				localStorage.setItem(key, data);
			} catch (e) {
				console.error('save storage', e);
			}
		}
	}
}

export const SETTINGS = {
	sound: storage<boolean>('SETTINGS.sound', true),
}
