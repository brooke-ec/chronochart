<script lang="ts">
	let { event, notches }: { event: { title: string }; notches: number[] } = $props();
</script>

<div style="position: relative;">
	<div class="pointer" style:--width="{Math.max(...notches)}px">
		{#each notches as notch}
			<div class="notch" style:right="{notch}px"></div>
		{/each}
	</div>

	<div class="box">
		<div class="title">
			{event.title}
		</div>
		<p>Lorem ipsum</p>
	</div>
</div>

<style>
	.pointer {
		background-color: var(--3a);
		position: absolute;
		height: 4px;
		top: 16px;

		display: flex;
		align-items: center;

		left: calc(0px - var(--width));
		width: var(--width);

		.notch {
			border: 3px solid var(--3a);
			background-color: var(--0a);
			border-radius: 50%;
			position: absolute;
			height: 10px;
			width: 10px;
		}
	}

	.box {
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
