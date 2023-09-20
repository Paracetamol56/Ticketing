<script lang="ts">
	import { createLabel, melt } from '@melt-ui/svelte';
	import { AtSign, MessageSquare, User } from 'lucide-svelte';
	import Tooltip from './Tooltip.svelte';
	import { issueTicket } from '../services/api';
	import { goto } from '$app/navigation';
	import { addToast } from './+layout.svelte';

	const {
		elements: { root }
	} = createLabel();

	let name = '';
	let email = '';
	let message = '';

	// TODO: Reactive form validation

	function handleSubmit(event: Event) {
		event.preventDefault();
		issueTicket(name, email, message).then((ticket) => {
			if (!ticket) {
				addToast({
					data: {
						title: 'Internal Error',
						description: 'An error occured while creating your ticket, please try again later',
						color: 'bg-red-500'
					}
				});
				return;
			}
			addToast({
				data: {
					title: 'Success',
					description: 'Your ticket has been successfully created, check your mailbox !',
					color: 'bg-green-500'
				}
			});
			const id = ticket.id;
			goto(`/?ticket=${id}`);
		});
	}
</script>

<form on:submit={handleSubmit}>
	<div class="h-full flex flex-col justify-start">
		<h3 class="text-2xl font-bold text-center">Get your ticket</h3>
		<label
			use:melt={$root}
			class="my-4 flex text-md font-semibold leading-none text-orange-500"
			for="name"
			data-melt-part="root"
		>
			<User size="16" strokeWidth="3" />
			<span class="ms-1">Name</span>
		</label>
		<input
			bind:value={name}
			class="h-10 flex-grow rounded-md border border-neutral-200 p-2 focus:ring-2 focus:ring-orange-500 focus:outline-none"
			type="text"
			id="name"
			placeholder="John Doe"
		/>
		<label
			use:melt={$root}
			class="my-4 flex text-md font-semibold leading-none text-orange-500"
			for="email"
			data-melt-part="root"
		>
			<AtSign size="16" strokeWidth="3" />
			<span class="ms-1">Email</span>
		</label>
		<input
			bind:value={email}
			class="h-10 flex-grow rounded-md border border-neutral-200 p-2 focus:ring-2 focus:ring-orange-500 focus:outline-none"
			type="email"
			id="email"
			placeholder="example@mail.com"
		/>
		<label
			use:melt={$root}
			class="my-4 flex text-md font-semibold leading-none text-orange-500"
			for="email"
			data-melt-part="root"
		>
			<MessageSquare size="16" strokeWidth="3" />
			<span class="ms-1 me-3">Message</span>
			<Tooltip>
				<p>Describe your needs here</p>
			</Tooltip>
		</label>
		<textarea
			bind:value={message}
			class="h-20 flex-grow rounded-md border resize-none border-neutral-200 p-2 focus:ring-2 focus:ring-orange-500 focus:outline-none"
			id="message"
			placeholder="I need help on a project I'm working on"
		/>
		<button
			class="my-4 self-end rounded-md bg-orange-500 px-4 py-2 font-medium text-orange-100 hover:opacity-75 active:opacity-50"
			type="submit"
		>
			<span>Submit</span>
		</button>
	</div>
</form>

<style lang="postcss">
</style>
