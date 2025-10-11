import type { Event } from "$lib/bindings";

export interface LineSegment {
	color: string;
	top: { index: number; new: boolean } | null;
	end: boolean;
}

export type EventCard = Event & { notches: number[] };

export interface Segment {
	lines: LineSegment[];
	events: EventCard[];
}
