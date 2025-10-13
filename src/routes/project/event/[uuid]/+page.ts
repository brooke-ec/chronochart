import { getEvent } from "$lib/bindings";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params: { uuid } }) => {
	return {
		event: await getEvent(uuid).catch((e) => error(404, e)),
	};
};
