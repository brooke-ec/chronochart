<script lang="ts">
	import type { Event, Timeline } from "$lib/bindings";
	import LinesSegment from "./LinesSegment.svelte";
	import EventCard from "./EventCard.svelte";
	import type { LineSegment, Segment } from ".";
	// import data from "./test.json";

	let { events, timelines }: { events: Event[]; timelines: Timeline[] } = $props();

	let segments: Segment[] = $derived.by(() => {
		let ends = timelines.map((t) => events.findLastIndex((e) => e.timelines.includes(t.uuid)));

		let lines: string[] = [];
		let result: Segment[] = [];
		for (let event of events) {
			if (event.timelines.some((t) => !lines.includes(t))) {
				lines = [...lines, ...event.timelines.filter((t) => !lines.includes(t))];
				result.push({
					events: [],
					lines: lines.map((uuid) => {
						let timeline = timelines.find((t) => t.uuid === uuid);
						if (!timeline) throw new Error(`Timeline '${uuid}' not found`);
						return { color: timeline.color, top: null, end: false };
					}),
				});
			}

			let segment = result[result.length - 1];
			segment.events.push({ ...event, connections: event.timelines.map((t) => lines.indexOf(t)) });
		}
		// [{ lines: [], events: events.map((e) => ({ ...e, lines: [] })) }]
		return result;
	});
</script>

<div>
	{#each segments as segment}
		<div class="segment">
			<LinesSegment lines={segment.lines} />
			<div class="event-container">
				{#each segment.events as event}
					<EventCard {event} notches={event.connections.map((l) => 3 + (segment.lines.length - l) * 14)} />
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
