<script lang="ts">
  export let icon
  export let text: string
  export let action: () => void = () => {}
</script>

<button class="button" on:click={action}>
  <span><svelte:component this={icon} size={24} /></span>
  <span>{text}</span>
</button>

<style lang="postcss">
  .button {
    @apply flex items-center justify-center;
    @apply space-x-2 w-full py-3 px-6;
    @apply bg-blue-700 text-white rounded;
    @apply transition duration-300 ease-in-out;
  }

  .button:hover {
    @apply bg-blue-800;
  }
</style>
