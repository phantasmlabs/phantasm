<script lang="ts">
  import { onMount, getContext } from "svelte"
  import { approver } from "$lib/store"
  import type { ApprovalRequest, ApprovalResponse, Approver } from "$lib/types"
  import { EditorView, keymap, highlightActiveLine } from "@codemirror/view"
  import { EditorState } from "@codemirror/state"
  import { defaultKeymap, indentWithTab } from "@codemirror/commands"
  import { javascript } from "@codemirror/lang-javascript"
  import Tooltip from "../utils/tooltip.svelte"

  // Icons.
  import {
    Checkmark,
    CloseLarge,
    PartitionSame,
    Information
  } from "carbon-icons-svelte"

  const iconSize = 16

  export let request: ApprovalRequest
  export let remove: () => void

  let editor: Element
  let ws: WebSocket = getContext("ws")
  let showTooltip: boolean = false
  let showParameters: boolean = false
  let error = false

  let parameters = JSON.parse(request.parameters)
  parameters = JSON.stringify(parameters, null, 2)

  onMount(() => {
    const onchange = EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        error = false
        parameters = update.state.doc.toString()

        try {
          JSON.parse(parameters)
        } catch (e) {
          console.error(e)
          error = true
        }
      }
    })

    const state = EditorState.create({
      doc: parameters,
      extensions: [
        keymap.of([...defaultKeymap, indentWithTab]),
        highlightActiveLine(),
        javascript(),
        onchange
      ]
    })

    let view = new EditorView({
      state,
      parent: editor
    })

    return () => view.destroy()
  })

  function approve() {
    if (error) return

    // This ensures that the parameters are always valid and formatted
    // before sending them back to the server.
    let _params = JSON.parse(parameters)
    let params = JSON.stringify(_params)

    let response: ApprovalResponse = {
      id: request.id,
      approved: true,
      parameters: params,
      approver: $approver as Approver
    }

    ws.send(JSON.stringify(response))
    remove()
  }

  function reject() {
    let response: ApprovalResponse = {
      id: request.id,
      approved: false,
      parameters: "",
      approver: $approver as Approver
    }

    ws.send(JSON.stringify(response))
    remove()
  }
</script>

<small class="text-xs text-gray-300">{request.id}</small>

<div class="group card space-x-4">
  <div class="w-full flex items-center space-x-2">
    {#if request.context}
      <button
        class="text-gray-400"
        on:click={() => {
          showTooltip = !showTooltip
        }}
      >
        <Information size={iconSize} />
      </button>
    {/if}
    <p class="font-mono font-bold truncate">
      {request.name}
    </p>
  </div>
  <div class="flex flex-none space-x-1">
    <button class="card-button green space-x-1" on:click={approve}>
      <Checkmark size={iconSize} />
      <span>Approve</span>
    </button>
    <button class="card-button red" on:click={reject}>
      <CloseLarge size={iconSize} />
    </button>
    <button
      class="card-button blue"
      on:click={() => {
        showParameters = !showParameters
      }}
    >
      <PartitionSame size={iconSize} />
    </button>
  </div>

  <!-- This tooltip is triggered by the info icon button. -->
  <Tooltip
    show={showTooltip}
    content={request.context}
    close={() => {
      showTooltip = false
    }}
  />
</div>

<div class="card-editor-container space-y-1 {showParameters ? 'show' : 'hide'}">
  <div bind:this={editor} class="card-editor" />
  {#if error}
    <div class="text-red-500 text-xs text-right">
      Syntax error occurred in the parameters
    </div>
  {/if}
</div>

<style lang="postcss">
  .card {
    @apply flex flex-row relative items-center p-4 rounded;
    @apply border bg-gray-50 border-gray-200;
  }

  .card-editor-container {
    @apply flex flex-col p-4;
    @apply bg-gray-50 rounded mt-1;
  }

  .card-editor-container.hide {
    @apply hidden;
  }

  .card-editor-container.show {
    @apply flex;
  }

  .card-editor {
    @apply w-full rounded;
    @apply text-sm font-mono;
  }

  .card-button {
    @apply flex items-center justify-center p-2 rounded text-xs;
    @apply transition duration-300 ease-in-out;
  }

  .card-button.green {
    @apply bg-green-600 text-white;
  }

  .card-button.green:hover {
    @apply bg-green-700;
  }

  .card-button.red {
    @apply border border-red-500 text-red-500;
  }

  .card-button.red:hover {
    @apply bg-red-50 border-red-600 text-red-600;
  }

  .card-button.blue {
    @apply border border-blue-500 text-blue-500;
  }

  .card-button.blue:hover {
    @apply bg-blue-50 border-blue-600 text-blue-600;
  }
</style>
