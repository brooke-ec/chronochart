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

	let draggable: true | null = true;
	function updateDraggable(e: MouseEvent) {
		const right = window.innerWidth - e.x;
		draggable = e.y >= 5 && e.x >= 5 && right > 5 ? true : null;
	}
</script>

<div
	class="container"
	role="application"
	use:melt={$menubar}
	on:mousemove={updateDraggable}
	data-tauri-drag-region={draggable}
>
	<img src="/logo.svg" alt="Chronochart Logo" draggable="false" />
	<ProjectMenu {createMenu} />
</div>

<style lang="scss">
	.container {
		background-color: var(--5bg);
		align-items: center;
		display: flex;
		height: 32px;
		width: 100%;
		top: 0;

		img {
			padding: 0 8px;
			height: 22px;
		}
	}
</style>
