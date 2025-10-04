<script lang="ts">
	import { faFileCirclePlus, faFolderOpen } from "@fortawesome/free-solid-svg-icons";
	import { newProject, openProject } from "$lib";
	import { version } from "$app/environment";
	import Fa from "svelte-fa";
</script>

<main>
	<img src="/logo.svg" alt="Chronochart Logo" draggable="false" />
	<h1>chronochart</h1>
	<div class="controls">
		<button class="button" onclick={newProject}>
			<Fa icon={faFileCirclePlus} size="2x" />
			New Project
		</button>
		<button class="button" onclick={openProject}>
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
		flex-grow: 1;
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
