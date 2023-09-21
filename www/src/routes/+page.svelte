<script lang="ts">
	import { Heart } from 'lucide-svelte';
	import Toast from '../components/Toast.svelte';
	import Ticket from './Ticket.svelte';
	import Accordion from './Accordion.svelte';
	import { checkStatus, getTicket } from '../services/api';
	import { onMount } from 'svelte';
	import { createToaster } from '@melt-ui/svelte';
	import { flip } from 'svelte/animate';
	import TicketForm from './TicketForm.svelte';
	import type TicketModel from '../models/ticket';
	import TicketDisplay from './TicketDisplay.svelte';
	import { addToast } from './+layout.svelte';

	// make this data reactive
	let currentTicket: TicketModel|null = null;
	const updateTicket = (ticket: TicketModel) => {
		currentTicket = ticket;
	};
	const resetTicket = () => {
		// Reset the url to /
		history.pushState({}, '', '/');
		currentTicket = null;
	};

	const loadTicket = async (ticketId: string) => {
		const ticket = await getTicket(ticketId);
		getTicket(ticketId).then((ticket) => {
			if (ticket) {
				currentTicket = ticket;
			} else {
				addToast({
					data: {
						title: 'Warning',
						description: 'The ticket ID you provided is not valid',
						color: 'bg-orange-500'
					}
				});
			}
		});
	};

	// On mount, check the api status
	onMount(async () => {
		// If a query param 'ticket' is provided, load the ticket
		const params = new URLSearchParams(window.location.search);
		const ticketId = params.get('ticket');
		if (ticketId) {
			loadTicket(ticketId);
		}
	});
</script>

<header class="py-16">
	<div class="container">
		<div class="flex flex-col justify-center text-white">
			<h1 class="my-4 text-5xl md:text-6xl font-bold text-center">Welcome to my ticketing app</h1>
			<div class="mx-auto">
				<p class="my-4 text-lg text-center">
					My favorite things in life don't cost any money.<br />It's really clear that the most
					precious resource we all have is time.
				</p>
				<p class="text-orange-500 opacity-75 text-md text-right">- Steve Jobs</p>
			</div>
		</div>
	</div>
</header>

<main class="py-20 w-full h-full">
	<div class="container">
		<div class="flex justify-center flex-col md:flex-row items-center gap-10">
			<Ticket>
				{#if currentTicket}
					<TicketDisplay ticket={currentTicket} resetCallback={resetTicket} />
				{:else}
					<TicketForm successCallback={updateTicket} />
				{/if}
			</Ticket>
			<Accordion
				items={[
					{
						id: 'item-1',
						title: 'What is it for?',
						description:
							"Hi, I'm Mathéo (aka Bob) and I realized a lot of people are asking for my attention, probably because I'm an amazing person. This is nice but I don't have enough time to satisfy everyone. So I made this app (probably took longer to make than to spend one hour with each person) to help me organize my time and to help you get in touch with me. I hope you'll enjoy it."
					},
					{
						id: 'item-2',
						title: 'How does it work?',
						description:
							"It's very straightforward. You just have to fill in the form with your name, email (the one I can reach you at) and a short description of your ticket. Submit, and you're done. You will recieve an email with a link to your ticket and I will reach out to you as soon as possible."
					},
					{
						id: 'item-3',
						title: 'A problem ?',
						description: 'Well, first, take a ticket and then figure it out by yourself.'
					}
				]}
			/>
		</div>
	</div>
</main>

<style lang="postcss">
	main {
		position: relative;
		z-index: 0;
		background-image: radial-gradient(circle at 1px 1px, #ffffff1a 1px, transparent 0);
		background-size: 1rem 1rem;
		background-repeat: repeat;
		background-position: 0.5rem center;
	}
	main::after {
		content: '';
		position: absolute;
		z-index: -1;
		top: 0;
		left: 0;
		bottom: 0;
		right: 0;
		background: linear-gradient(
			to bottom,
			theme('colors.neutral.900') 0%,
			transparent 10%,
			transparent 90%,
			theme('colors.neutral.900') 100%
		);
	}
</style>
