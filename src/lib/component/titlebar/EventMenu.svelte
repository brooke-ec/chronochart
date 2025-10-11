<script lang="ts">
	import { open as openDialog } from "$lib/component/timeline/event/EventDialog.svelte";
	import type { MenuBuilder } from "./Titlebar.svelte";
	import { melt } from "@melt-ui/svelte";

	interface Props {
		createMenu: MenuBuilder;
	}

	let { createMenu }: Props = $props();

	const {
		elements: { menu, item, trigger },
	} = createMenu();
</script>

<button use:melt={$trigger} class="link menu">Event</button>
<div use:melt={$menu} class="dropdown">
	<button use:item class="option" onclick={openDialog}>New Event...</button>
</div>

<style lang="scss">
	@use "./style.scss";
</style>
