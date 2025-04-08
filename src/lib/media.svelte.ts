export function media(query: string): { readonly matches: boolean } {
	let mediaQuery = window.matchMedia(query);

	let matches = $state(mediaQuery.matches);

	mediaQuery.addListener(() => {
		matches = mediaQuery.matches;
	});

	return {
		get matches() {
			return matches;
		}
	}
}
