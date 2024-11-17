<script lang="ts">
  import { SidePanelOpenFilled } from "carbon-icons-svelte"
</script>

<header class="header space-x-6">
  <button class="sidebar-button-toggle">
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
