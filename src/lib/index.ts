import { addToast } from "./component/toast/Toaster.svelte";
import { open, save } from "@tauri-apps/api/dialog";
import { commands } from "$lib/specta";

const PROJECT_FILTER = { name: "Chronochart Project", extensions: ["crc"] };

export function display_error(description: string, details?: string[] | string) {
	console.error([description, ...(details ? details : [])].join("\n• "));
	addToast({ data: { type: "error", description, details } });
}

async function connect_file(path: string | string[] | null, create: boolean): Promise<string[] | null> {
	if (path !== null && !Array.isArray(path)) {
		const result = await commands.connect(path, create);
		if (result.status == "error") return result.error;
	}

	return null;
}

export async function open_project() {
	const selected = await open({
		filters: [PROJECT_FILTER],
		title: "Open Project",
	});

	const error = await connect_file(selected, false);
	if (error) display_error("Failed to open project file", error);
}

export async function new_project() {
	const selected = await save({
		filters: [PROJECT_FILTER],
		title: "New Project",
	});

	const error = await connect_file(selected, true);
	if (error) display_error("Failed to create new project", error);
}
