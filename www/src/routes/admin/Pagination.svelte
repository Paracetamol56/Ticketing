<script lang="ts">
	import { createPagination, melt, type CreatePaginationProps } from '@melt-ui/svelte';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';

  export let changePageCallback: (page: number) => void;
  export let count: number;

  const handlePageChange: CreatePaginationProps['onPageChange'] = ({ curr, next }) => {
    if (next) {
      console.log(next);
      changePageCallback(next);
    }
    return next;
  };
	const {
		elements: { root, pageTrigger, prevButton, nextButton },
		states: { pages, range }
	} = createPagination({
		count: count,
		perPage: 10,
		defaultPage: 1,
		siblingCount: 1,
    onPageChange: handlePageChange,
	});
</script>

<nav
  class="flex flex-col items-center gap-4"
  aria-label="pagination"
  use:melt={$root}
>
  <p class="text-center text-orange-800">
    Showing items {$range.start} - {$range.end}
  </p>
  <div class="flex items-center gap-2">
    <button
      class="grid h-8 items-center rounded-md bg-white px-3 text-sm text-orange-800 shadow-sm
      hover:opacity-75 disabled:cursor-not-allowed disabled:opacity-50 data-[selected]:bg-orange-800
      data-[selected]:text-white"
      use:melt={$prevButton}><ChevronLeft class="square-4" /></button
    >
    {#each $pages as page (page.key)}
      {#if page.type === 'ellipsis'}
        <span>...</span>
      {:else}
        <button
          class="grid h-8 items-center rounded-md bg-white px-3 text-sm text-orange-800 shadow-sm
          hover:opacity-75 disabled:cursor-not-allowed disabled:opacity-50 data-[selected]:bg-orange-800
        data-[selected]:text-white"
          use:melt={$pageTrigger(page)}>{page.value}</button
        >
      {/if}
    {/each}
    <button
      class="grid h-8 items-center rounded-md bg-white px-3 text-sm text-orange-800 shadow-sm
      hover:opacity-75 disabled:cursor-not-allowed disabled:opacity-50 data-[selected]:bg-orange-800
    data-[selected]:text-white"
      use:melt={$nextButton}><ChevronRight class="square-4" /></button
    >
  </div>
</nav>