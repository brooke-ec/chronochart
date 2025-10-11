import { addToast } from "./component/toast/Toaster.svelte";
import { open, save } from "@tauri-apps/api/dialog";
import * as bindings from "$lib/bindings";
import { goto } from "$app/navigation";
import { closeDialog } from "./component/dialog/Dialog.svelte";

const PROJECT_FILTER = { name: "Chronochart Project", extensions: ["cro"] };

Promise.prototype.display = async function displayError(description: string) {
	try {
		return await this;
	} catch (e) {
		const details = typeof e === "string" ? [e] : Array.isArray(e) ? e : undefined;
		addToast({ data: { type: "error", description, details } });
		throw e;
	}
};

async function connectFile(path: string | string[] | null, create: boolean): Promise<string[] | null> {
	if (path !== null && !Array.isArray(path)) {
		await bindings.connect(path, create);
		await goto("/project/timeline", { replaceState: true });
		closeDialog();
	}

	return null;
}

export async function openProject() {
	const selected = await open({
		filters: [PROJECT_FILTER],
		title: "Open Project",
	});

	await connectFile(selected, false).display("Failed to open project file.");
}

export async function newProject() {
	const selected = await save({
		filters: [PROJECT_FILTER],
		title: "New Project",
	});

	await connectFile(selected, true).display("Failed to create new project.");
}

export async function closeProject() {
	await bindings.disconnect().display("Failed to close project.");
	await goto("/", { replaceState: true });
	closeDialog();
}
