<script lang="ts">
	import { onMount } from 'svelte';
	import '../../app.css';
	import type TicketModel from '$lib/models/ticket';
	import { getTicketPage } from '$lib/services/api';
	import TokenForm from './TokenForm.svelte';
	import TicketRow from './TicketRow.svelte';
	import { addToast } from '../+layout.svelte';
	import Pagination from './Pagination.svelte';

	let tickets: TicketModel[] = [];
	let page: number = 1;
	let count: number = 1;
	const handlePageChange = (newPage: number) => {
		page = newPage;
		fetchTickets(window.sessionStorage.getItem('token')!);
	};
	const revokeToken = () => {
		window.sessionStorage.removeItem('token');
		addToast({
			data: {
				title: 'Success',
				description: 'Token revoked',
				color: 'bg-green-500'
			}
		});
	};
	const fetchTickets = async (token: string) => {
		const result = await getTicketPage(token, page);
		if (!result) {
			addToast({
				data: {
					title: 'Error',
					description: 'An error occurred while fetching tickets',
					color: 'bg-red-500'
				}
			});
			return;
		}
		window.sessionStorage.setItem('token', token);
		tickets = result.items as TicketModel[];
		count = result.total_items;
	};

	onMount(() => {
		let stored_token = window.sessionStorage.getItem('token');
		if (stored_token) {
			fetchTickets(stored_token);
		}
	});
</script>

{#if tickets.length > 0}
	<header class="py-16">
		<div class="container">
			<h1 class="my-4 text-white text-6xl font-bold text-center">My tickets</h1>
			<div class="flex gap-4">
				<a class="underline text-orange-500" href="/">Back to app</a>
				<button class="underline text-orange-500" on:click={revokeToken}>Forget token</button>
			</div>
		</div>
	</header>
	<main>
		<div class="container">
			<div class="flex flex-col gap-2">
				{#each tickets as ticket}
					<TicketRow {ticket} />
				{/each}
				<Pagination changePageCallback={handlePageChange} {count} />
			</div>
		</div>
	</main>
{:else}
	<header class="py-16">
		<div class="container">
			<h1 class="my-4 text-white text-6xl font-bold text-center">Please, authenticate</h1>
		</div>
	</header>
	<main class="mb-32">
		<div class="container">
			<TokenForm submitCallback={fetchTickets} />
		</div>
	</main>
{/if}
