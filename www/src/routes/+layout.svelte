<script lang="ts">
	import { onMount } from 'svelte';
	import { createPopover, createToaster, melt } from '@melt-ui/svelte';
	import { checkStatus } from '../services/api';
	import { Github, Heart, Info, X } from 'lucide-svelte';
	import type { ToastData } from '$lib';
	import { flip } from 'svelte/animate';
	import Toast from '../components/Toast.svelte';
  
	import '../app.css';
	import { fade } from 'svelte/transition';
	import Ticket from './Ticket.svelte';

	onMount(async () => {
		checkStatus().then((status) => {
			if (!status) {
				addToast({
					data: {
						title: 'Error',
						description: 'The API is not available, please try again later',
						color: 'bg-red-500'
					}
				});
			}
		});
	});

  const {
    elements: { trigger, content, arrow, close },
    states: { open },
  } = createPopover({
    forceVisible: true,
  });
</script>

<script lang="ts" context="module">
  const {
		elements,
		helpers: { addToast },
		states: { toasts },
		actions: { portal }
	} = createToaster<ToastData>();

  export { addToast };
</script>

<!-- Head -->
<svelte:head>
	<title>Ticketing - Mathéo Galuba</title>
	<meta name="description" content="Ticketing app to get in touch with me" />
	<script data-host="https://cdn.micrometrics.es" data-dnt="false" src="https://cdn.micrometrics.es/js/script.js" id="ZwSg9rf6GA" async defer></script>
</svelte:head>

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

<slot />

<footer class="relative w-full py-4  text-white">
	<div class="absolute top-0 bottom-0 left-1/2 transform -translate-x-1/2
   flex flex-wrap items-center justify-center gap-1">
		<p>App made with</p>
		<Heart class="text-orange-500" size="16" />
		<p>
			by <a class="underline text-orange-100" href="https://matheo-galuba.com/" target="_blank"
				>Matheo Galuba</a
			>
		</p>
  </div>

  <div class="absultue top-0 bottom-0 right-0 mx-4 flex items-center justify-end gap-2">
    <a href="https://github.com/Paracetamol56/Ticketing" target="_blank">
      <Github class="square-8" />
    </a>
    <button
      type="button"
      class="trigger"
      use:melt={$trigger}
      aria-label="Update dimensions"
    >
      <Info class="square-8" />
    </button>
    {#if $open}
      <div use:melt={$content} transition:fade={{ duration: 100 }} class="z-10 w-60 rounded-md bg-white p-5 shadow-sm">
        <div use:melt={$arrow} />
        <div class="flex flex-col gap-2.5">
          <p><strong>Backend</strong></p>
          <ul>
            <li><a class="underline text-orange-500" href="https://www.rust-lang.org">Rust</a></li>
            <li><a class="underline text-orange-500" href="https://tokio.rs">Tokio</a></li>
            <li><a class="underline text-orange-500" href="https://github.com/tokio-rs/axum">Axum</a></li>
          </ul>
          <p><strong>Frontend</strong></p>
          <ul>
            <li><a class="underline text-orange-500" href="https://www.svelte.dev">Svelte</a></li>
            <li><a class="underline text-orange-500" href="https://www.tailwindcss.com">TailwindCSS</a></li>
            <li><a class="underline text-orange-500" href="https://www.melt-ui.com">MeltUI</a></li>
            <li><a class="underline text-orange-500" href="https://lucide.dev">Lucide</a></li>
          </ul>
          <p><strong>Hosting</strong></p>
          <ul>
            <li><a class="underline text-orange-500" href="https://www.netlify.com">Netlify</a></li>
            <li><a class="underline text-orange-500" href="https://www.shuttle.rs">Shuttle.rs</a></li>
          </ul>
        </div>
        <button
          class="absolute right-1.5 top-1.5 flex h-7 w-7 items-center justify-center rounded-full
            text-orange-800 transition-colors hover:bg-orange-500/10
            focus-visible:ring focus-visible:ring-orange-400 focus-visible:ring-offset-2
            bg-white p-0 text-sm font-medium"
          use:melt={$close}
        >
          <X class="square-4" />
        </button>
      </div>
    {/if}
  </div>

	<!-- div class="flex items-center justify-center gap-1 text-white">
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
	</div> -->
</footer>
