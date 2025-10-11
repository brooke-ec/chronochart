<script lang="ts">
	import type { MenuBuilder } from "./Titlebar.svelte";
	import { melt } from "@melt-ui/svelte";
	import { page } from "$app/state";

	interface Props {
		createMenu: MenuBuilder;
	}

	let { createMenu }: Props = $props();

	const {
		elements: { menu, item, trigger },
	} = createMenu();
</script>

{#if page.url.pathname == "/"}
	<button class="link menu disabled">Timeline</button>
{:else}
	<button use:melt={$trigger} class="link menu">Timeline</button>
{/if}
<div use:melt={$menu} class="dropdown">
	<button use:item class="option">New Timeline...</button>
</div>

<style lang="scss">
	@use "./style.scss";
</style>
