<script lang="ts">
  import type { PageData } from "./$types"
  import type { Connection, Alert, ApprovalRequest } from "$lib/types"
  import { flip } from "svelte/animate"
  import { onMount, setContext } from "svelte"
  import { connections, alerts } from "$lib/store"
  import { goto } from "$app/navigation"
  import Title from "$lib/components/utils/title.svelte"
  import ApprovalCard from "$lib/components/cards/approval.svelte"

  export let data: PageData

  let connected = true
  let connection: Connection
  let requests: ApprovalRequest[] = []

  let ws: WebSocket
  $: if (ws) setContext("ws", ws)

  function returnHome() {
    goto("/")
  }

  function removeRequest(id: string) {
    requests = requests.filter((req) => req.id !== id)
    window.localStorage.setItem(connection.id, JSON.stringify(requests))
  }

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

      returnHome()
      return
    }

    connection = conn
    ws = new WebSocket(`ws://${connection.address}`)

    let storedRequests = window.localStorage.getItem(connection.id)
    if (storedRequests) {
      requests = JSON.parse(storedRequests)
    }

    ws.onopen = () => {
      connected = true
      alerts.update((alerts) => {
        let newAlert: Alert = {
          id: crypto.randomUUID(),
          type: "success",
          message: "Connected to the server successfully."
        }

        alerts.push(newAlert)
        return alerts
      })
    }

    ws.onmessage = (event) => {
      const message = event.data
      requests = [...requests, JSON.parse(message)]
      window.localStorage.setItem(connection.id, JSON.stringify(requests))
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

      returnHome()
    }

    ws.onclose = () => {
      returnHome()
    }

    return () => {
      ws.close()
      returnHome()
    }
  })
</script>

<Title title="Dashboard" />

{#if connected}
  <div class="max-w-screen-sm mx-auto px-6 py-24">
    <div class="flex flex-col space-y-6">
      <h1>Approval Requests</h1>
      {#if requests.length == 0}
        <p>There is no approval request to review at the moment.</p>
      {:else}
        <div class="flex flex-col space-y-3">
          {#each requests as request (request.id)}
            <div animate:flip={{ duration: 200 }}>
              <ApprovalCard
                {request}
                remove={() => removeRequest(request.id)}
              />
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
{/if}
