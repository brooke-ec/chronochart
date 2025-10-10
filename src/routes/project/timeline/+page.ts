import { getEvents } from "$lib/bindings";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({}) => {
	return {
		events: await getEvents(),
	};
};
