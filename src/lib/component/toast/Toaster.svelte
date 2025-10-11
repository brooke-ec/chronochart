<script lang="ts" module>
	import { faCircleInfo, faTriangleExclamation, faCircleCheck } from "@fortawesome/free-solid-svg-icons";
	import Toast, { type ToastInfo } from "./Toast.svelte";
	import { createToaster } from "@melt-ui/svelte";
	import { flip } from "svelte/animate";

	const TYPES = {
		error: {
			icon: faTriangleExclamation,
			title: "Error",
			color: "red",
		},
		info: {
			icon: faCircleInfo,
			title: "Info",
			color: "blue",
		},
		ok: {
			icon: faCircleCheck,
			title: "Success",
			color: "green",
		},
	};

	export type ToastData = {
		type: "info" | "ok" | "error";
		details?: string[] | string;
		description: string;
	};

	const {
		helpers,
		elements,
		states: { toasts },
	} = createToaster<ToastData>({ closeDelay: 10000 });

	export const addToast = helpers.addToast;
	export const removeToast = helpers.removeToast;

	function getInfo(data: ToastData): ToastInfo {
		const type = TYPES[data.type];
		return {
			description: data.description,
			details: data.details,
			color: type.color,
			title: type.title,
			icon: type.icon,
		};
	}
</script>

<div class="container">
	{#each $toasts as toast (toast.id)}
		<div animate:flip={{ duration: 250 }}>
			<Toast {toast} {elements} info={getInfo(toast.data)} />
		</div>
	{/each}
</div>

<style lang="scss">
	.container {
		z-index: 30;
		position: fixed;
		padding: 10px;
		bottom: 0;
		right: 0;
	}
</style>
