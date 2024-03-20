<script lang="ts">
	import { close_project, new_project as newProject, open_project as openProject } from "$lib";
	import type { MenuBuilder } from "./MenuBar.svelte";
	import { melt } from "@melt-ui/svelte";

	export let createMenu: MenuBuilder;

	const {
		elements: { menu, item, trigger },
	} = createMenu();
</script>

<button use:melt={$trigger} class="link menu">Project</button>
<div use:melt={$menu} class="dropdown">
	<button use:item class="option" on:click={openProject}>Open Project</button>
	<button use:item class="option" on:click={newProject}>New Project</button>
	<hr />
	<button use:item class="option" on:click={close_project}>Close Project</button>
</div>

<style lang="scss">
	@import "./style.scss";
</style>
