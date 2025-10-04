<script lang="ts">
	import Titlebar from "$lib/component/titlebar/Titlebar.svelte";
	import Toaster from "$lib/component/toast/Toaster.svelte";
	import Dialog from "$lib/component/dialog/Dialog.svelte";

	let { children } = $props();
</script>

<div class="container">
	<Titlebar />
	{@render children()}
</div>

<Dialog />
<Toaster />

<style lang="scss">
	.container {
		flex-direction: column;
		display: flex;
		height: 100vh;
	}
</style>
