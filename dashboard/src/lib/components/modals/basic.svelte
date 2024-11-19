<script lang="ts">
  import { fly, fade } from "svelte/transition"
  export let show: boolean = true
</script>

{#if show}
  <div class="modal-container">
    <div
      class="backdrop"
      role="none"
      transition:fade={{ duration: 50 }}
      on:click={() => {
        show = false
      }}
    />
    <div class="modal" transition:fly={{ y: 128 }}>
      <slot />
    </div>
  </div>
{/if}

<style lang="postcss">
  .modal-container {
    @apply flex items-center justify-center;
    @apply fixed top-0 left-0 w-screen h-dvh z-10;
  }

  .backdrop {
    @apply fixed top-0 left-0 w-full h-full;
    @apply bg-black bg-opacity-50 cursor-pointer;
  }

  .modal {
    @apply flex flex-col p-6 z-[11];
    @apply bg-white w-full h-full;
  }

  @screen md {
    .modal {
      @apply rounded w-[480px] h-auto;
    }
  }
</style>
