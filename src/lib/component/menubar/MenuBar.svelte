<script lang="ts" context="module">
	import { createMenubar, melt, type MenubarBuilders } from "@melt-ui/svelte";
	import ProjectMenu from "./ProjectMenu.svelte";

	export type MenuBuilder = MenubarBuilders["createMenu"];
</script>

<script lang="ts">
	const {
		elements: { menubar },
		builders: { createMenu },
	} = createMenubar();
</script>

<div use:melt={$menubar} class="container">
	<ProjectMenu {createMenu} />
</div>

<style lang="scss">
	.container {
		background-color: var(--5bg);
		width: 100%;
		top: 0;
	}
</style>
