<script lang="ts">
	import { onMount, tick } from 'svelte';
	import type TicketModel from '../models/ticket';
	import JsBarcode from 'jsbarcode';
	import { CheckCircle2, Circle, CircleDot, CircleDotDashed } from 'lucide-svelte';

	onMount(() => {
		JsBarcode('#barcode', ticket.uuid, {
			format: 'CODE128',
			displayValue: false,
			height: 100,
			width: 2,
			margin: 0
		});
	});

	export let ticket: TicketModel;
	export let resetCallback: () => void;
</script>

<div class="h-full flex flex-col justify-start font-mono text-neutral-800">
	<svg id="barcode" class="w-full h-fit" />
	<h3 class="mt-4 text-2xl font-bold text-orange-500 text-center">Your ticket</h3>
	<p class="text-9xl font-bold text-center">{ticket.number}</p>

	<div class="flex justify-between items-start">
		<p class="text-base me-2">Status</p>
		<div class="flex justify-center items-center gap-2">
			{#if ticket.status === 'open'}
				<CircleDot class="stroke-orange-500" size="16" />
			{:else if ticket.status === 'pending'}
				<CircleDotDashed class="stroke-red-500" size="16" />
			{:else if ticket.status === 'closed'}
				<CheckCircle2 class="stroke-green-500" size="16" />
			{:else}
				<Circle class="stroke-gray-500" size="16" />
			{/if}
			<p class="text-base font-bold uppercase">{ticket.status}</p>
		</div>
	</div>
	<hr class="my-3 h-[2px] w-full border-none bg-neutral-800" />
	<div class="flex justify-between items-start">
		<p class="text-sm me-2">Name</p>
		<p class="text-sm text-end text-ellipsis overflow-hidden">
			{ticket.name}
		</p>
	</div>
	<div class="flex justify-between items-start">
		<p class="text-sm me-2">Email</p>
		<p class="text-sm text-end text-ellipsisoverflow-hidden">
			{ticket.email}
		</p>
	</div>
	<div class="flex justify-between items-start">
		<p class="text-sm me-2">Message</p>
		<p class="h-20 text-sm text-end text-ellipsis overflow-hidden">
			{ticket.message}
		</p>
	</div>

	<button class="mt-auto text-xs text-orange-800 text-center" on:click={() => resetCallback()}>
		Get another ticket !
	</button>
</div>
