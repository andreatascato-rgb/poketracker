<script lang="ts">
  import { CheckCircle } from "@lucide/svelte";
  import type { PokedexStatus } from "$lib/data/pokedex-species";
  import { getSpeciesTypes, getTypeAbbreviationIta } from "$lib/data/pokedex-types";
  import type { TypeName } from "$lib/data/pokedex-types";

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
  const typeList = $derived(types.filter((t): t is TypeName => Boolean(t)));

  /** Slug per variabile CSS tipo (es. Fuoco → fuoco). Coerente con token in app.css. */
  function typeSlug(t: TypeName): string {
    return t.toLowerCase();
  }

  /** Doppio tipo: abbreviazioni ITA (NOR, FUO, …); tipo singolo: nome completo in maiuscolo. Nome completo in title. */
  const typeLabels = $derived(
    typeList.length === 2
      ? typeList.map((t) => getTypeAbbreviationIta(t))
      : typeList.map((t) => t.toUpperCase())
  );
</script>

<article
  aria-label="{displayName} ({numberLabel}) — {status === 'unseen' ? 'Non visto' : status === 'seen' ? 'Visto' : 'Catturato'}"
  class="flex h-[13rem] w-full min-w-0 flex-col items-center overflow-hidden rounded-lg border border-[var(--border-primary)] bg-[var(--bg-tertiary)] px-3 pt-3 pb-2 transition-[background-color] duration-200 hover:bg-[var(--hover-bg)]"
>
  <span class="shrink-0 text-xs text-muted-foreground">{numberLabel}</span>
  <div class="relative mt-1 flex h-24 w-24 shrink-0 items-center justify-center">
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
        width="96"
        height="96"
        class="h-24 w-24 object-contain {status === 'seen' ? 'grayscale opacity-90' : ''}"
        loading="lazy"
      />
    {/if}
    {#if status === "caught"}
      <div
        class="absolute top-0.5 right-0.5 flex shrink-0 items-center justify-center rounded-full border border-[var(--border-primary)] bg-[var(--bg-tertiary)]"
        title="Catturato"
        aria-hidden="true"
      >
        <CheckCircle class="size-3.5 text-[var(--icon-success)]" aria-hidden="true" />
      </div>
    {/if}
  </div>
  <span
    class="mt-1 min-h-[1.25rem] min-w-0 shrink line-clamp-2 text-center text-sm font-medium"
    title={displayName}
  >{displayName}</span>
  <!-- Doppio tipo: etichette troncate (max caratteri) su una riga; tipo singolo: nome completo -->
  <div
    class="mt-1 flex min-h-[1.5rem] min-w-0 shrink flex-nowrap items-center justify-center gap-1 overflow-hidden"
    aria-hidden="true"
  >
    {#if isUnseen}
      <span class="text-xs text-muted-foreground">—</span>
    {:else}
      {#each typeList as typeName, i (typeName + i)}
        {@const slug = typeSlug(typeName)}
        {@const label = typeLabels[i]}
        <span
          class="pokedex-type-badge shrink-0 rounded-full px-2 py-1 text-[10px] font-semibold uppercase leading-none"
          style="background: var(--type-{slug}-bg); color: var(--type-{slug}-fg); border: 1px solid var(--type-badge-border); box-shadow: var(--type-badge-shadow); text-shadow: var(--type-badge-text-shadow);"
          title={typeName}
        >
          {label}
        </span>
      {/each}
    {/if}
  </div>
</article>
