<script lang="ts">
	import type { DebuggingSession } from "$lib/ai/debug.svelte";

	const COLORS = ['red', 'blue', 'red'];

	let { debuggingSession }: {
		debuggingSession: DebuggingSession
	} = $props();
</script>

<div class="container">
	<div class="time">
		{#each debuggingSession.timings as { startMs, durationMs }, i}
			<div
				style:background="rgb(var(--{COLORS[i % COLORS.length]}))"
				style:left="{startMs / debuggingSession.durationMs * 100}%"
				style:width="{durationMs / debuggingSession.durationMs * 100}%"
			></div>
		{/each}
	</div>

	<div class="logs">
		{#each debuggingSession.timings as { name, durationMs }, i}
			<div>
				<div class="color" style:background="rgb(var(--{COLORS[i % COLORS.length]}))"></div>
				{name}:
				{durationMs?.toFixed(1)}ms
			</div>
		{/each}
		<div>
			<div class="color" style:background="var(--layer-1)"></div>
			total:
			{debuggingSession.durationMs.toFixed(1)}ms
		</div>
	</div>
</div>

<style>
	.container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.time {
		border-radius: 0.5rem;
		height: 2rem;
		overflow: hidden;
		position: relative;
		background: var(--layer-1);

		& > div {
			height: 100%;
			position: absolute;
			opacity: 0.5;
			border: solid var(--border);
			border-width: 0 1px;
		}
	}

	.color {
		width: 1rem;
		height: 1rem;
		border-radius: 0.25rem;
		font-family: monospace;
		display: grid;
		place-items: center;
		line-height: 1;
	}

	.logs {
		display: flex;
		flex-direction: column;

		& > div {
			display: flex;
			align-items: center;
			gap: 0.5rem;
		}
	}
</style>
