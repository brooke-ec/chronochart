<script lang="ts">
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

<button use:melt={$trigger} class="link menu">Timeline</button>
<div use:melt={$menu} class="dropdown">
	<button use:item class="option">New Timeline...</button>
</div>

<style lang="scss">
	@use "./style.scss";
</style>
