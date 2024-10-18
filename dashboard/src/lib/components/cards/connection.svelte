<script lang="ts">
  import type { Connection } from "$lib/types"
  import { ChevronRight } from "carbon-icons-svelte"
  export let connection: Connection
</script>

<div class="group card">
  <div class="flex flex-col space-y-1 w-full truncate">
    <h4 class="font-bold text-xl">{connection.name}</h4>
    <small><code>{connection.address}</code></small>
  </div>
  <div class="card-button text-blue-700 group-hover:translate-x-2">
    <ChevronRight size={24} />
  </div>
</div>

<style lang="postcss">
  .card {
    @apply flex items-center space-x-4 p-4 cursor-pointer rounded;
    @apply border bg-blue-50 border-blue-100;
    @apply transition duration-300 ease-in-out;
  }

  .card:hover {
    @apply shadow border-blue-300;
  }

  .card-button {
    @apply flex flex-none w-8 h-8 items-center justify-center;
    @apply transition duration-300 ease-in-out;
  }
</style>
