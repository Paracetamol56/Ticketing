<script lang="ts">
	import { createLabel, melt } from '@melt-ui/svelte';
	import { AtSign, MessageSquare, User } from 'lucide-svelte';
	import Tooltip from './Tooltip.svelte';
	import { issueTicket } from '../services/api';
	import { goto } from '$app/navigation';
	import { addToast } from './+layout.svelte';
	import type TicketModel from '../models/ticket';

	const {
		elements: { root }
	} = createLabel();

	let name = '';
	let nameError = '';
	let email = '';
	let emailError = '';
	let message = '';
  let messageCharactersLeft = 1000;
	let messageError = '';

	export let successCallback: (ticket: TicketModel) => void;

  function validateName(): boolean {
    if (name.length == 0) {
      nameError = 'Name is required';
      return false;
    }
    if (name.length < 3) {
      nameError = 'Name must be at least 3 characters long';
      return false;
    }
    if (name.length > 100) {
      nameError = 'Name must be less than 100 characters long';
      return false;
    }
    nameError = '';
    return true;
  }
  function validateEmail(): boolean {
    if (email.length == 0) {
      emailError = 'Email is required';
      return false;
    }
    if (!email.match(/^[^\s@]+@[^\s@]+\.[^\s@]+$/)) {
      emailError = 'Invalid email';
      return false;
    }
    emailError = '';
    return true;
  }
  function validateMessage(): boolean {
    if (message.length == 0) {
      messageError = 'Message is required';
      return false;
    }
    if (message.length < 3) {
      messageError = 'Message must be at least 3 characters long';
      return false;
    }
    if (message.length > 1000) {
      messageError = 'Message must be less than 1000 characters long';
      return false;
    }
    messageError = '';
    return true;
  }
	function handleSubmit(event: Event) {
		event.preventDefault();

    if (!validateName() || !validateEmail() || !validateMessage()) {
      addToast({
        data: {
          title: 'Invalid form',
          description: 'Please check your inputs',
          color: 'bg-red-500'
        }
      });
      return;
    }

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
			successCallback(ticket);
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
      on:blur={validateName}
			class="h-10 flex-grow rounded-md border border-neutral-200 p-2 focus:ring-2 focus:ring-orange-500 focus:outline-none {nameError != '' ? 'ring-2 ring-red-500' : ''}"
			type="text"
			id="name"
			placeholder="John Doe"
		/>
    {#if nameError}
      <p class="text-xs text-red-500">{nameError}</p>
    {/if}
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
      on:blur={validateEmail}
			class="h-10 flex-grow rounded-md border border-neutral-200 p-2 focus:ring-2 focus:ring-orange-500 focus:outline-none {emailError != '' ? 'ring-2 ring-red-500' : ''}"
			type="email"
			id="email"
			placeholder="example@mail.com"
		/>
    {#if emailError}
      <p class="text-xs text-red-500">{emailError}</p>
    {/if}
		<label
			use:melt={$root}
			class="my-4 flex justify-start  text-md font-semibold leading-none text-orange-500"
			for="email"
			data-melt-part="root"
		>
			<MessageSquare size="16" strokeWidth="3" />
			<span class="ms-1 me-3">Message</span>
			<Tooltip>
				<p>Describe your needs here</p>
			</Tooltip>
      <span class="ms-auto text-xs text-neutral-400 {messageCharactersLeft != 1000 && messageCharactersLeft > 997 || messageCharactersLeft < 0 ? 'text-orange-400' : ''}">{messageCharactersLeft}</span>
		</label>
    <textarea
      bind:value={message}
      on:blur={validateMessage}
      on:input={() => messageCharactersLeft = 1000 - message.length}
      class="h-20 flex-grow rounded-md border resize-none border-neutral-200 p-2 focus:ring-2 focus:ring-orange-500 focus:outline-none {messageError != '' ? 'ring-2 ring-red-500' : ''}"
      id="message"
      placeholder="I need help on a project I'm working on"
    />
    {#if messageError}
      <p class="text-xs text-red-500">{messageError}</p>
    {/if}
		<button
			class="my-4 self-end rounded-md bg-orange-500 px-4 py-2 font-medium text-orange-100 shadow-lg hover:opacity-75 active:opacity-50"
			type="submit"
		>
			<span>Submit</span>
		</button>
	</div>
</form>

<style lang="postcss">
</style>
