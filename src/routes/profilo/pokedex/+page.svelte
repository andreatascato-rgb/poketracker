<script lang="ts">
  import { onMount } from "svelte";
  import { get } from "svelte/store";
  import { goto } from "$app/navigation";
  import { listen } from "@tauri-apps/api/event";
  import {
    getSpeciesList,
    POKEDEX_GEN4_MAX,
    type PokedexStatus,
  } from "$lib/data/pokedex-species";
  import PokedexTile from "$lib/components/pokedex/PokedexTile.svelte";
  import { activeProfile, profilesLoaded, loadProfiles } from "$lib/stores/profile";
  import * as pokedexService from "$lib/services/pokedex";
  import { reportSystemError } from "$lib/stores/error-archive";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
  } from "$lib/components/ui/card";
  import PokedexStats from "$lib/components/pokedex/PokedexStats.svelte";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
  } from "$lib/components/ui/dropdown-menu";
  import { EmptyState } from "$lib/components/ui/empty-state";
  import { toast } from "$lib/components/ui/sonner";
  import { UserCircle, Home, FolderOpen, Search, ChevronDown, Check } from "@lucide/svelte";

  const species = getSpeciesList();

  /** Range specie per generazione (National Dex Gen 1–4). */
  const GEN_RANGES: Readonly<Record<number, [number, number]>> = {
    1: [1, 151],
    2: [152, 251],
    3: [252, 386],
    4: [387, 493],
  };

  /** Opzioni dropdown gioco: value "" = tutti, "1"–"4" = range come generazione. */
  const GAME_OPTIONS: ReadonlyArray<{ value: string; label: string }> = [
    { value: "", label: "Tutti i giochi" },
    { value: "1", label: "Rosso/Blu/Verde" },
    { value: "2", label: "Oro/Argento/Cristallo" },
    { value: "3", label: "Rubino/Zaffiro/Smeraldo" },
    { value: "4", label: "Diamante/Perla/Platino" },
  ];

  /** Opzioni dropdown generazione: value "0" = tutte, "1"–"4" = Gen 1–4. */
  const GEN_OPTIONS: ReadonlyArray<{ value: string; label: string }> = [
    { value: "0", label: "Tutte" },
    { value: "1", label: "Gen 1 (1–151)" },
    { value: "2", label: "Gen 2 (152–251)" },
    { value: "3", label: "Gen 3 (252–386)" },
    { value: "4", label: "Gen 4 (387–493)" },
  ];

  function idInRange(id: number, genValue: string): boolean {
    if (!genValue || genValue === "0") return true;
    const n = parseInt(genValue, 10);
    const range = GEN_RANGES[n as keyof typeof GEN_RANGES];
    if (!range) return true;
    return id >= range[0] && id <= range[1];
  }

  /** Filtro testuale per nome: filtra solo la lista mostrata in UI. */
  let nameFilter = $state("");
  /** Filtro generazione: "0" = tutte, "1"–"4" = Gen 1–4. Sincronizzato con gameFilter. */
  let generationFilter = $state("0");
  /** Filtro gioco: "" = tutti, "1"–"4" = stesso range della generazione. Sincronizzato con generationFilter. */
  let gameFilter = $state("");

  /** Selezione generazione: aggiorna anche il filtro gioco per coerenza (stessa gen = stessi giochi). */
  function selectGeneration(value: string) {
    generationFilter = value;
    gameFilter = value === "0" ? "" : value;
  }
  /** Selezione gioco: aggiorna anche il filtro generazione per coerenza. */
  function selectGame(value: string) {
    gameFilter = value;
    generationFilter = value === "" ? "0" : value;
  }

  const generationLabel = $derived(
    GEN_OPTIONS.find((o) => o.value === generationFilter)?.label ?? "Tutte"
  );
  const gameLabel = $derived(
    GAME_OPTIONS.find((o) => o.value === gameFilter)?.label ?? "Tutti i giochi"
  );

  /** Filtro effettivo: generazione e gioco sono sempre sincronizzati, basta un solo range. */
  const filteredSpecies = $derived(
    species.filter((s) => {
      if (
        nameFilter.trim() &&
        !s.name.toLowerCase().includes(nameFilter.trim().toLowerCase())
      )
        return false;
      if (!idInRange(s.id, generationFilter)) return false;
      return true;
    })
  );

  const defaultUnseen = Object.fromEntries(
    Array.from({ length: 493 }, (_, i) => [i + 1, "unseen" as PokedexStatus])
  );

  /** Stato per profilo: caricato da get_pokedex_state; assenza = unseen. */
  let statusById = $state<Record<number, PokedexStatus>>({ ...defaultUnseen });

  /** true se per questo profilo è stata fatta almeno una sync (pokedex_state ha almeno una riga). */
  let hasPokedexData = $state(false);

  /** true fino al primo completamento di loadPokedexState per il profilo corrente (evita flash empty state). */
  let pokedexLoading = $state(true);

  const caughtCount = $derived(
    Object.values(statusById).filter((s) => s === "caught").length
  );
  const seenCount = $derived(
    Object.values(statusById).filter((s) => s === "seen" || s === "caught").length
  );

  /** Statistiche sul sottoinsieme filtrato (nome + generazione): per card animata. */
  const filteredCaught = $derived(
    filteredSpecies.filter((s) => statusById[s.id] === "caught").length
  );
  const filteredSeen = $derived(
    filteredSpecies.filter(
      (s) =>
        statusById[s.id] === "seen" || statusById[s.id] === "caught"
    ).length
  );
  const filteredTotal = $derived(filteredSpecies.length);

  async function loadPokedexState(profileId: string) {
    pokedexLoading = true;
    try {
      const entries = await pokedexService.getPokedexState(profileId);
      hasPokedexData = entries.length > 0;
      const next = { ...defaultUnseen };
      for (const e of entries) {
        const raw = (e.status ?? "").trim().toLowerCase();
        if (raw === "unseen" || raw === "seen" || raw === "caught") {
          next[e.species_id] = raw as PokedexStatus;
        }
      }
      statusById = next;
    } catch (err) {
      reportSystemError({
        type: "Caricamento Pokedex fallito",
        detail: err instanceof Error ? err.message : String(err),
      });
      hasPokedexData = false;
      statusById = { ...defaultUnseen };
    } finally {
      pokedexLoading = false;
    }
  }

  /** Bootstrap: se il Pokedex è vuoto ma ci sono watcher attivi, sync da tutti i sav watchati (risolve "2 watcher e 0 pokemon"). */
  async function bootstrapIfWatched() {
    const profile = get(activeProfile);
    if (!profile || hasPokedexData) return;
    try {
      const watched = await pokedexService.getSavWatchedPaths();
      if ((watched?.length ?? 0) > 0) {
        await pokedexService.syncPokedexFromWatchedSavsNow();
        await loadPokedexState(profile.id);
        toast.success("Pokedex aggiornato dai salvataggi osservati.");
      }
    } catch (e) {
      reportSystemError({
        type: "Bootstrap Pokedex da salvataggi fallito",
        detail: e instanceof Error ? e.message : String(e),
      });
    }
  }

  onMount(() => {
    loadProfiles();
    const unsub = activeProfile.subscribe((profile) => {
      if (profile) {
        loadPokedexState(profile.id).then(() => bootstrapIfWatched());
      } else {
        pokedexLoading = false;
        hasPokedexData = false;
        statusById = { ...defaultUnseen };
      }
    });
    let unlisten: (() => void) | undefined;
    let unlistenEntries: (() => void) | undefined;
    listen<string>("sav-file-changed", async () => {
      const profile = get(activeProfile);
      if (!profile) return;
      try {
        await pokedexService.syncPokedexFromWatchedSavsNow();
        await loadPokedexState(profile.id);
      } catch (_) {
        void loadPokedexState(profile.id);
      }
    }).then((fn) => (unlisten = fn));
    listen("sav-entries-changed", () => {
      const profile = get(activeProfile);
      if (profile) void loadPokedexState(profile.id);
    }).then((fn) => (unlistenEntries = fn));
    return () => {
      unsub();
      unlisten?.();
      unlistenEntries?.();
    };
  });
</script>

<!-- Area main: Pokedex personale -->
{#if !profilesLoaded}
  <div
    class="flex min-h-[calc(100vh-96px)] flex-col items-center justify-center text-muted-foreground text-sm"
    role="status"
    aria-label="Caricamento profilo"
  >
    Caricamento…
  </div>
{:else if !$activeProfile}
  <!-- Empty state: nessun profilo selezionato (full-page come Profili) -->
  <div
    class="flex min-h-[calc(100vh-96px)] flex-col items-center justify-center"
    role="region"
    aria-label="Stato vuoto: nessun profilo selezionato"
  >
    <EmptyState
      title="Seleziona un profilo"
      description="Il Pokedex è legato al profilo allenatore. Seleziona un profilo dalla barra in alto o creane uno dalle Impostazioni."
      icon={UserCircle}
      ctaLabel="Vai alla Home"
      ctaIcon={Home}
      onCtaClick={() => goto("/")}
      ariaLabel="Stato vuoto: nessun profilo selezionato"
    />
  </div>
{:else if pokedexLoading}
  <div
    class="flex min-h-[calc(100vh-96px)] flex-col items-center justify-center text-muted-foreground text-sm"
    role="status"
    aria-label="Caricamento Pokedex"
  >
    Caricamento…
  </div>
{:else if !hasPokedexData}
  <!-- Empty state: Pokedex non ancora popolato (stesso layout di Salvataggi: Card + EmptyState in CardContent) -->
  <div class="flex flex-col min-h-[calc(100vh-96px)] w-full">
    <Card
      role="region"
      aria-labelledby="pokedex-title"
      class="w-full min-w-0 flex flex-1 flex-col min-h-0"
    >
      <CardHeader>
        <CardTitle id="pokedex-title" class="text-xl font-semibold min-h-9">
          Pokedex
        </CardTitle>
      </CardHeader>
      <CardContent class="flex flex-1 flex-col items-center justify-center min-h-0">
        <EmptyState
          title="Popola il Pokedex"
          description="Aggiungi salvataggi da Salvataggi e attiva il watcher sui file: il Pokedex si aggiorna da solo quando rileva modifiche."
          icon={FolderOpen}
          ctaLabel="Vai a Salvataggi"
          ctaIcon={FolderOpen}
          onCtaClick={() => goto("/profilo/salvataggi")}
          ariaLabel="Stato vuoto: Pokedex non ancora popolato"
        />
      </CardContent>
    </Card>
  </div>
{:else}
  <div class="space-y-6">
    <Card role="region" aria-labelledby="pokedex-title">
      <CardHeader class="flex flex-col gap-3">
        <CardTitle id="pokedex-title" class="text-xl font-semibold min-h-9">
          Pokedex
        </CardTitle>
        <div class="flex w-full min-w-0 flex-wrap items-stretch gap-4">
          <div class="flex min-w-0 shrink-0">
            <PokedexStats
              caught={filteredCaught}
              seen={filteredSeen}
              total={filteredTotal}
              class="h-full min-h-0"
            />
          </div>
          <figure
            class="min-w-[220px] flex-1 rounded-xl border border-[var(--border-primary)] bg-[var(--bg-tertiary)] px-4 py-3"
            role="group"
            aria-label="Filtri Pokedex"
          >
            <Label for="pokedex-name-filter" class="sr-only">Filtra per nome</Label>
            <div class="relative flex items-center">
              <Search
                class="pointer-events-none absolute left-3 size-4 text-muted-foreground"
                aria-hidden="true"
              />
              <Input
                id="pokedex-name-filter"
                type="text"
                bind:value={nameFilter}
                placeholder="Filtra per nome"
                class="h-9 pl-9"
                aria-label="Filtra per nome"
              />
            </div>
            <div class="mt-2 grid min-w-0 grid-cols-1 gap-2 sm:grid-cols-2">
              <div class="min-w-0 flex flex-col">
                <Label for="pokedex-gen-trigger" class="mb-1.5 block text-xs font-medium text-muted-foreground">
                  Generazione
                </Label>
                <DropdownMenu>
                  <DropdownMenuTrigger
                    id="pokedex-gen-trigger"
                    class="border-input bg-background ring-offset-background flex h-9 w-full min-w-0 items-center justify-between rounded-md border pl-3 pr-3 py-1.5 text-left text-sm shadow-xs transition-[color,box-shadow] outline-none hover:bg-[var(--hover-bg)] focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 data-[state=open]:border-ring/50"
                    aria-label="Filtra per generazione"
                    aria-haspopup="listbox"
                  >
                    <span class="truncate">{generationLabel}</span>
                    <ChevronDown class="ml-2 size-4 shrink-0 text-muted-foreground" aria-hidden="true" />
                  </DropdownMenuTrigger>
                  <DropdownMenuContent
                    class="pokedex-filter-dropdown min-w-[10rem] rounded-md border border-[var(--border-primary)] bg-[var(--bg-secondary)] p-1 shadow-[0_4px_12px_rgba(0,0,0,0.4)]"
                    side="bottom"
                    align="start"
                    sideOffset={4}
                  >
                    {#each GEN_OPTIONS as opt (opt.value)}
                      <DropdownMenuItem
                        class="pokedex-filter-item cursor-pointer rounded px-3 py-2 text-sm font-normal transition-[background-color] duration-200 active:!bg-[var(--pressed-bg)] data-[highlighted]:bg-[var(--hover-bg)] data-[highlighted]:outline-none {opt.value === generationFilter
                          ? 'bg-[var(--active-bg)] text-white data-[highlighted]:bg-[var(--active-bg)]'
                          : 'text-[var(--text-primary)]'}"
                        onclick={() => selectGeneration(opt.value)}
                      >
                        {#if opt.value === generationFilter}
                          <Check class="size-4 shrink-0" aria-hidden="true" />
                        {:else}
                          <span class="size-4 shrink-0" aria-hidden="true"></span>
                        {/if}
                        <span class="flex-1">{opt.label}</span>
                      </DropdownMenuItem>
                    {/each}
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>
              <div class="min-w-0 flex flex-col">
                <Label for="pokedex-game-trigger" class="mb-1.5 block text-xs font-medium text-muted-foreground">
                  Gioco
                </Label>
                <DropdownMenu>
                  <DropdownMenuTrigger
                    id="pokedex-game-trigger"
                    class="border-input bg-background ring-offset-background flex h-9 w-full min-w-0 items-center justify-between rounded-md border pl-3 pr-3 py-1.5 text-left text-sm shadow-xs transition-[color,box-shadow] outline-none hover:bg-[var(--hover-bg)] focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 data-[state=open]:border-ring/50"
                    aria-label="Filtra per gioco"
                    aria-haspopup="listbox"
                  >
                    <span class="truncate">{gameLabel}</span>
                    <ChevronDown class="ml-2 size-4 shrink-0 text-muted-foreground" aria-hidden="true" />
                  </DropdownMenuTrigger>
                  <DropdownMenuContent
                    class="pokedex-filter-dropdown min-w-[10rem] rounded-md border border-[var(--border-primary)] bg-[var(--bg-secondary)] p-1 shadow-[0_4px_12px_rgba(0,0,0,0.4)]"
                    side="bottom"
                    align="start"
                    sideOffset={4}
                  >
                    {#each GAME_OPTIONS as opt (opt.value)}
                      <DropdownMenuItem
                        class="pokedex-filter-item cursor-pointer rounded px-3 py-2 text-sm font-normal transition-[background-color] duration-200 active:!bg-[var(--pressed-bg)] data-[highlighted]:bg-[var(--hover-bg)] data-[highlighted]:outline-none {opt.value === gameFilter
                          ? 'bg-[var(--active-bg)] text-white data-[highlighted]:bg-[var(--active-bg)]'
                          : 'text-[var(--text-primary)]'}"
                        onclick={() => selectGame(opt.value)}
                      >
                        {#if opt.value === gameFilter}
                          <Check class="size-4 shrink-0" aria-hidden="true" />
                        {:else}
                          <span class="size-4 shrink-0" aria-hidden="true"></span>
                        {/if}
                        <span class="flex-1">{opt.label}</span>
                      </DropdownMenuItem>
                    {/each}
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>
            </div>
          </figure>
        </div>
      </CardHeader>
      <CardContent>
        <div
          class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
          role="list"
          aria-label="Elenco specie Pokedex"
        >
          {#each filteredSpecies as { id, name } (id)}
            <div role="listitem">
              <PokedexTile
                {id}
                {name}
                status={statusById[id] ?? "unseen"}
              />
            </div>
          {/each}
        </div>
      </CardContent>
    </Card>
  </div>
{/if}
