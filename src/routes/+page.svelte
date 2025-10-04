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
		<button class="link" onclick={newProject}>
			<Fa icon={faFileCirclePlus} />
			New Project
		</button>
		<button class="link" onclick={openProject}>
			<Fa icon={faFolderOpen} />
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
		padding-top: max(25px, 10vh);
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
		margin-top: 10px;
		display: grid;
		gap: 10px 10vw;

		button {
			flex-direction: row;
			align-items: center;
			display: flex;
			gap: 10px;
		}
	}

	footer {
		background-color: var(--1bg);
		margin-top: auto;
		padding: 5px;
		width: 100%;
	}
</style>
