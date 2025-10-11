<script lang="ts">
	import { open as openDialog } from "$lib/component/timeline/event/EventDialog.svelte";
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
	<button class="link menu disabled">Event</button>
{:else}
	<button use:melt={$trigger} class="link menu">Event</button>
{/if}
<div use:melt={$menu} class="dropdown">
	<button use:item class="option" onclick={openDialog}>New Event...</button>
</div>

<style lang="scss">
	@use "./style.scss";
</style>
