<script lang="ts" module>
	import { faSquare, faWindowRestore } from "@fortawesome/free-regular-svg-icons";
	import { faWindowMinimize, faXmark } from "@fortawesome/free-solid-svg-icons";
	import { createMenubar, melt, type MenubarBuilders } from "@melt-ui/svelte";
	import { appWindow } from "@tauri-apps/api/window";
	import ProjectMenu from "./ProjectMenu.svelte";
	import { onMount } from "svelte";
	import Fa from "svelte-fa";

	export type MenuBuilder = MenubarBuilders["createMenu"];
</script>

<script lang="ts">
	const {
		elements: { menubar },
		builders: { createMenu },
	} = createMenubar();

	let draggable: true | null = $state(true);
	function updateDraggable(e: MouseEvent) {
		const right = window.innerWidth - e.x;
		draggable = maximized || (e.y >= 5 && e.x >= 5 && right > 5) ? true : null;
	}

	let maximized: boolean = $state(false);
	const updateMaximized = async () => (maximized = await appWindow.isMaximized());
	appWindow.onResized(updateMaximized);
	onMount(updateMaximized);
</script>

<div
	class="container"
	role="application"
	use:melt={$menubar}
	onmousemove={updateDraggable}
	data-tauri-drag-region={draggable}
>
	<img src="/logo.svg" alt="Chronochart Logo" draggable="false" />
	<ProjectMenu {createMenu} />
	<div class="controls">
		<button onclick={() => appWindow.minimize()}>
			<Fa icon={faWindowMinimize} />
		</button>
		<button onclick={() => appWindow.toggleMaximize()}>
			<Fa icon={maximized ? faWindowRestore : faSquare} />
		</button>
		<button class="a-red" onclick={() => appWindow.close()}>
			<Fa icon={faXmark} size="1.15x" />
		</button>
	</div>
</div>

<style lang="scss">
	.container {
		background-color: var(--5bg);
		align-items: center;
		user-select: none;
		display: flex;
		height: 32px;
		z-index: 100;
		width: 100%;
		top: 0;

		img {
			padding: 0 8px;
			height: 22px;
		}
	}

	.controls {
		align-items: center;
		margin-left: auto;
		display: flex;
		height: 100%;

		button {
			transition: background-color 125ms;
			background-color: var(--5bg);
			text-align: center;
			height: 100%;
			width: 48px;

			&:hover {
				background-color: var(--6bg);

				&:last-child {
					background-color: var(--2a);
				}
			}
		}
	}
</style>
