<script lang="ts">
	import { Unlock } from 'lucide-svelte';
	import { createLabel, melt } from '@melt-ui/svelte';

	let token: string = '';
	export let submitCallback: (token: string) => void;

	const {
		elements: { root }
	} = createLabel();
</script>

<form
	class="w-full"
	on:submit={(e) => {
		e.preventDefault();
		submitCallback(token);
	}}
>
	<div class="flex flex-col justify-center">
		<label
			use:melt={$root}
			for="token"
			class="mb-0.5 font-medium text-orange-500"
			data-melt-part="root"
		>
			<span>Admin token</span>
		</label>
		<div class="flex align-center gap-2">
			<input
				bind:value={token}
				type="password"
				id="token"
				class="h-10 flex-grow rounded-md border border-neutral-200 p-2 focus:ring-2 focus:ring-orange-500 focus:outline-none"
			/>
			<button
				class="self-end rounded-md bg-orange-500 p-3 font-medium text-orange-100 hover:opacity-75 active:opacity-50"
				type="submit"
			>
				<Unlock size="16" strokeWidth="3" />
			</button>
		</div>
	</div>
</form>
