import { addToast } from "./component/toast/Toaster.svelte";
import { open, save } from "@tauri-apps/api/dialog";
import { commands } from "$lib/specta";
import { goto } from "$app/navigation";

const PROJECT_FILTER = { name: "Chronochart Project", extensions: ["cro"] };

export function displayError(description: string, details?: string[] | string) {
	console.error([description, ...(details ? details : [])].join("\n• "));
	addToast({ data: { type: "error", description, details } });
}

async function connectFile(path: string | string[] | null, create: boolean): Promise<string[] | null> {
	if (path !== null && !Array.isArray(path)) {
		const result = await commands.connect(path, create);
		if (result.status == "error") return result.error;
		await goto("/project/timeline", { replaceState: true });
	}

	return null;
}

export async function openProject() {
	const selected = await open({
		filters: [PROJECT_FILTER],
		title: "Open Project",
	});

	const error = await connectFile(selected, false);
	if (error) displayError("Failed to open project file.", error);
}

export async function newProject() {
	const selected = await save({
		filters: [PROJECT_FILTER],
		title: "New Project",
	});

	const error = await connectFile(selected, true);
	if (error) displayError("Failed to create new project.", error);
}

export async function closeProject() {
	const result = await commands.disconnect();
	if (result.status == "error") displayError("Failed to close project.");
	else await goto("/", { replaceState: true });
}
