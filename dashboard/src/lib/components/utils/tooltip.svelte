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
    <p class="text-xs text-gray-500 whitespace-pre-line">
      {content}
    </p>
  </div>
{/if}

<style lang="postcss">
  .tooltip {
    @apply absolute bottom-[48px] left-0;
    @apply max-w-96 p-4 rounded shadow bg-white;
  }
</style>
