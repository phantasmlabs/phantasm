<script lang="ts">
  import { flip } from "svelte/animate"
  import { connections } from "$lib/store"
  import type { Connection } from "$lib/types"
  import Title from "$lib/components/utils/title.svelte"
  import ActionButton from "$lib/components/buttons/action.svelte"
  import BasicButton from "$lib/components/buttons/basic.svelte"
  import ConnectionCard from "$lib/components/cards/connection.svelte"
  import BasicModal from "$lib/components/modals/basic.svelte"
  import InputField from "$lib/components/inputs/field.svelte"
  import Add from "carbon-icons-svelte/lib/Add.svelte"
  import { goto } from "$app/navigation"

  let showAddConnectionModal = false
  let connectionName = ""
  let connectionAddress = ""

  $: if (!showAddConnectionModal) {
    connectionName = ""
    connectionAddress = ""
  }

  function addConnection(name: string, address: string) {
    if (!name || !address) {
      return
    }

    let connection: Connection = {
      id: crypto.randomUUID(),
      name: name,
      address: address
    }

    connections.update((conn) => {
      conn.push(connection)
      return conn
    })

    window.localStorage.setItem("connections", JSON.stringify($connections))
    connect(connection.id)
  }

  function deleteConnection(id: string) {
    connections.update((conn) => {
      return conn.filter((c) => c.id !== id)
    })

    window.localStorage.setItem("connections", JSON.stringify($connections))
  }

  function connect(id: string) {
    goto(`/connections/${id}`)
  }
</script>

<Title title="Connections" />

<BasicModal bind:show={showAddConnectionModal}>
  <div class="flex flex-col space-y-6">
    <h3>Add Connection</h3>
    <div class="flex flex-col space-y-3">
      <InputField
        id="connection-name"
        label="Name"
        placeholder="ChatGPT"
        bind:value={connectionName}
      />
      <InputField
        id="connection-address"
        label="Address"
        placeholder="25.5.200.0:2505"
        bind:value={connectionAddress}
      />
    </div>
    <div class="flex flex-col space-y-3 items-center">
      <BasicButton
        text="Connect"
        action={() => {
          if (connectionName && connectionAddress) {
            addConnection(connectionName, connectionAddress)
            showAddConnectionModal = false
          }
        }}
      />
      <BasicButton
        text="Cancel"
        theme="secondary"
        action={() => {
          showAddConnectionModal = false
        }}
      />
    </div>
  </div>
</BasicModal>

<div class="max-w-screen-sm mx-auto px-6">
  <div class="py-24 space-y-9">
    {#if $connections.length == 0}
      <div class="flex flex-col items-center text-center space-y-3">
        <h3>It looks quite empty in here!</h3>
        <p class="max-w-md">
          Add your first connection to a Phantasm server to start receiving
          approval requests from your AI agents.
        </p>
      </div>
    {:else}
      <div class="flex flex-col space-y-3">
        {#each $connections as conn (conn.id)}
          <div animate:flip={{ duration: 200 }}>
            <ConnectionCard
              connection={conn}
              connect={() => {
                connect(conn.id)
              }}
              deleteConnection={() => {
                deleteConnection(conn.id)
              }}
            />
          </div>
        {/each}
      </div>
    {/if}
    <ActionButton
      text="Add Connection"
      icon={Add}
      action={() => {
        showAddConnectionModal = true
      }}
    />
  </div>
</div>
