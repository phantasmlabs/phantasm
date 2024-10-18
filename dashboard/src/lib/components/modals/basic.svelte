<script lang="ts">
  export let show: boolean = true
</script>

{#if show}
  <div class="modal-container">
    <div
      class="backdrop"
      role="none"
      on:click={() => {
        show = false
      }}
    />
    <div class="modal">
      <slot />
    </div>
  </div>
{/if}

<style lang="postcss">
  .modal-container {
    @apply flex items-center justify-center;
    @apply fixed top-0 left-0 w-screen h-dvh z-[1];
  }

  .backdrop {
    @apply fixed top-0 left-0 w-full h-full;
    @apply bg-black bg-opacity-50;
  }

  .modal {
    @apply flex flex-col p-6 z-[2];
    @apply bg-white w-full h-full;
  }

  @screen md {
    .modal {
      @apply rounded w-[480px] h-auto;
    }
  }
</style>
