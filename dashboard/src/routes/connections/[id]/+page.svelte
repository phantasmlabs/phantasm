<script lang="ts">
  import type { PageData } from "./$types"
  import type { Connection, Alert } from "$lib/types"
  import { onMount } from "svelte"
  import { connections, alerts } from "$lib/store"
  import Title from "$lib/components/utils/title.svelte"
  import { goto } from "$app/navigation"

  export let data: PageData

  let connected = false
  let connection: Connection
  let ws: WebSocket

  onMount(() => {
    let conn = $connections.find((conn) => conn.id === data.id)
    if (!conn) {
      alerts.update((alerts) => {
        let newAlert: Alert = {
          id: crypto.randomUUID(),
          type: "error",
          message: "Failed to find the connection details."
        }

        alerts.push(newAlert)
        return alerts
      })

      goto("/")
      return
    }

    connection = conn
    ws = new WebSocket(`ws://${connection.address}`)

    ws.onopen = () => {
      alerts.update((alerts) => {
        let newAlert: Alert = {
          id: crypto.randomUUID(),
          type: "success",
          message: "Connected to the server successfully."
        }

        alerts.push(newAlert)
        return alerts
      })

      connected = true
    }

    ws.onmessage = (event) => {
      const message = event.data
      console.log(message)
    }

    ws.onerror = (error) => {
      console.error(error)
      alerts.update((alerts) => {
        let newAlert: Alert = {
          id: crypto.randomUUID(),
          type: "error",
          message: "Encountered an error connecting to the server."
        }

        alerts.push(newAlert)
        return alerts
      })

      goto("/")
    }

    ws.onclose = () => {
      goto("/")
    }

    return () => {
      ws.close()
      goto("/")
    }
  })
</script>

<Title title="Dashboard" />

{#if connected}
  <p>Connected</p>
{/if}
