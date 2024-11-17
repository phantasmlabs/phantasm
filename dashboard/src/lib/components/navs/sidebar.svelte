<script lang="ts">
  import { approver } from "$lib/store"
  import type { Approver } from "$lib/types"
  import Wordmark from "$lib/components/utils/wordmark.svelte"
  import BasicModal from "$lib/components/modals/basic.svelte"
  import InputField from "$lib/components/inputs/field.svelte"
  import BasicButton from "$lib/components/buttons/basic.svelte"
  import { SidePanelCloseFilled } from "carbon-icons-svelte"

  export let open: boolean = true

  let showApproverModal = !$approver
  let approverName = ""
  let approverEmail = ""

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

{#if open}
  <div class="overlay" />
  <nav class="sidebar">
    <div class="control-section">
      <Wordmark size="sm" />
      <button class="sidebar-button-toggle">
        <SidePanelCloseFilled size={24} />
      </button>
    </div>
  </nav>
{/if}

<BasicModal bind:show={showApproverModal}>
  <div class="flex flex-col space-y-6">
    <div class="flex flex-col space-y-3">
      <h3>Approver Profile</h3>
      <small>
        This information adds more context to the approval response.
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

<style lang="postcss">
  .sidebar {
    @apply flex flex-col flex-none fixed top-0 left-0;
    @apply bg-white h-dvh z-[6];
    width: 320px;
  }

  .overlay {
    @apply fixed top-0 left-0 w-dvw h-dvh;
    @apply bg-black bg-opacity-50 z-[5];
  }

  .control-section {
    @apply flex items-center justify-between p-4;
    height: 80px;
  }

  @screen lg {
    .sidebar {
      @apply static;
    }

    .overlay {
      @apply hidden;
    }
  }
</style>
