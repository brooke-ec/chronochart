<script lang="ts">
	let { description, details }: Props = $props();
	interface Props {
		description: string;
		details: string | string[];
	}
</script>

<p>{description}</p>
{#if Array.isArray(details)}
	<ul class="details">
		{#each details as detail}
			<li>{detail}</li>
		{/each}
	</ul>
{:else}
	<p class="details">{JSON.stringify(details)}</p>
{/if}

<style lang="scss">
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
