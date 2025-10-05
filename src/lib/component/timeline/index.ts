export interface LineSegment {
	color: string;
	top: { line: number; new: boolean } | null;
	end: boolean;
}

export interface Segment {
	lines: LineSegment[];
	events: any[];
}
