<script lang="ts">
	let { description, details }: Props = $props();
	interface Props {
		description: string;
		details: string | string[];
	}
</script>

{#if Array.isArray(details)}
	<p>{description}</p>
	<ul>
		{#each details as detail}
			<li>{detail}</li>
		{/each}
	</ul>
{:else}
	Single
{/if}

<style lang="scss">
	ul {
		font-family: "DejaVu Sans Mono", monospace;
		background-color: var(--1bg);
		padding: 10px 10px 10px 30px;
		flex-direction: column;
		word-wrap: break-word;
		border-radius: 5px;
		margin-top: 15px;
		font-size: 14px;
		display: flex;
		gap: 10px;
	}
</style>
