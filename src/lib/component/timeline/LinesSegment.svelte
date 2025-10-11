<script lang="ts">
	import type { LineSegment } from ".";

	let { lines }: { lines: LineSegment[] } = $props();
	let length = $derived(Math.max(lines.length, ...lines.map((l) => (l.top ? l.top.index + 1 : 0))));
	let width = $derived(length * 4 + (length - 1) * 10);
</script>

<div class="container">
	{#each lines as line, j}
		{#if line.top}
			{@const start = 2 + line.top.index * 14}
			{@const gap = (j - line.top.index) * 14}

			<svg class="a-{line.color}" class:top={!line.top.new} {width} viewBox="0 0 {width} 20">
				<path stroke-width="4" d="M {start} 0 c 0 14 {gap} 6 {gap} 20" />
			</svg>
		{/if}

		<div class="line a-{line.color}">
			{#if !line.top}
				<div class="first"></div>
			{/if}
			{#if line.end}
				<div class="final"></div>
			{/if}
		</div>
	{/each}
</div>

<style lang="scss">
	%fade {
		content: "";
		background-color: var(--1a);
		border-radius: 4px;
		position: absolute;
		width: 4px;
	}

	.container {
		display: flex;
		gap: 10px;

		svg {
			position: absolute;
			stroke: var(--1a);
			margin-top: -20px;

			&.top {
				z-index: 1;
			}
		}
	}

	.line {
		width: 4px;
		background-color: var(--1a);

		&:has(.final) {
			border-end-start-radius: 4px;
			border-end-end-radius: 4px;
			margin-bottom: 15px;

			display: flex;
			flex-direction: column;
		}

		&:has(.first) {
			border-start-start-radius: 4px;
			border-start-end-radius: 4px;
		}

		.first {
			margin-bottom: auto;
			position: relative;

			&::before {
				@extend %fade;

				height: 7px;
				top: -9px;
			}

			&::after {
				@extend %fade;

				height: 4px;
				top: -15px;
			}
		}

		.final {
			position: relative;
			margin-top: auto;

			&::before {
				@extend %fade;

				height: 7px;
				bottom: -9px;
			}

			&::after {
				@extend %fade;

				bottom: -15px;
				height: 4px;
			}
		}
	}
</style>
