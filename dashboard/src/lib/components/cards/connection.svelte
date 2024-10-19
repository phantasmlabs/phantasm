<script lang="ts">
  import type { Connection } from "$lib/types"
  import { TrashCan, Link } from "carbon-icons-svelte"

  const iconSize = 16

  export let connection: Connection
  export let connect: () => void
  export let deleteConnection: () => void
</script>

<div class="group card" id="conn-{connection.id}">
  <div class="flex flex-col items-start space-y-1 w-full truncate">
    <h4 class="font-bold text-xl">{connection.name}</h4>
    <small><code>{connection.address}</code></small>
  </div>
  <div class=" flex-none space-x-2 hidden group-hover:flex">
    <button class="card-button primary space-x-1" on:click={connect}>
      <Link size={iconSize} />
      <span class="text-xs">Connect</span>
    </button>
    <button class="card-button red" on:click={deleteConnection}>
      <TrashCan size={iconSize} />
    </button>
  </div>
</div>

<style lang="postcss">
  .card {
    @apply flex items-center space-x-4 p-4 rounded;
    @apply border bg-blue-50 border-blue-100;
    @apply transition duration-300 ease-in-out;
  }

  .card:hover {
    @apply shadow border-blue-300;
  }

  .card-button {
    @apply flex items-center justify-center p-2 rounded;
    @apply transition duration-300 ease-in-out;
  }

  .card-button.primary {
    @apply bg-blue-700 text-white;
  }

  .card-button.primary:hover {
    @apply bg-blue-800;
  }

  .card-button.red {
    @apply border border-red-500 text-red-500;
  }

  .card-button.red:hover {
    @apply bg-red-50 border-red-600 text-red-600;
  }
</style>
