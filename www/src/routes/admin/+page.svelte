<script lang="ts">
	import { onMount } from 'svelte';
	import '../../app.css';
	import type TicketModel from '../../models/ticket';
	import { getTicketPage } from '../../services/api';
	import TokenForm from './TokenForm.svelte';
	import TicketRow from './TicketRow.svelte';

	let token: string = '';
	let tickets: TicketModel[] = [];

	onMount(() => {
		let stored_token = window.sessionStorage.getItem('token');
		if (stored_token) {
			fetchTickets(stored_token);
		}
	});

	const fetchTickets = async (token: string) => {
		const result = await getTicketPage(token, 1);
		if (!result) {
			// TODO: show error
			return;
		}
		window.sessionStorage.setItem('token', token);

		tickets = result.tickets as TicketModel[];
	};
</script>

{#if tickets.length > 0}
	<header class="py-16">
		<div class="container">
			<h1 class="my-4 text-white text-6xl font-bold text-center">My tickets</h1>
      <div class="flex gap-4">
        <a class="underline text-orange-500" href="/">Back to app</a>
        <button class="underline text-orange-500" on:click={() => window.sessionStorage.removeItem('token')}>Logout</button>
      </div>
		</div>
	</header>
	<main>
		<div class="container">
			<div class="flex flex-col gap-2">
				{#each tickets as ticket}
					<TicketRow {ticket} />
				{/each}
			</div>
		</div>
	</main>
{:else}
	<header class="py-16">
		<div class="container">
			<h1 class="my-4 text-white text-6xl font-bold text-center">Please, authenticate</h1>
		</div>
	</header>
  <main>
    <div class="container">
      <TokenForm submitCallback={fetchTickets} />
    </div>
  </main>
{/if}
