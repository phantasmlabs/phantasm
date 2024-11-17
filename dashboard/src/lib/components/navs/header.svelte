<script lang="ts">
  import { SidePanelOpenFilled } from "carbon-icons-svelte"
  export let openSidebar: () => void
</script>

<header class="header space-x-4">
  <button class="sidebar-button-toggle" on:click={openSidebar}>
    <SidePanelOpenFilled size={24} />
  </button>
</header>

<style lang="postcss">
  .header {
    @apply flex items-center p-4 bg-gray-50;
    @apply absolute top-0 left-0 w-full z-[4];
    height: 80px;
  }
</style>
