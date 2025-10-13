<script lang="ts">
	import { faTriangleExclamation } from "@fortawesome/free-solid-svg-icons";
	import { page } from "$app/state";
	import Fa from "svelte-fa";
</script>

<main>
	<h1><span class="a-red" style="color: var(--2a);"><Fa icon={faTriangleExclamation} /></span> Unexpected Error</h1>
	{#if Array.isArray(page.error)}
		<ul class="details">
			{#each page.error as detail}
				<li>{detail}</li>
			{/each}
		</ul>
	{:else}
		<p class="details">
			{#if page.error?.message}
				{page.error.message}
			{:else}
				{JSON.stringify(page.error)}
			{/if}
		</p>
	{/if}
</main>

<style lang="scss">
	main {
		padding: 10px;
	}

	.details {
		font-family: "DejaVu Sans Mono", monospace;
		background-color: var(--1bg);
		flex-direction: column;
		word-wrap: break-word;
		border-radius: 5px;
		margin-top: 15px;
		font-size: 14px;
		display: flex;
		padding: 10px;
		gap: 10px;
	}

	ul.details {
		padding: 10px 10px 10px 30px;
	}
</style>
