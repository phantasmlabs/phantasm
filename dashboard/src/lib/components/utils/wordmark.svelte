<script lang="ts">
  const logoClasses = {
    sm: "w-10 h-10",
    md: "w-12 h-12",
    lg: "w-16 h-16"
  }

  const fontClasses = {
    sm: "text-lg",
    md: "text-2xl",
    lg: "text-3xl"
  }

  export let size: "sm" | "md" | "lg" = "md"
  let logoClass = logoClasses[size]
  let fontClass = fontClasses[size]
</script>

<div class="flex items-center">
  <img src="/imgs/favicon256.png" alt="Logo" class={logoClass} />
  <span class="font-bold {fontClass}">Phantasm</span>
</div>
