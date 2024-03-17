<script lang="ts">
	import { commands } from "$lib/specta";
	let result: string | null = null;
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<button on:click={async () => (result = JSON.stringify(await commands.connect()))}>Test</button>

{#if result}
	<p>{result}</p>
{/if}
