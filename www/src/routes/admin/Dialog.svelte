<script lang="ts">
  import { createDialog, createSwitch, melt } from '@melt-ui/svelte';
  import { X } from 'lucide-svelte';
 
  export let saveCallback: (notify: boolean) => void;

  const {
    elements: {
      trigger,
      overlay,
      content,
      title,
      description,
      close,
      portalled,
    },
    states: { open },
  } = createDialog({
    role: 'alertdialog',
  });
  const {
    elements: { root, input },
    states: { checked },
  } = createSwitch({
    defaultChecked: false,
  });
</script>
 
<button
  use:melt={$trigger}
  class="rounded-md bg-orange-500 px-4 py-2 font-medium text-orange-100 shadow-lg hover:opacity-75 active:opacity-50"
>
  Save
</button>
 
<div use:melt={$portalled}>
  {#if $open}
    <div use:melt={$overlay} class="fixed inset-0 z-50 bg-black/50" />
    <div
      class="fixed left-[50%] top-[50%] z-50 max-h-[85vh] w-[90vw]
            max-w-[450px] translate-x-[-50%] translate-y-[-50%] rounded-md bg-white
            p-6 shadow-lg"
      use:melt={$content}
    >
      <h2 use:melt={$title} class="m-0 text-lg font-orange text-black">
        Are you sure you want to persist the changes?
      </h2>
      <p use:melt={$description} class="mb-5 mt-2 leading-normal text-zinc-600">
        An email notification can sent to the user, enable this option below if you want to notify the user.
      </p>

      <div class="flex items-center">
        <label class="pr-4 leading-none text-orange-800 cursor-pointer" for="notification">
          Send notification
        </label>
        <button
          use:melt={$root}
          class="switch relative h-6 rounded-full bg-orange-800 transition-colors data-[state=checked]:bg-orange-500"
          id="notification"
          aria-labelledby="notification"
        >
          <span class="thumb block rounded-full bg-white transition" />
        </button>
        <input use:melt={$input} />
      </div>

      <div class="mt-6 flex justify-end gap-4">
        <button
          class="rounded-md border-2 border-orange-500 px-4 py-1.5 font-medium text-orange-500 shadow-lg bg-transparent hover:bg-orange-50 active:bg-orange-100"
          use:melt={$close}
        >
          Cancel
        </button>
        <button
          class="rounded-md bg-orange-500 px-4 py-2 font-medium text-orange-100 shadow-lg hover:opacity-75 active:opacity-50"
          use:melt={$close}
          on:mousedown={() => {
            saveCallback($checked);
          }}
        >
          Save
        </button>
      </div>
 
      <button
        use:melt={$close}
        aria-label="Close"
        class="absolute right-[10px] top-[10px] inline-flex h-6 w-6
                appearance-none items-center justify-center rounded-full text-orange-800
                hover:bg-orange-100 focus:shadow-orange-400"
      >
        <X class="square-4" />
      </button>
    </div>
  {/if}
</div>

<style>
  .switch {
    --w: 2.75rem;
    --padding: 0.125rem;
    width: var(--w);
  }
 
  .thumb {
    --size: 1.25rem;
    width: var(--size);
    height: var(--size);
    transform: translateX(var(--padding));
  }
 
  :global([data-state='checked']) .thumb {
    transform: translateX(calc(var(--w) - var(--size) - var(--padding)));
  }
</style>
