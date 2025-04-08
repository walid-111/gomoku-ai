<script lang="ts"></script>

{#snippet video(src: string)}
	<video src={src} onclick={(e) => e.target?.play()} onmouseover={(e) => e.target?.play()}></video>
{/snippet}

<dl>
	<dt>Pente:</dt>
	<dd>A board game where players take turns placing a stone at a time.</dd>
</dl>

<dl>
	<dt>Goal:</dt>
	<dd>Line up 5 stones or make 5 captures.</dd>
</dl>

<dl>
	<dt>Capture:</dt>
	<dd>Surround an opponent's pair of stones with your stones.</dd>
	{@render video("./videos/capture.webm")}
</dl>


<div class="card exception">
	<p>The only way to counter a winning line once created is to capture one of its stones.</p>
	{@render video("./videos/wining-line.webm")}
</div>

<dl>
	<dt>Free-threes:</dt>
	<dd>3 stones that would lead to a winning line if not blocked immediately.</dd>
	{@render video("./videos/free-three.webm")}
</dl>


<div class="card exception">
	<p>In a turn, you can't create multiple 'free-threes' except by capturing stones.</p>
	{@render video("./videos/double-three.webm")}
</div>

<small>* joseph a spoth direct</small>

<p>Created by <a href="https://github.com/matubu" target="_blank" class="link">Matubu</a> & <a href="https://github.com/walid-111" target="_blank" class="link">Walid</a></p>

<style>
	dt {
		font-weight: 900;
	}
	dt, dd {
		display: inline;
	}
	video {
		display: block;
		margin-top: 0.5rem;
		cursor: pointer;
	}
</style>
