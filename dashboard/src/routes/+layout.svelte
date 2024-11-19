<script lang="ts">
  import "../app.css"
  import "@fontsource/cabin/400.css"
  import "@fontsource/cabin/700.css"
  import "@fontsource/inconsolata/400.css"
  import "@fontsource/inconsolata/700.css"

  import { flip } from "svelte/animate"
  import { onMount } from "svelte"
  import { connections, alerts, approver } from "$lib/store"
  import { fly } from "svelte/transition"

  import Alert from "$lib/components/cards/alert.svelte"
  import Sidebar from "$lib/components/navs/sidebar.svelte"
  import Header from "$lib/components/navs/header.svelte"

  const animationDuration = 200
  let hydrated = false
  let showSidebar = true

  onMount(() => {
    let storedConnections = window.localStorage.getItem("connections")
    if (storedConnections) {
      connections.set(JSON.parse(storedConnections))
    }

    let storedApprover = window.localStorage.getItem("approver")
    if (storedApprover) {
      approver.set(JSON.parse(storedApprover))
    }

    hydrated = true
  })

  function removeAlert(id: string) {
    alerts.update((alerts) => alerts.filter((alert) => alert.id !== id))
  }
</script>

{#if hydrated}
  <div class="flex bg-gray-100">
    <Sidebar
      open={showSidebar}
      close={() => {
        showSidebar = false
      }}
    />
    <div class="relative w-full">
      <Header
        openSidebar={() => {
          showSidebar = true
        }}
      />
      <main class="h-dvh overflow-y-auto">
        <!-- This pads the layout in place of the header. -->
        <div class="h-[80px]" />
        <slot />
      </main>
    </div>
  </div>
{/if}

<div class="fixed bottom-0 right-0 space-y-3 p-6 w-full md:w-[480px]">
  {#each $alerts as alert (alert.id)}
    <div
      animate:flip={{ duration: animationDuration }}
      in:fly={{ x: -100, duration: animationDuration }}
      out:fly={{ x: 300, duration: animationDuration }}
    >
      <Alert
        {alert}
        remove={() => {
          removeAlert(alert.id)
        }}
      />
    </div>
  {/each}
</div>
