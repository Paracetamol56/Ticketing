<script lang="ts" context="module">
	export type ToastData = {
		title: string;
		description: string;
		color: string;
	};
</script>

<script lang="ts">
	import { Heart } from 'lucide-svelte';
	import Toast from './Toast.svelte';
	import Ticket from './Ticket.svelte';
	import Accordion from './Accordion.svelte';
	import '../app.css';
	import { checkStatus, getTicket } from '../services/api';
	import { onMount } from 'svelte';
	import { createToaster } from '@melt-ui/svelte';
	import { flip } from 'svelte/animate';
	import TicketForm from './TicketForm.svelte';
	import type TicketModel from '../models/ticket';
	import TicketDisplay from './TicketDisplay.svelte';

	let currentTicket: TicketModel;

	// On mount, check the api status
	onMount(async () => {
		checkStatus().then((status) => {
			if (!status) {
				addToast({
					data: {
						title: 'Error',
						description: 'The API is not available',
						color: 'red-500'
					}
				});
			}
		});
		// If a query param 'ticket' is provided, load the ticket
		const params = new URLSearchParams(window.location.search);
		const ticketId = params.get('ticket');
		if (ticketId) {
			getTicket(ticketId).then((ticket) => {
				if (ticket) {
					currentTicket = ticket;
		      console.log(ticket);
		      console.log(ticket.id);
				} else {
					addToast({
						data: {
							title: 'Warning',
							description: 'The ticket ID you provided is not valid',
							color: 'orange-500'
						}
					});
				}
			});
		}
	});

	const {
		elements,
		helpers: { addToast },
		states: { toasts },
		actions: { portal }
	} = createToaster<ToastData>();
</script>

<!-- Toast list -->
<div
	class="fixed right-0 top-0 z-50 m-4 flex flex-col items-end gap-2 md:bottom-0 md:top-auto"
	use:portal
>
	{#each $toasts as toast (toast.id)}
		<div animate:flip={{ duration: 500 }}>
			<Toast {elements} {toast} />
		</div>
	{/each}
</div>

<header class="py-16">
	<div class="container">
		<div class="flex flex-col justify-center text-white">
			<h1 class="my-4 text-6xl font-bold text-center">Welcome to my ticketing app</h1>
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
		<div class="flex justify-center items-center">
			<Ticket>
				{#if currentTicket}
					<TicketDisplay ticket={currentTicket} />
				{:else}
					<TicketForm />
				{/if}
			</Ticket>
			<Accordion
				items={[
					{
						id: 'item-1',
						title: 'What is it for?',
						description:
							"I realized a lot of people are asking for my attention, probably because I'm an extremely handsome person. This is nice but I don't have enough time to satisfy everyone. So I made this app (probably took longer to make it than to spend one hour with each person) to help me organize my time and to help you get my attention. I hope you'll enjoy it."
					},
					{
						id: 'item-2',
						title: 'How does it work?',
						description:
							"It's very straightforward. You just have to fill in the form with your name, email (the one I can reach you at) and a short description of your ticket. Submit, and you're done. You will recieve an email with a link to your ticket and I will reach out to you as soon as possible."
					},
					{
						id: 'item-3',
						title: 'I have a problem',
						description: 'Well, I think you should figure it out by yourself...'
					}
				]}
			/>
		</div>
	</div>
</main>

<footer class="w-full py-2">
	<div class="flex items-center justify-center gap-1 text-white">
		<p>App made with</p>
		<Heart class="text-orange-500" size="16" />
		<p>
			by <a class="underline text-orange-100" href="https://matheo-galuba.com/" target="_blank"
				>Matheo Galuba</a
			>
		</p>
	</div>

  <hr class="my-3 h-[1px] max-w-xs border-none bg-neutral-400 mx-auto">

	<div class="flex items-center justify-center gap-1 text-white">
		<p>
			Powered by <a class="underline text-orange-100" href="https://www.rust-lang.org">Rust</a>,
			<a class="underline text-orange-100" href="https://www.shuttle.rs">Shuttle.rs</a>,
			<a class="underline text-orange-100" href="https://www.svelte.dev">Svelte</a>
			and <a class="underline text-orange-100" href="https://www.tailwindcss.com">TailwindCSS</a>
		</p>
	</div>
	<div class="flex items-center justify-center gap-1 text-white">
		<p>
			Icons by <a class="underline text-orange-100" href="https://lucide.dev">Lucide</a>
		</p>
	</div>
	<div class="flex items-center justify-center gap-1 text-white">
		<p>
			Hosted on <a class="underline text-orange-100" href="https://www.netlify.com">Netlify</a> and
			<a class="underline text-orange-100" href="https://www.shuttle.rs">Shuttle.rs</a>
		</p>
	</div>
</footer>

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
