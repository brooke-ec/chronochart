<script lang="ts">
	import MenuBar from "$lib/component/titlebar/Titlebar.svelte";
	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();
</script>

{@render children?.()}
