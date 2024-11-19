<script lang="ts">
  import type { Approver } from "$lib/types"
  import { approver } from "$lib/store"
  import BasicModal from "$lib/components/modals/basic.svelte"
  import InputField from "$lib/components/inputs/field.svelte"
  import BasicButton from "$lib/components/buttons/basic.svelte"
  import { SidePanelOpenFilled, UserAvatarFilled } from "carbon-icons-svelte"
  export let openSidebar: () => void

  let showApproverModal = !$approver
  let approverName = $approver ? $approver.name : ""
  let approverEmail = $approver ? $approver.email : ""

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

<header class="header space-x-4">
  <button class="control-button" on:click={openSidebar}>
    <SidePanelOpenFilled size={24} />
  </button>
  <button
    class="control-button"
    on:click={() => {
      showApproverModal = true
    }}
  >
    <UserAvatarFilled size={24} />
  </button>
</header>

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
  .header {
    @apply flex items-center justify-between p-4 bg-gray-50;
    @apply absolute top-0 left-0 w-full z-[4];
    height: 80px;
  }
</style>
