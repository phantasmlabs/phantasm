<script lang="ts">
  export let id: string
  export let label: string
  export let placeholder: string
  export let value: string

  function changeHandler(event: Event) {
    const target = event.target as HTMLInputElement
    value = target.value
  }
</script>

<label {id} class="input-group">
  <small class="absolute top-2 left-3">{label}</small>
  <input
    name={id}
    on:change={changeHandler}
    bind:value
    type="text"
    class="input-field"
    {placeholder}
  />
</label>

<style lang="postcss">
  .input-group {
    @apply flex flex-col relative;
  }

  .input-field {
    @apply p-3 pt-8 border border-blue-100 rounded;
    @apply transition duration-200 ease-in-out;
  }

  .input-field:hover {
    @apply border-blue-300;
  }

  .input-field:focus {
    @apply outline-blue-700;
  }
</style>
