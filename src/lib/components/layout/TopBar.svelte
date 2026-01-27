<script lang="ts">
  import ProfileSelector from "$lib/components/profile/ProfileSelector.svelte";
</script>

<header
  class="flex h-14 shrink-0 items-center justify-between border-b border-border bg-card px-4"
  aria-label="Intestazione applicazione"
>
  <h1 class="text-lg font-semibold tracking-tight text-foreground">
    PokeTracker
  </h1>
  <div class="flex items-center gap-4">
    <ProfileSelector />
  </div>
</header>
