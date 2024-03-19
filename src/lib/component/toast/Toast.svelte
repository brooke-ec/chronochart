<script lang="ts" context="module">
	import { createProgress, melt, type Toast, type ToastsElements } from "@melt-ui/svelte";
	import { faXmark, type IconDefinition } from "@fortawesome/free-solid-svg-icons";
	import { writable } from "svelte/store";
	import { fly } from "svelte/transition";
	import Fa from "svelte-fa";
	import { onMount } from "svelte";

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
</script>

<div
	use:melt={$content(id)}
	class="toast a-{info.color}"
	in:fly={{ duration: 150, x: "100%" }}
	out:fly={{ duration: 150, x: "100%" }}
>
	<div use:melt={$progress} class="progress">
		<div style={`transform: translateX(-${(100 * ($percentage ?? 0)) / ($max ?? 1)}%)`} />
	</div>
	<div class="content">
		<h3>
			<span class="icon"><Fa icon={info.icon} size="0.95x" /></span>
			<span use:melt={$title(id)}>{info.title}</span>
			<button class="href white" use:melt={$close(id)} aria-label="close notification">
				<Fa icon={faXmark} />
			</button>
		</h3>
		<p use:melt={$description(id)}>{info.description}</p>
		{#if info.details}
			<!-- todo: pop up modal with details -->
			<button class="href details">More Details</button>
		{/if}
	</div>
</div>

<style lang="scss">
	.progress {
		background-color: var(--6bg);
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
		background-color: var(--9bg);
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

		.icon {
			color: var(--2a);
		}

		button {
			font-size: 18px;
			float: right;
		}
	}

	.details {
		margin-top: 5px;
	}
</style>
