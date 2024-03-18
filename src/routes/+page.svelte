<script lang="ts">
	import { faFileCirclePlus, faFolderOpen } from "@fortawesome/free-solid-svg-icons";
	import { open, save } from "@tauri-apps/api/dialog";
	import { version } from "$app/environment";
	import { commands } from "$lib/specta";
	import Fa from "svelte-fa";

	async function open_file() {
		const selected = await open({ filters: [{ name: "Chronochart Project", extensions: ["crc"] }] });
		if (selected !== null && !Array.isArray(selected)) await commands.connect(selected, false);
	}

	async function new_file() {
		const selected = await save({ filters: [{ name: "Chronochart Project", extensions: ["crc"] }] });
		if (selected !== null && !Array.isArray(selected)) await commands.connect(selected, true);
	}
</script>

<main>
	<img src="logo.svg" alt="Chronochart Logo" draggable="false" />
	<h1>chronochart</h1>
	<div class="controls">
		<button on:click={new_file}>
			<Fa icon={faFileCirclePlus} size="2x" />
			New Project
		</button>
		<button on:click={open_file}>
			<Fa icon={faFolderOpen} size="2x" />
			Open Project
		</button>
	</div>
	<footer class="minor">v{version}</footer>
</main>

<style lang="scss">
	main {
		justify-content: center;
		flex-direction: column;
		align-items: center;
		padding-top: 25px;
		user-select: none;
		display: flex;
		height: 100vh;
		gap: 5px;

		img {
			height: 125px;
		}
	}

	.controls {
		grid-template-columns: repeat(2, 1fr);
		margin-top: 60px;
		display: grid;
		gap: 20px;

		button {
			flex-direction: column;
			align-items: center;
			display: flex;
		}
	}

	footer {
		background-color: var(--1bg);
		margin-top: auto;
		padding: 5px;
		width: 100%;
	}
</style>
