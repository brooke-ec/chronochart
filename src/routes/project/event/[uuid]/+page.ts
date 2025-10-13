import { getEvent } from "$lib/bindings";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params: { uuid } }) => {
	return {
		event: await getEvent(uuid),
	};
};
