<script lang="ts">
	import { createAccordion, melt } from '@melt-ui/svelte';
	import { slide } from 'svelte/transition';

	const {
		elements: { content, item, trigger, root },
		helpers: { isSelected }
	} = createAccordion({
		defaultValue: 'item-1'
	});

	let items = [];
	export { items };
</script>

<div
	class="z-10 w-[300px] max-w-full rounded-xl bg-white shadow-lg sm:w-[25rem]"
	{...$root}
>
	{#each items as { id, title, description }, i}
		<div
			use:melt={$item(id)}
			class="overflow-hidden transition-colors first:rounded-t-xl
              last:rounded-b-xl"
		>
			<h3 class="flex">
				<button
					use:melt={$trigger(id)}
					class="flex flex-1 cursor-pointer items-center justify-between
						bg-white px-5 py-5 text-lg font-medium leading-none
						text-black transition-colors hover:bg-neutral-100 focus:!ring-0
						border-t border-t-neutral-200"
				>
					{title}
				</button>
			</h3>
			{#if $isSelected(id)}
				<div
					class="overflow-hidden bg-neutral-100 text-base text-neutral-600"
					use:melt={$content(id)}
					transition:slide
				>
					<div class="px-5 py-4">
						{description}
					</div>
				</div>
			{/if}
		</div>
	{/each}
</div>

<style lang="postcss">
	.content {
		box-shadow: inset 0px 1px 0px theme('colors.neutral.300');
	}
</style>
