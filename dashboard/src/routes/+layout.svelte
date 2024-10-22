<script lang="ts">
  import "../app.css"
  import "@fontsource/cabin/400.css"
  import "@fontsource/cabin/700.css"
  import "@fontsource/inconsolata/400.css"
  import "@fontsource/inconsolata/700.css"

  import { flip } from "svelte/animate"
  import { onMount } from "svelte"
  import { connections, alerts } from "$lib/store"
  import Alert from "$lib/components/cards/alert.svelte"

  let hydrated = false

  onMount(() => {
    let storedConnections = window.localStorage.getItem("connections")
    if (storedConnections) {
      connections.set(JSON.parse(storedConnections))
    }

    hydrated = true
  })

  function removeAlert(id: string) {
    alerts.update((alerts) => alerts.filter((alert) => alert.id !== id))
  }
</script>

<div class="fixed bottom-0 right-0 space-y-3 p-6 w-full md:w-[480px]">
  {#each $alerts as alert (alert.id)}
    <div animate:flip={{ duration: 200 }}>
      <Alert
        {alert}
        remove={() => {
          removeAlert(alert.id)
        }}
      />
    </div>
  {/each}
</div>

{#if hydrated}
  <slot />
{/if}
