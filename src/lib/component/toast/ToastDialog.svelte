<script lang="ts">
	export let description: string;
	export let details: string | string[];
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
		background-color: var(--1bg);
		padding: 10px 10px 10px 30px;
		flex-direction: column;
		border-radius: 5px;
		margin-top: 15px;
		display: flex;
		gap: 10px;
	}
</style>
