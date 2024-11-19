<script lang="ts">
  import type { ComponentType } from "svelte"
  import { fade, slide } from "svelte/transition"
  import Wordmark from "$lib/components/utils/wordmark.svelte"
  import { SidePanelCloseFilled, Home, LogoGithub } from "carbon-icons-svelte"

  export let open: boolean = true
  export let close: () => void

  type Navigation = {
    icon: ComponentType
    label: string
    route: string
  }

  const navigation: Navigation[] = [
    {
      icon: Home,
      label: "Home",
      route: "/"
    },
    {
      icon: LogoGithub,
      label: "Repository",
      route: "https://github.com/phantasmlabs/phantasm"
    }
  ]
</script>

{#if open}
  <div
    class="overlay"
    on:click={close}
    role="none"
    transition:fade={{ duration: 100 }}
  />
  <nav class="sidebar" transition:slide={{ axis: "x", duration: 250 }}>
    <div class="control-section space-x-4">
      <Wordmark size="sm" />
      <button class="control-button" on:click={close}>
        <SidePanelCloseFilled size={24} />
      </button>
    </div>
    <div class="flex flex-col p-4 space-y-1">
      {#each navigation as nav}
        <a
          href={nav.route}
          class="navigation-link space-x-3"
          target={nav.route.startsWith("http") ? "_blank" : null}
        >
          <svelte:component this={nav.icon} size={20} class="flex-none" />
          <span>{nav.label}</span>
        </a>
      {/each}
    </div>
  </nav>
{/if}

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

  .navigation-link {
    @apply flex items-center p-2 rounded;
  }

  .navigation-link:hover {
    @apply bg-gray-100;
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
