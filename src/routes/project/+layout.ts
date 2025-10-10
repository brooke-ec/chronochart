import { getTimelines } from "$lib/bindings";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({}) => {
	return { timelines: await getTimelines() };
};
