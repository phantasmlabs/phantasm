<script lang="ts">
  import { onMount } from "svelte"
  import type { PageData } from "./$types"
  import { connections } from "$lib/store"

  export let data: PageData

  let ws: WebSocket

  onMount(() => {
    let conn = $connections.find((c) => c.id === data.id)
    if (!conn) {
      return
    }

    let address = conn?.address
    ws = new WebSocket(`ws://${address}`)

    ws.onopen = () => {
      console.log("WebSocket connection opened")
    }

    ws.onmessage = (event) => {
      const message = event.data
      console.log(message)
    }

    ws.onerror = (error) => {
      console.error("WebSocket error: ", error)
    }

    ws.onclose = () => {
      console.log("WebSocket connection closed")
    }

    return () => {
      ws.close()
    }
  })
</script>
