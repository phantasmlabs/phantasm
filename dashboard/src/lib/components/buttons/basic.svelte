<script lang="ts">
  const themeClasses = {
    primary: "bg-blue-700 text-white hover:bg-blue-800",
    secondary: "bg-gray-200 text-gray-800 hover:bg-gray-300"
  }

  export let text: string
  export let theme: "primary" | "secondary" = "primary"
  export let action: () => void = () => {}

  let themeClass = themeClasses[theme]
</script>

<button class="button {themeClass}" on:click={action}>{text}</button>

<style lang="postcss">
  .button {
    @apply flex items-center justify-center;
    @apply w-full py-3 px-6;
    @apply rounded transition duration-300 ease-in-out;
  }
</style>
