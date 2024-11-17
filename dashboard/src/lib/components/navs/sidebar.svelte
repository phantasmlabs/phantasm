<script lang="ts">
  import Wordmark from "$lib/components/utils/wordmark.svelte"
  import { SidePanelCloseFilled } from "carbon-icons-svelte"

  export let open: boolean = true
</script>

{#if open}
  <div class="overlay" />
  <nav class="sidebar">
    <div class="control-section">
      <Wordmark size="sm" />
      <button class="sidebar-button-toggle">
        <SidePanelCloseFilled size={24} />
      </button>
    </div>
  </nav>
{/if}

<style lang="postcss">
  .sidebar {
    @apply flex flex-col flex-none fixed top-0 left-0;
    @apply bg-white h-dvh z-[6];
    width: 320px;
  }

  .overlay {
    @apply fixed top-0 left-0 w-dvw h-dvh;
    @apply bg-black bg-opacity-50 z-[5];
  }

  .control-section {
    @apply flex items-center justify-between p-4;
    height: 80px;
  }

  @screen lg {
    .sidebar {
      @apply static;
    }

    .overlay {
      @apply hidden;
    }
  }
</style>
