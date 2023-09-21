<script>
  import { page } from '$app/stores';
</script>

<div class="container text-white">
    <div class="w-fit mx-auto my-16 flex flex-col">
        <h1 class="text-9xl ">{$page.status}</h1>
        <p class="mt-2 text-2xl">{$page.error?.message}</p>
        <a class="mt-16 text-orange-500 underline" href="/">Go home</a>
    </div>
</div>
