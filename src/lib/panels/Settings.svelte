<script>
	import Switch from "$lib/components/Switch.svelte";
	import { SETTINGS } from "$lib/storage.svelte";
</script>

<Switch bind:checked={SETTINGS.sound.value} id="settings.sound">
	<span>Sound</span>
</Switch>
