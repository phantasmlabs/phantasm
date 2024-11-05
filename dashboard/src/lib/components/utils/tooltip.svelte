<script lang="ts">
  export let content: string
  export let show: boolean = false
  export let close: () => void

  function clickoutside(node: HTMLElement, callback: () => void) {
    const click = (event: MouseEvent) => {
      if (!node.contains(event.target as Node)) {
        callback()
      }
    }

    document.addEventListener("click", click, true)
    const destroy = () => document.removeEventListener("click", click, true)
    return { destroy }
  }
</script>

{#if show && content}
  <div class="tooltip" use:clickoutside={close}>
    <div class="tooltip-content">
      {content}
    </div>
  </div>
{/if}

<style lang="postcss">
  .tooltip {
    @apply absolute bottom-[48px] left-0;
    @apply rounded shadow bg-white;
  }

  .tooltip-content {
    @apply text-xs text-gray-500 whitespace-pre-line;
    @apply max-w-96 max-h-48 p-4 overflow-y-auto;
  }
</style>
