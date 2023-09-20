<script lang="ts">
	import { createTooltip, melt } from '@melt-ui/svelte';
	import { fade } from 'svelte/transition';
	import { Info } from 'lucide-svelte';

	const {
		elements: { trigger, content, arrow },
		states: { open }
	} = createTooltip({
		positioning: {
			placement: 'top'
		},
		openDelay: 500,
		closeOnPointerDown: false,
		forceVisible: true
	});
</script>

<button
	class="text-neutral-400 transition-colors rounded-full bg-transparent hover:text-neutral-600"
	type="button"
	use:melt={$trigger}
	aria-label="Add"
>
	<Info class="square-4" aria-label="plus" />
</button>

{#if $open}
	<div
		use:melt={$content}
		transition:fade={{ duration: 100 }}
		class="z-10 p-3 text-white rounded-lg bg-neutral-900 shadow"
	>
		<div use:melt={$arrow} />
		<slot />
	</div>
{/if}
