<script lang="ts">
  import { onMount } from "svelte";
  import { get } from "svelte/store";
  import { goto, afterNavigate } from "$app/navigation";
  import { listen } from "@tauri-apps/api/event";
  import { getSpeciesList, type PokedexStatus } from "$lib/data/pokedex-species";
  import PokedexTile from "$lib/components/pokedex/PokedexTile.svelte";
  import { activeProfile, profilesLoaded, loadProfiles } from "$lib/stores/profile";
  import * as pokedexService from "$lib/services/pokedex";
  import { reportSystemError } from "$lib/stores/error-archive";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardDescription,
    CardContent,
  } from "$lib/components/ui/card";
  import EmptyState from "$lib/components/ui/empty-state/empty-state.svelte";
  import { toast } from "$lib/components/ui/sonner";
  import { UserCircle, Home, FolderOpen } from "@lucide/svelte";

  const species = getSpeciesList();

  const defaultUnseen = Object.fromEntries(
    Array.from({ length: 493 }, (_, i) => [i + 1, "unseen" as PokedexStatus])
  );

  /** Stato per profilo: caricato da get_pokedex_state; assenza = unseen. */
  let statusById = $state<Record<number, PokedexStatus>>({ ...defaultUnseen });

  /** true se per questo profilo è stata fatta almeno una sync (pokedex_state ha almeno una riga). */
  let hasPokedexData = $state(false);

  const caughtCount = $derived(
    Object.values(statusById).filter((s) => s === "caught").length
  );
  const seenCount = $derived(
    Object.values(statusById).filter((s) => s === "seen" || s === "caught").length
  );

  async function loadPokedexState(profileId: string) {
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

  afterNavigate(({ to }) => {
    if (to?.url?.pathname === "/profilo/pokedex") {
      const profile = get(activeProfile);
      if (profile) void loadPokedexState(profile.id);
    }
  });

  onMount(() => {
    loadProfiles();
    const unsub = activeProfile.subscribe((profile) => {
      if (profile) {
        loadPokedexState(profile.id).then(() => bootstrapIfWatched());
      } else {
        hasPokedexData = false;
        statusById = { ...defaultUnseen };
      }
    });
    let unlisten: (() => void) | undefined;
    listen<string>("sav-file-changed", () => {
      const profile = get(activeProfile);
      if (profile) void loadPokedexState(profile.id);
    }).then((fn) => (unlisten = fn));
    return () => {
      unsub();
      unlisten?.();
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
    />
  </div>
{:else if !hasPokedexData}
  <div
    class="flex min-h-[calc(100vh-96px)] flex-col items-center justify-center"
    role="region"
    aria-label="Stato vuoto: Pokedex non ancora popolato"
  >
    <EmptyState
      title="Popola il Pokedex"
      description="Aggiungi salvataggi da Salvataggi e attiva il watcher sui file: il Pokedex si aggiorna da solo quando rileva modifiche."
      icon={FolderOpen}
      ctaLabel="Vai a Salvataggi"
      ctaIcon={FolderOpen}
      onCtaClick={() => goto("/profilo/salvataggi")}
      ariaLabel="Stato vuoto: Pokedex non ancora popolato"
    />
  </div>
{:else}
  <div class="space-y-6">
    <Card role="region" aria-labelledby="pokedex-title">
      <CardHeader>
        <CardTitle id="pokedex-title" class="text-xl font-semibold min-h-9">
          Pokedex
        </CardTitle>
        <CardDescription class="text-sm text-muted-foreground">
          Registro Pokémon del profilo. Si aggiorna da solo quando i salvataggi osservati cambiano. Catturati: {caughtCount} / 493 · Visti: {seenCount} / 493
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div
          class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
          role="list"
          aria-label="Elenco specie Pokedex"
        >
          {#each species as { id, name } (id)}
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
