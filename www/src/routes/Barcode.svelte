<script lang="ts">
	import { onMount } from 'svelte';
	import JsBarcode from 'jsbarcode';

	onMount(() => {
		JsBarcode('#barcode', value, {format: 'CODE128', displayValue: false});
	});

  export let value: string = '6509852fd107ccc2b0f8574e';
</script>

<svg id="barcode" class="w-full h-20" />
