<script lang="ts">
  import { Eye, CheckCircle } from "@lucide/svelte";
  import type { PokedexStatus } from "$lib/data/pokedex-species";
  import { getSpeciesTypes } from "$lib/data/pokedex-types";

  interface Props {
    id: number;
    name: string;
    status: PokedexStatus;
  }

  let { id, name, status }: Props = $props();

  const spriteSrc = $derived(`/pokedex-sprites/${id}.png`);
  const displayName = $derived(status === "unseen" ? "???" : name);
  const numberLabel = $derived(`#${String(id).padStart(3, "0")}`);
  const isUnseen = $derived(status === "unseen");
  const types = $derived(getSpeciesTypes(id));
</script>

<article
  aria-label="{displayName} ({numberLabel}) — {status === 'unseen' ? 'Non visto' : status === 'seen' ? 'Visto' : 'Catturato'}"
  class="flex h-[10.5rem] w-full flex-col items-center rounded-lg border border-[var(--border-primary)] bg-[var(--bg-tertiary)] px-3 pt-3 pb-2 transition-[background-color] duration-200 hover:bg-[var(--hover-bg)]"
>
  <span class="text-xs text-muted-foreground">{numberLabel}</span>
  <div class="relative mt-1 flex h-16 w-16 shrink-0 items-center justify-center">
    {#if isUnseen}
      <!-- Non visto: solo punto di domanda, senza box né riempimento (docs/project/pokedex-personal.md). -->
      <span
        class="text-4xl font-bold text-muted-foreground"
        aria-hidden="true"
      >?</span>
    {:else}
      <!-- Visto: sprite in grayscale; Catturato: sprite a colori (docs/project/pokedex-personal.md). -->
      <img
        src={spriteSrc}
        alt={name}
        width="64"
        height="64"
        class="h-16 w-16 object-contain {status === 'seen' ? 'grayscale opacity-90' : ''}"
        loading="lazy"
      />
    {/if}
    {#if !isUnseen}
      <div
        class="absolute top-0.5 right-0.5 flex shrink-0 items-center justify-center rounded-full border border-[var(--border-primary)] bg-[var(--bg-tertiary)]"
        title={status === "seen" ? "Visto" : "Catturato"}
        aria-hidden="true"
      >
        {#if status === "seen"}
          <Eye class="size-3.5 text-muted-foreground" aria-hidden="true" />
        {:else}
          <CheckCircle class="size-3.5 text-[var(--icon-success)]" aria-hidden="true" />
        {/if}
      </div>
    {/if}
  </div>
  <span class="mt-1 min-h-[1.25rem] text-center text-sm font-medium">{displayName}</span>
  <div class="mt-1 flex min-h-[1.5rem] flex-wrap items-center justify-center gap-1" aria-hidden="true">
    {#if isUnseen}
      <span class="text-xs text-muted-foreground">—</span>
    {:else}
      {#each types.filter(Boolean) as typeName (typeName)}
        <span
          class="rounded-md border border-[var(--border-primary)] bg-[var(--bg-primary)]/60 px-2 py-0.5 text-[11px] font-medium tracking-wide text-[var(--text-secondary)]"
        >
          {typeName}
        </span>
      {/each}
    {/if}
  </div>
</article>
