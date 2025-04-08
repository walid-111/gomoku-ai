<script lang="ts">
	import { tweened } from 'svelte/motion';

	let { values }: {
		values: number[];
	} = $props();

	function clamp(min: number, value: number, max: number) {
		return Math.min(max, Math.max(min, value));
	}

	let min = $derived(clamp(-10_000, -Math.max(...values), -1));
	let max = $derived(clamp(1, -Math.min(...values), 10_000));

	let height = $derived(max - min);

	let boundingBoxMin = tweened(-1);
	let boundingBoxHeight = tweened(2);
	let boundingBoxWidth = tweened(1);

	$effect(() => {
		boundingBoxMin.set(min);
		boundingBoxHeight.set(height);
		boundingBoxWidth.set(Math.max(values.length - 1, 1));
	});
</script>

<svg
	viewBox="0 {$boundingBoxMin} {$boundingBoxWidth} {$boundingBoxHeight}" preserveAspectRatio="none"
	class="card bordered"
>
	<linearGradient id="gradient" x1="0" x2="0" y1="-1" y2="1" gradientUnits="userSpaceOnUse">
		<stop offset="50%" stop-color="rgb(var(--stone-player-1))" />
		<stop offset="50%" stop-color="rgb(var(--stone-player-2))" />
	</linearGradient>

	<!-- zero line -->
	<line
		x1="0"
		y1="0"
		x2={values.length+1}
		y2="0"
		stroke="var(--border)"
		stroke-width="2"
		vector-effect="non-scaling-stroke"
	/>

	<!-- grid lines -->
	<path
		d={values.slice(2).map((_, i) => `M${i+1},${min}V${max}`).join(' ')}
		fill="none"
		stroke="var(--border)"
		stroke-width="2"
		vector-effect="non-scaling-stroke"
	/>

	<!-- graph fill -->
	<path
		d={'M0,0 ' + values
			.map((value, i) => `L${i},${-value}`)
			.join(' ') + 'V0'}
		fill="url(#gradient)"
		opacity="0.2"
	/>

	<!-- graph border -->
	<path
		d={values
			.map((value, i) => `${i===0?'M':'L'}${i},${-value}`)
			.join(' ')}
		fill="none"
		stroke="url(#gradient)"
		stroke-width="2"
		vector-effect="non-scaling-stroke"
	/>
</svg>

<style>
	svg {
		height: 8rem;
		padding: 0;
	}
</style>
