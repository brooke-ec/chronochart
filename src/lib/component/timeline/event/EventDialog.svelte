<script lang="ts" module>
	import { faCalendarPlus } from "@fortawesome/free-solid-svg-icons";
	import { openDialog } from "$lib/component/dialog/Dialog.svelte";
	import EventDialog from "./EventDialog.svelte";

	export function open() {
		openDialog(EventDialog, { icon: faCalendarPlus, title: "Create New Event", color: "green", props: {} });
	}
</script>

<p>Hello World :D</p>
