<script lang="ts">
	let { event } = $props<{ event: { title: string } }>();
</script>

<div class="container">
	<div class="title">
		{event.title}
	</div>
	<p>Lorem ipsum</p>
</div>

<style>
	.container {
		background-color: var(--1bg);
		border-radius: 10px;
		width: fit-content;
		overflow: hidden;
	}

	.title {
		padding: 10px;
		background-color: var(--3a);
	}

	p {
		padding: 10px;
		font-size: 0.85rem;
	}
</style>
