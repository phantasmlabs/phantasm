<script lang="ts">
  export let icon
  export let text: string
</script>

<button class="button">
  <span><svelte:component this={icon} size={32} /></span>
  <span>{text}</span>
</button>

<style lang="postcss">
  .button {
    @apply flex items-center justify-center;
    @apply space-x-2 w-full py-3 px-6;
    @apply bg-blue-700 text-white rounded;
  }

  .button:hover {
    @apply brightness-90;
  }
</style>
