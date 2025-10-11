<script lang="ts">
	import type { Event, Timeline } from "$lib/bindings";
	import LinesSegment from "./LinesSegment.svelte";
	import EventCard from "./EventCard.svelte";
	import type { Segment } from ".";

	let { events, timelines }: { events: Event[]; timelines: Timeline[] } = $props();

	let segments: Segment[] = $derived.by(() => {
		let ends = timelines.map((timeline) => ({
			event: events.findLast((e) => e.timelines.includes(timeline.uuid))?.uuid,
			timeline,
		}));

		let starts = timelines.map((timeline) => ({
			event: events.find((e) => e.timelines.includes(timeline.uuid))?.uuid,
			timeline,
		}));

		let lines: Timeline[] = [];
		let result: Segment[] = [];
		let ending: string[] = [];
		for (let event of events) {
			let starting = starts.filter((s) => s.event === event.uuid).map((s) => s.timeline);

			if (starting.length || ending.length) {
				let previous = lines.map((l) => l.uuid);
				lines = [...lines, ...starting].filter((l) => !ending.includes(l.uuid));

				ending = ends.filter((e) => e.event === event.uuid).map((e) => e.timeline.uuid);

				result.push({
					lines: lines.map((l) => {
						let index = previous.indexOf(l.uuid);
						let starts = index === -1;

						if (starts && l.parent_uuid) index = previous.indexOf(l.parent_uuid);
						let top = index === -1 ? null : { index, new: starts };

						return { color: l.color, end: ending.includes(l.uuid), top };
					}),
					events: [],
				});
			}

			result[result.length - 1].events.push({
				notches: event.timelines.map((t) => 3 + (lines.length - lines.findIndex((l) => l.uuid === t)) * 14),
				...event,
			});
		}

		return result;
	});
</script>

<div>
	{#each segments as segment}
		<div class="segment">
			<LinesSegment lines={segment.lines} />
			<div class="event-container">
				{#each segment.events as event}
					<EventCard {event} />
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
