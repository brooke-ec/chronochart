<script lang="ts">
	import Event from "./Event.svelte";
	import data from "./test.json";
</script>

<div>
	{#each data as section, i}
		{@const length = section.lines.length}
		{@const width = length * 4 + (length - 1) * 10}

		<div class="section">
			<div class="line-container">
				{#each section.lines as line, j}
					{@const final = !data[i + 1]?.lines.some((l) => l.connection == j)}

					{#if line.connection}
						{@const start = 2 + line.connection * 14}
						{@const gap = (j - line.connection) * 14}
						{@const top = line.color == data[i - 1]?.lines[line.connection].color}

						<svg class="a-{line.color}" class:top {width} viewBox="0 0 {width} 20">
							<path stroke-width="4" d="M {start} 0 c 0 14 {gap} 6 {gap} 20" />
						</svg>
					{/if}

					<div class="line a-{line.color}">
						{#if !line.connection}
							<div class="first"></div>
						{/if}
						{#if final}
							<div class="final"></div>
						{/if}
					</div>
				{/each}
			</div>
			<div class="event-container">
				{#each section.events as event}
					{@const pointer = 22 + (length - 1 - Math.min(...event.lines)) * 14}
					<div style="position: relative;">
						<div class="pointer" style:--width="{pointer}px">
							{#each event.lines as line}
								<div class="lump" style:--index={length - line}></div>
							{/each}
						</div>
						<Event {event} />
					</div>
				{/each}
			</div>
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

	.section {
		margin-top: 20px;
		position: relative;
		display: flex;
		gap: 20px;
	}

	.line-container {
		justify-content: center;
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

	.event-container {
		flex-direction: column;
		display: flex;
		gap: 20px;
	}

	.pointer {
		background-color: var(--3a);
		position: absolute;
		height: 4px;
		top: 16px;

		display: flex;
		align-items: center;

		left: calc(0px - var(--width));
		width: var(--width);

		.lump {
			right: calc(3px + var(--index) * 14px);
			border: 3px solid var(--3a);
			background-color: var(--0a);
			border-radius: 2px;
			position: absolute;
			height: 10px;
			width: 10px;
		}
	}
</style>
