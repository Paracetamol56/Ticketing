<script lang="ts">
	import type TicketModel from '../../models/ticket';
	import { ArrowUpRight, CheckCircle2, Circle, CircleDot, CircleDotDashed } from 'lucide-svelte';
	import { createSelect, melt, type CreateSelectProps, createDialog } from '@melt-ui/svelte';
	import { Check, ChevronDown } from 'lucide-svelte';
	import { updateTicket } from '../../services/api';
	import { addToast } from '../+layout.svelte';
	import Dialog from './Dialog.svelte';

	export let ticket: TicketModel;

	let new_note: string = ticket.note || '';
	let new_status: string = ticket.status;

	const handleSave = async (notify: boolean) => {
		let token = window.sessionStorage.getItem('token');
		let result: TicketModel | null = await updateTicket(
			token!,
			ticket.id,
			new_status,
			new_note,
			notify
		);
		if (!result) {
			addToast({
				data: {
					title: 'Error',
					description: 'An error occurred while updating the ticket',
					color: 'bg-red-500'
				}
			});
			return;
		}
		ticket = result!;
		addToast({
			data: {
				title: 'Success',
				description: 'The ticket has been updated',
				color: 'bg-green-500'
			}
		});
	};
	const handleCancel = () => {
		new_note = ticket.note || '';
		new_status = ticket.status;
		selected.set({ value: ticket.status, label: ticket.status });
	};
	const humanReadableDate = (date: string) => {
		return new Date(date).toLocaleString();
	};

	const options = ['open', 'pending', 'closed'];
	const handleStatusChange: CreateSelectProps['onSelectedChange'] = ({ curr, next }) => {
		if (next) {
			new_status = next.value!.toString();
		}
		return next;
	};

	const {
		elements: { trigger, menu, option },
		states: { selectedLabel, open, selected },
		helpers: { isSelected }
	} = createSelect({
		onSelectedChange: handleStatusChange,
		forceVisible: true,
		positioning: {
			placement: 'bottom',
			fitViewport: true,
			sameWidth: true
		}
	});

	selected.set({ value: ticket.status, label: ticket.status });
</script>

<div class="w-full bg-neutral-100 rounded-md flex gap-2 p-2">
	<div class="flex flex-col items-center w-24">
		{#if new_status === 'open'}
			<CircleDot class="m-8 stroke-orange-500" size="32" />
		{:else if new_status === 'pending'}
			<CircleDotDashed class="m-8 stroke-red-500" size="32" />
		{:else if new_status === 'closed'}
			<CheckCircle2 class="m-8 stroke-green-500" size="32" />
		{:else}
			<Circle class="m-8 stroke-gray-500" size="32" />
		{/if}
		<p><strong>Number</strong></p>
		<p class="text-6xl text-center">{ticket.number}</p>
	</div>
	<div class="flex-grow flex flex-col">
		<div class="flex items-center gap-2">
			<p class="truncate"><strong>ID:</strong> {ticket.uuid}</p>
			<a
				class="text-neutral-400 transition-colors hover:text-neutral-600"
				href={`/?ticket=${ticket.uuid}`}
				target="_blank"
				rel="noopener noreferrer"
			>
				<ArrowUpRight size="18" />
			</a>
		</div>
		<p><strong>Name:</strong> {ticket.name}</p>
		<p>
			<strong>Email:</strong>
			<a class="underline text-orange-800" href={`mailto:${ticket.email}`}>{ticket.email}</a>
		</p>
		<p><strong>Message:</strong> {ticket.message}</p>
		<p><strong>Issued at:</strong> {humanReadableDate(ticket.created_at)}</p>
		{#if ticket.updated_at}
			<p><strong>Updated at:</strong> {humanReadableDate(ticket.updated_at)}</p>
		{/if}
		{#if ticket.closed_at}
			<p><strong>Closed at:</strong> {humanReadableDate(ticket.closed_at)}</p>
		{/if}
		<p><strong>Note:</strong></p>
		<textarea
			bind:value={new_note}
			class="h-20 flex-grow rounded-md border resize-none border-neutral-200 shadow-lg p-2 focus:ring-2 focus:ring-orange-500 focus:outline-none"
			id="message"
			placeholder="Private note"
		/>
		<p><strong>Status:</strong></p>
		<div>
			<button
				class="flex h-10 min-w-[220px] items-center justify-between rounded-lg bg-white px-3 py-2
      text-magnum-700 shadow-lg transition-opacity hover:opacity-90"
				use:melt={$trigger}
				on:m-keydown={(e) => {
					e.preventDefault(); // Cancel default builder behabiour
					e.detail.originalEvent.preventDefault(); // Cancel page scroll

					const { key } = e.detail.originalEvent;

					if (!['ArrowDown', 'ArrowUp', 'Space', 'Enter'].includes(key)) return;
					const allOptions = Object.values(options).flat();
					const index = allOptions.indexOf(`${$selectedLabel}`);

					if (key === 'ArrowDown') {
						const nextIndex = index + 1;
						const nextOption = allOptions[nextIndex] || allOptions[0];
						selected.set({ value: nextOption, label: nextOption });
					} else if (key === 'ArrowUp') {
						const prevIndex = index - 1;
						const prevOption = allOptions[prevIndex] || allOptions[allOptions.length - 1];
						selected.set({ value: prevOption, label: prevOption });
					} else {
						open.set(true);
					}
				}}
				aria-label="Select status"
			>
				<p class="capitalize">{$selectedLabel}</p>
				<ChevronDown class="square-5" />
			</button>
			{#if $open}
				<div
					class="z-10 flex max-h-[300px] flex-col
        overflow-y-auto rounded-lg bg-white p-1
        shadow focus:!ring-0"
					use:melt={$menu}
				>
					{#each options as item}
						<div
							class="relative cursor-pointer rounded-lg py-1 pl-8 pr-4 text-neutral-800
              focus:z-10 focus:text-orange-700
              data-[highlighted]:bg-orange-50 data-[selected]:bg-orange-100
              data-[highlighted]:text-orange-900 data-[selected]:text-orange-900"
							use:melt={$option({ value: item, label: item })}
						>
							<div class="check {$isSelected(item) ? 'block' : 'hidden'}">
								<Check class="square-4" />
							</div>
							<p class="capitalize">{item}</p>
						</div>
					{/each}
				</div>
			{/if}
		</div>
		<div class="my-2 flex justify-end gap-2">
			<button
				class="rounded-md border-2 border-orange-500 px-4 py-1.5 font-medium text-orange-500 shadow-lg bg-transparent hover:bg-orange-50 active:bg-orange-100"
				on:click={handleCancel}
			>
				Cancel
			</button>
			<Dialog saveCallback={handleSave} />
		</div>
	</div>
</div>

<style lang="postcss">
	.check {
		position: absolute;
		left: theme(spacing.2);
		top: 50%;
		z-index: theme(zIndex.20);
		translate: 0 calc(-50% + 1px);
		color: theme(colors.orange.500);
	}
</style>
