<script lang="ts" module>
	import { faExclamationCircle, faXmark, type IconDefinition } from "@fortawesome/free-solid-svg-icons";
	import { createDialog, melt } from "@melt-ui/svelte";
	import { fade, scale } from "svelte/transition";
	import { type Component } from "svelte";
	import { quadOut } from "svelte/easing";
	import Fa from "svelte-fa";

	export type DialogOptions<T extends Record<string, any>> = {
		icon: IconDefinition;
		title: string;
		color: string;
		props: T;
	};

	const {
		elements: { overlay, content, title, close },
		states: { open },
	} = createDialog({ closeOnOutsideClick: false });

	let options = $state<DialogOptions<any>>();
	let Content = $state<Component<any>>();

	export const closeDialog = () => open.set(false);
	export function openDialog<T extends Record<string, any>>(
		component: Component<T>,
		opt: DialogOptions<T> = {
			icon: faExclamationCircle,
			color: "green",
			title: "Dialog",
			props: {} as T,
		},
	) {
		if (open.get()) throw Error("There is already a dialog open");
		Content = component;
		options = opt;
		open.set(true);
	}
</script>

{#if Content && options && $open}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="overlay"
		use:melt={$overlay}
		onclick={closeDialog}
		transition:fade={{ duration: 250, easing: quadOut }}
	></div>

	<div use:melt={$content} class="content a-{options.color}" transition:scale={{ duration: 250, easing: quadOut }}>
		<h2>
			<button use:melt={$close} class="link white close" aria-label="close dialog">
				<Fa icon={faXmark} />
			</button>
			<span class="icon"><Fa icon={options.icon} size="0.95x" /></span>
			<span use:melt={$title}>{options.title}</span>
		</h2>
		<Content {...options.props} />
	</div>
{/if}

<style lang="scss">
	.overlay {
		background-color: #00000080;
		backdrop-filter: blur(3px);
		position: fixed;
		height: 100vh;
		width: 100vw;
		z-index: 10;
		left: 0;
		top: 0;
	}

	.content {
		transform: translate(-50%, -50%);
		background-color: var(--4bg);
		border-radius: 5px;
		width: max-content;
		min-width: 350px;
		max-width: 90vw;
		position: fixed;
		padding: 10px;
		z-index: 11;
		left: 50%;
		top: 50%;
	}

	h2 {
		margin-bottom: 5px;
	}

	.icon {
		color: var(--2a);
	}

	.close {
		font-size: 20px;
		margin: 0 10px;
		float: right;
	}
</style>
