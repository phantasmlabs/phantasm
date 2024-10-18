<script lang="ts">
  export let title: string
  let full = title ? `${title} \u2013 Phantasm` : "Phantasm"
</script>

<svelte:head>
  <title>{full}</title>
</svelte:head>
