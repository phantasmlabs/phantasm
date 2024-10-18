<script>
  import Wordmark from "$lib/components/utils/wordmark.svelte"
  import Title from "$lib/components/utils/title.svelte"
  import Button from "$lib/components/buttons/with-icon.svelte"
  import Add from "carbon-icons-svelte/lib/Add.svelte"

  let connections = []
</script>

<Title title="Connections" />

<div class="max-w-screen-sm mx-auto px-6">
  <div class="py-24 space-y-12">
    <div class="flex items-center justify-center">
      <Wordmark size="lg" />
    </div>
    {#if connections.length == 0}
      <div class="flex flex-col items-center text-center space-y-3">
        <h3>It looks quite empty in here!</h3>
        <p class="max-w-md">
          Add a connection to a Phantasm server to start receiving approval
          requests from your AI agents.
        </p>
      </div>
    {:else}
      <div></div>
    {/if}
    <div class="flex flex-col space-y-3 items-center text-center">
      <Button text="Add Connection" icon={Add} />
      <small>Connection details are stored locally!</small>
    </div>
  </div>
</div>
