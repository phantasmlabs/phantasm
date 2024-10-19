<script>
  import "../app.css"
  import "@fontsource/cabin/400.css"
  import "@fontsource/cabin/700.css"

  import { onMount } from "svelte"
  import { connections } from "$lib/store"

  let hydrated = false

  onMount(() => {
    let storedConnections = window.localStorage.getItem("connections")
    if (storedConnections) {
      connections.set(JSON.parse(storedConnections))
    }

    hydrated = true
  })
</script>

{#if hydrated}
  <slot />
{/if}
