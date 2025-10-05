<script lang="ts">
	import Event from "./Event.svelte";
	import Lines from "./Lines.svelte";
	import data from "./test.json";
</script>

<div>
	{#each data as segment}
		<div class="segment">
			<Lines lines={segment.lines} />
			<div class="event-container">
				{#each segment.events as event}
					<Event {event} notches={event.lines.map((l) => 3 + (segment.lines.length - l) * 14)} />
				{/each}
			</div>
		</div>
	{/each}
</div>

<style lang="scss">
	.segment {
		margin-top: 20px;
		position: relative;
		display: flex;
		gap: 20px;
	}

	.event-container {
		flex-direction: column;
		display: flex;
		gap: 20px;
	}
</style>
