<script lang="ts">
	import { closeProject, newProject, openProject } from "$lib";
	import type { MenuBuilder } from "./Titlebar.svelte";
	import { melt } from "@melt-ui/svelte";
	import { page } from "$app/stores";

	interface Props {
		createMenu: MenuBuilder;
	}

	let { createMenu }: Props = $props();

	const {
		elements: { menu, item, trigger },
	} = createMenu();
</script>

<button use:melt={$trigger} class="link menu">Project</button>
<div use:melt={$menu} class="dropdown">
	<button use:item class="option" onclick={openProject}>Open Project...</button>
	<button use:item class="option" onclick={newProject}>New Project...</button>
	<hr />
	<button use:item class="option" disabled={$page.url.pathname == "/"} onclick={closeProject}>
		Close Project
	</button>
</div>

<style lang="scss">
	@import "./style.scss";
</style>
