<script lang="ts">
  import type { Alert } from "$lib/types"
  export let alert: Alert
  export let remove: () => void
</script>

<div role="none" class="card {alert.type}" on:click={remove}>
  <p>{alert.message}</p>
</div>

<style lang="postcss">
  .card {
    @apply flex items-center p-4 rounded;
    @apply border border-l-8 cursor-pointer;
  }

  .card.success {
    @apply border-green-500 bg-green-50;
  }

  .card.error {
    @apply border-red-500 bg-red-50;
  }
</style>
