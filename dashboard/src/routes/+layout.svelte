<script lang="ts">
  import "../app.css"
  import "@fontsource/cabin/400.css"
  import "@fontsource/cabin/700.css"
  import "@fontsource/inconsolata/400.css"
  import "@fontsource/inconsolata/700.css"

  import { flip } from "svelte/animate"
  import { onMount } from "svelte"
  import { connections, alerts, approver } from "$lib/store"
  import type { Approver } from "$lib/types"
  import Alert from "$lib/components/cards/alert.svelte"
  import BasicModal from "$lib/components/modals/basic.svelte"
  import InputField from "$lib/components/inputs/field.svelte"
  import BasicButton from "$lib/components/buttons/basic.svelte"

  let hydrated = false
  let showApproverModal = false

  let approverName = ""
  let approverEmail = ""

  onMount(() => {
    let storedConnections = window.localStorage.getItem("connections")
    if (storedConnections) {
      connections.set(JSON.parse(storedConnections))
    }

    let storedApprover = window.localStorage.getItem("approver")
    if (storedApprover) {
      approver.set(JSON.parse(storedApprover))
    }

    showApproverModal = !$approver
    hydrated = true
  })

  function removeAlert(id: string) {
    alerts.update((alerts) => alerts.filter((alert) => alert.id !== id))
  }

  function saveProfile(name: string, email: string) {
    if (!name || !email) {
      return
    }

    let profile: Approver = {
      name: name,
      email: email
    }

    approver.set(profile)
    window.localStorage.setItem("approver", JSON.stringify($approver))
    showApproverModal = false
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

  <BasicModal bind:show={showApproverModal}>
    <div class="flex flex-col space-y-6">
      <div class="flex flex-col space-y-3">
        <h3>Approver Profile</h3>
        <small>
          This information gives more context to the approval response.
        </small>
      </div>
      <div class="flex flex-col space-y-3">
        <InputField
          id="name"
          label="Full Name"
          placeholder="Justin Case"
          bind:value={approverName}
        />
        <InputField
          id="email"
          label="Email Address"
          placeholder="justincase@example.com"
          bind:value={approverEmail}
        />
      </div>
      <BasicButton
        text="Save Profile"
        action={() => saveProfile(approverName, approverEmail)}
      />
    </div>
  </BasicModal>
{/if}
