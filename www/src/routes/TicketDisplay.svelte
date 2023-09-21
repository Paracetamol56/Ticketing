<script lang="ts">
	import { onMount, tick } from "svelte";
	import type TicketModel from "../models/ticket";
  import JsBarcode from "jsbarcode";

  onMount(() => {
    JsBarcode("#barcode", ticket.id, {
      format: "CODE128",
      displayValue: false,
      height: 100,
      width: 2,
      margin: 0,
    });
  });

  function getStatusColor() {
    switch (ticket.status) {
      case "open":
        return "bg-orange-500";
      case "pending":
        return "bg-red-500";
      case "closed":
        return "bg-green-500";
      default:
        return "bg-gray-500";
    }
  }

  export let ticket: TicketModel;
  export let resetCallback: () => void;
</script>

<div class="h-full flex flex-col justify-start font-mono text-neutral-800">
  <svg id="barcode" class="w-full h-fit"></svg>
  <h3 class="mt-4 text-2xl font-bold text-orange-500 text-center">Your ticket</h3>
  <p class="text-9xl font-bold text-center">{ticket.number}</p>
  
  <div class="flex justify-between items-start">
    <p class="text-base me-2">
      Status
    </p>
    <div class="flex justify-center items-center gap-2">
      <div class={`w-3 h-3 rounded-full ${getStatusColor()}`}></div>
      <p class="text-base font-bold uppercase">{ticket.status}</p>
    </div>
  </div>
  <hr class="my-3 h-[2px] w-full border-none bg-neutral-800" />
  <div class="flex justify-between items-start">
    <p class="text-sm me-2">
      Name
    </p>
    <p class="text-sm text-end text-ellipsis overflow-hidden">
      {ticket.name}
    </p>
  </div>
  <div class="flex justify-between items-start">
    <p class="text-sm me-2">
      Email
    </p>
    <p class="text-sm text-end text-ellipsisoverflow-hidden">
      {ticket.email}
    </p>
  </div>
  <div class="flex justify-between items-start">
    <p class="text-sm me-2">
      Message
    </p>
    <p class="text-sm text-end text-ellipsis overflow-hidden">
      {ticket.message}
    </p>
  </div>

  <button class="mt-auto text-xs text-orange-800 text-center" on:click={() => resetCallback()}>
    Get another ticket !
  </button>
</div>
