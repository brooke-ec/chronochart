<script lang="ts" context="module">
	import { createProgress, melt, type Toast, type ToastsElements } from "@melt-ui/svelte";
	import { faXmark, type IconDefinition } from "@fortawesome/free-solid-svg-icons";
	import { openDialog } from "../dialog/Dialog.svelte";
	import ToastDialog from "./ToastDialog.svelte";
	import { removeToast } from "./Toaster.svelte";
	import { writable } from "svelte/store";
	import { fly } from "svelte/transition";
	import { quadOut } from "svelte/easing";
	import { onMount } from "svelte";
	import Fa from "svelte-fa";

	export type ToastInfo = {
		icon: IconDefinition;
		title: string;
		color: string;
		description: string;
		details?: string[] | string;
	};
</script>

<script lang="ts">
	export let info: ToastInfo;
	export let toast: Toast;
	$: ({ id, getPercentage } = toast);
	export let elements: ToastsElements;
	$: ({ content, title, description, close } = elements);

	const percentage = writable(0);
	const {
		elements: { root: progress },
		options: { max },
	} = createProgress({
		max: 100,
		value: percentage,
	});

	onMount(() => {
		let frame: number;
		const updatePercentage = () => {
			percentage.set(getPercentage());
			frame = requestAnimationFrame(updatePercentage);
		};
		frame = requestAnimationFrame(updatePercentage);

		return () => cancelAnimationFrame(frame);
	});

	function openDetails() {
		removeToast(id);
		openDialog({
			title: info.title + " Details",
			component: ToastDialog,
			color: info.color,
			icon: info.icon,
			props: {
				description: info.description,
				details: info.details,
			},
		});
	}
</script>

<div
	use:melt={$content(id)}
	class="toast a-{info.color}"
	in:fly={{ duration: 250, x: "100%", easing: quadOut }}
	out:fly={{ duration: 250, x: "100%" }}
>
	<div use:melt={$progress} class="progress">
		<div style={`transform: translateX(-${(100 * ($percentage ?? 0)) / ($max ?? 1)}%)`} />
	</div>
	<div class="content">
		<h3>
			<button class="link white close" use:melt={$close(id)} aria-label="close notification">
				<Fa icon={faXmark} />
			</button>
			<span class="icon"><Fa icon={info.icon} size="0.95x" /></span>
			<span use:melt={$title(id)}>{info.title}</span>
		</h3>
		<p use:melt={$description(id)}>{info.description}</p>
		{#if info.details}
			<button class="link details" on:click={openDetails}>More Details</button>
		{/if}
	</div>
</div>

<style lang="scss">
	.progress {
		background-color: var(--7bg);
		width: 100%;
		height: 5px;

		div {
			background-color: var(--2a);
			height: inherit;
			width: inherit;
		}
	}

	.toast {
		box-shadow: 2px 2px 5px #00000080;
		background-color: var(--6bg);
		border-radius: 5px;
		overflow: hidden;
		margin-top: 5px;
		width: 300px;
	}

	.content {
		padding: 5px 10px;
		padding-top: 1px;
	}

	h3 {
		margin-bottom: 5px;
	}

	.icon {
		color: var(--2a);
	}

	.close {
		font-size: 18px;
		float: right;
	}

	.details {
		margin-top: 5px;
	}
</style>
