<script lang="ts">
	import { open, save } from "@tauri-apps/api/dialog";
	import { commands } from "$lib/specta";

	let result: string | null = null;

	async function open_file() {
		const selected = await open({ filters: [{ name: "Chronochart Project", extensions: ["crc"] }] });
		if (selected !== null && !Array.isArray(selected))
			result = JSON.stringify(await commands.connect(selected, false));
	}

	async function new_file() {
		const selected = await save({ filters: [{ name: "Chronochart Project", extensions: ["crc"] }] });
		if (selected !== null && !Array.isArray(selected))
			result = JSON.stringify(await commands.connect(selected, true));
	}
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<button on:click={open_file}>Open</button>
<button on:click={new_file}>New</button>

{#if result}
	<p>{result}</p>
{/if}
