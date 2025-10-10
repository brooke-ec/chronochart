<script>
	import Timeline from "$lib/component/timeline/Timeline.svelte";

	let { data } = $props();
</script>

<main>
	<Timeline events={data.events} />
</main>

<style lang="scss">
	main {
		padding: 10px 20px;
	}
</style>
