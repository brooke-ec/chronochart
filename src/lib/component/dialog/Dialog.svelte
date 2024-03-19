<script lang="ts" context="module">
	import { faXmark, type IconDefinition } from "@fortawesome/free-solid-svg-icons";
	import { createDialog, melt } from "@melt-ui/svelte";
	import { fade, scale } from "svelte/transition";
	import { SvelteComponent } from "svelte";
	import { writable } from "svelte/store";
	import { quadOut } from "svelte/easing";
	import Fa from "svelte-fa";

	export type DialogProps<T extends Record<string, any>> = {
		component: typeof SvelteComponent<T>;
		icon: IconDefinition;
		title: string;
		color: string;
		props: T;
	};

	const {
		elements: { overlay, content, title, close },
		states: { open },
	} = createDialog();

	const current = writable<DialogProps<Record<string, any>>>();

	export const closeDialog = () => open.set(false);
	export function openDialog<T extends Record<string, any>>(config: DialogProps<T>) {
		if (open.get()) throw Error("There is already a dialog open");
		current.set(config);
		open.set(true);
	}
</script>

{#if $open}
	<div use:melt={$overlay} class="overlay" transition:fade={{ duration: 250, easing: quadOut }} />
	<div use:melt={$content} class="content a-{$current.color}" transition:scale={{ duration: 250, easing: quadOut }}>
		<h2>
			<button use:melt={$close} class="href white close" aria-label="close dialog">
				<Fa icon={faXmark} />
			</button>
			<span class="icon"><Fa icon={$current.icon} size="0.95x" /></span>
			<span use:melt={$title}>{$current.title}</span>
		</h2>
		<svelte:component this={$current.component} {...$current.props} />
	</div>
{/if}

<style lang="scss">
	.overlay {
		background-color: #00000080;
		backdrop-filter: blur(3px);
		position: fixed;
		height: 100vh;
		width: 100vw;
		left: 0;
		top: 0;
	}

	.content {
		transform: translate(-50%, -50%);
		background-color: var(--5bg);
		border-radius: 5px;
		width: max-content;
		min-width: 350px;
		max-width: 90vw;
		position: fixed;
		padding: 10px;
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
