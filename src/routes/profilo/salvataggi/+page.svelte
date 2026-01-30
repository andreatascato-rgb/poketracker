<script lang="ts">
  import { onMount } from "svelte";
  import { listen, emit } from "@tauri-apps/api/event";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import { activeProfile } from "$lib/stores/profile";
  import { getSyncingPaths, addSyncing, removeSyncing, setWatchedCountFromSalvataggi } from "$lib/stores/sync.svelte";
  import * as savService from "$lib/services/sav";
  import * as pokedexService from "$lib/services/pokedex";
  import {
    formatErrorDetail,
    reportSystemError,
  } from "$lib/stores/error-archive";

  import { Button } from "$lib/components/ui/button";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
    CardAction,
  } from "$lib/components/ui/card";
  import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogFooter,
    DialogDescription,
  } from "$lib/components/ui/dialog";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import { EmptyState } from "$lib/components/ui/empty-state";
  import { Tooltip, TooltipContent, TooltipTrigger } from "$lib/components/ui/tooltip";
  import { toast } from "$lib/components/ui/sonner";
  import { FolderPlus, FolderOpen, Pencil, Trash2, CheckCircle, Loader2 } from "@lucide/svelte";

  /** Voce in tabella: da get_sav_entries (path, game, version, generation, updated_at). */
  type SaveEntry = { path: string; game: string; version: string; generation?: number; updated_at: string };
  let saves = $state<SaveEntry[]>([]);
  /** Path per cui il watcher è attivo (get_sav_watched_paths). */
  let watchedPaths = $state<string[]>([]);

  /** Set path in sync (reattivo via getter Svelte 5). */
  const syncingPathsSet = $derived(getSyncingPaths());

  /** Dialog riepilogo prima di aggiungere: path, game, version, generation (profilo da store). */
  let summaryOpen = $state(false);
  let summaryPath = $state("");
  let summaryGame = $state("");
  let summaryVersion = $state("");
  let summaryGeneration = $state(0);
  /** LanguageID raw dal save; usato per Archivio errori quando versione non determinata. */
  let summaryLanguageIdRaw = $state<number | null>(null);
  let summaryAdding = $state(false);

  /** True mentre il sidecar analizza il file (invoke detect_save_game_version). */
  let detectingSave = $state(false);

  /** Dialog modifica salvataggio: path attuale (da sostituire), nuovo path e metadati da detect. */
  let editOpen = $state(false);
  let editPath = $state("");
  let editPathNew = $state("");
  let editGame = $state("");
  let editVersion = $state("");
  let editGeneration = $state(0);
  let editAdding = $state(false);
  let editDetecting = $state(false);

  /** Dialog rimozione salvataggio: avviso + conferma prima di removeSave (anche offline). */
  let removeDialogOpen = $state(false);
  let removePath = $state("");
  let removeSubmitting = $state(false);

  /** Debounce per sav-file-changed: evita sync ripetute per lo stesso path in breve tempo. */
  const pendingSync = new Map<string, ReturnType<typeof setTimeout>>();
  const DEBOUNCE_MS = 1500;

  /** Carica la lista salvataggi e i path con watcher attivo. */
  async function loadSaves() {
    try {
      const [list, watched] = await Promise.all([
        savService.getSavEntries(),
        pokedexService.getSavWatchedPaths(),
      ]);
      saves = list ?? [];
      watchedPaths = watched ?? [];
      setWatchedCountFromSalvataggi(watchedPaths.length);
    } catch (err) {
      saves = [];
      watchedPaths = [];
      const msg = err instanceof Error ? err.message : String(err);
      reportSystemError({
        type: "Caricamento salvataggi fallito",
        detail: formatErrorDetail({ type: "Caricamento salvataggi fallito", message: msg }),
        toastMessage: msg,
      });
    }
  }

  /** Toggle watcher: attiva/disattiva sync per questo salvataggio. Se si attiva, prima sync_sav_now (spinner inline), poi set_sav_watched. */
  async function toggleWatcher(path: string) {
    const watched = watchedPaths.includes(path);
    if (watched) {
      try {
        await savService.setSavWatched(path, false);
        await loadSaves();
        toast.success("Osservazione disattivata.");
      } catch (e) {
        const msg = typeof e === "string" ? e : "Impossibile disattivare l'osservazione.";
        reportSystemError({
          type: "Disattivazione osservazione fallita",
          detail: formatErrorDetail({ type: "Disattivazione osservazione fallita", message: msg, path }),
          toastMessage: msg,
        });
      }
      return;
    }
    addSyncing(path);
    try {
      await savService.syncSavNow(path);
      await savService.setSavWatched(path, true);
      await loadSaves();
      await emit("sav-entries-changed");
      toast.success("Osservazione attivata.");
    } catch (e) {
      const msg = typeof e === "string" ? e : "Sincronizzazione non riuscita. Verifica che il file sia accessibile.";
      reportSystemError({
        type: "Sincronizzazione salvataggio fallita",
        detail: formatErrorDetail({ type: "Sincronizzazione salvataggio fallita", message: msg, path }),
        toastMessage: msg,
      });
      console.error("sync_sav_now/set_sav_watched failed:", e);
    } finally {
      removeSyncing(path);
    }
  }

  onMount(() => {
    loadSaves();
    let unlisten: (() => void) | undefined;
    listen<string>("sav-file-changed", (ev) => {
      const path = ev.payload;
      // Debounce: se arrivano più eventi per lo stesso path, aspetta DEBOUNCE_MS prima di syncare
      if (pendingSync.has(path)) {
        clearTimeout(pendingSync.get(path)!);
      }
      pendingSync.set(
        path,
        setTimeout(async () => {
          pendingSync.delete(path);
          addSyncing(path);
          try {
            await savService.touchSavEntryUpdatedAt(path);
            await pokedexService.syncPokedexFromWatchedSavsNow();
            await loadSaves();
          } catch (e) {
            const msg = e instanceof Error ? e.message : String(e);
            reportSystemError({
              type: "Aggiornamento salvataggio fallito",
              detail: formatErrorDetail({
                type: "Aggiornamento salvataggio fallito",
                message: msg,
                path,
              }),
              toastMessage: "Aggiornamento salvataggio fallito. Verifica che il file sia accessibile.",
            });
            console.error("sav-file-changed failed:", path, e);
          } finally {
            removeSyncing(path);
          }
        }, DEBOUNCE_MS)
      );
    }).then((fn) => (unlisten = fn));
    return () => {
      unlisten?.();
      // Cleanup pending timeouts
      for (const timeout of pendingSync.values()) {
        clearTimeout(timeout);
      }
      pendingSync.clear();
    };
  });

  /** Apri Sfoglia file .sav/.dsv → detect → dialog riepilogo → add_sav_entry. */
  async function startAddSave() {
    const selected = await openDialog({
      directory: false,
      multiple: false,
      filters: [
        { name: "Salvataggio Pokémon", extensions: ["sav", "dsv"] },
      ],
    });
    const path =
      selected == null ? null : Array.isArray(selected) ? selected[0] ?? null : selected;
    if (path == null) return;

    detectingSave = true;
    try {
      const detected = await savService.detectSaveGameVersion(path);
      summaryPath = path;
      summaryGame = detected.game ?? "";
      summaryVersion = detected.version ?? "";
      summaryGeneration = typeof detected.generation === "number" ? detected.generation : 0;
      summaryLanguageIdRaw =
        typeof detected.languageIdRaw === "number" ? detected.languageIdRaw : null;
      const versionEmpty = !summaryVersion || summaryVersion === "—";
      if (versionEmpty && summaryLanguageIdRaw !== null) {
        reportSystemError({
          type: "Versione non determinata",
          detail: formatErrorDetail({
            type: "Versione non determinata",
            message: "Impossibile determinare la versione del salvataggio.",
            path: summaryPath,
            languageIdRaw: summaryLanguageIdRaw,
          }),
          toastMessage: "Versione non determinata per questo salvataggio. Controlla Impostazioni → Errori.",
        });
      }
      summaryOpen = true;
    } catch (e) {
      const msg = typeof e === "string" ? e : "Impossibile riconoscere il file. Verifica che sia un salvataggio Pokémon valido.";
      reportSystemError({
        type: "Riconoscimento salvataggio fallito",
        detail: formatErrorDetail({ type: "Riconoscimento salvataggio fallito", message: msg, path }),
        toastMessage: msg,
      });
    } finally {
      detectingSave = false;
    }
  }

  /** Conferma aggiunta: add_sav_entry, ricarica lista. Il Pokédex si aggiorna solo quando attivi l’osservazione (watcher). */
  async function confirmAddSave() {
    if (!summaryPath) return;
    summaryAdding = true;
    try {
      await savService.addSavEntry({
        path: summaryPath,
        game: summaryGame,
        version: summaryVersion,
        generation: summaryGeneration > 0 ? summaryGeneration : undefined,
      });
      summaryOpen = false;
      summaryPath = "";
      summaryGame = "";
      summaryVersion = "";
      summaryGeneration = 0;
      summaryLanguageIdRaw = null;
      await loadSaves();
      toast.success("Salvataggio aggiunto. Attiva l’osservazione per aggiornare il Pokédex.");
    } catch (e) {
      const msg = typeof e === "string" ? e : "Impossibile aggiungere il salvataggio.";
      reportSystemError({
        type: "Aggiunta salvataggio fallita",
        detail: formatErrorDetail({
          type: "Aggiunta salvataggio fallita",
          message: msg,
          path: summaryPath,
        }),
        toastMessage: msg,
      });
      console.error("add_sav_entry failed:", e);
    } finally {
      summaryAdding = false;
    }
  }

  /** Apre il dialog di conferma rimozione per il path indicato. */
  function openRemoveDialog(path: string) {
    removePath = path;
    removeDialogOpen = true;
  }

  function closeRemoveDialog() {
    if (!removeSubmitting) {
      removeDialogOpen = false;
      removePath = "";
    }
  }

  /** Rimuove la voce, ricalcola Pokedex dai sav rimanenti (o svuota se ultimo) e notifica gli altri componenti. */
  async function removeSave(path: string) {
    try {
      await savService.removeSavEntry(path);
      await pokedexService.syncPokedexFromWatchedSavsNow();
      await loadSaves();
      await emit("sav-entries-changed");
      toast.success("Salvataggio rimosso.");
    } catch (e) {
      const msg = typeof e === "string" ? e : "Impossibile rimuovere il salvataggio.";
      reportSystemError({
        type: "Rimozione salvataggio fallita",
        detail: formatErrorDetail({
          type: "Rimozione salvataggio fallita",
          message: msg,
          path,
        }),
        toastMessage: msg,
      });
      console.error("remove_sav_entry failed:", e);
    }
  }

  /** Conferma rimozione: removeSave(removePath) e chiude il dialog. */
  async function confirmRemoveSave() {
    if (!removePath) return;
    removeSubmitting = true;
    try {
      await removeSave(removePath);
      removeDialogOpen = false;
      removePath = "";
    } finally {
      removeSubmitting = false;
    }
  }

  /** Apre file picker per scegliere un nuovo file e avvia il flusso modifica (detect → dialog → update_sav_entry). */
  async function startEditSave(save: SaveEntry) {
    const selected = await openDialog({
      directory: false,
      multiple: false,
      filters: [
        { name: "Salvataggio Pokémon", extensions: ["sav", "dsv"] },
      ],
    });
    const path =
      selected == null ? null : Array.isArray(selected) ? selected[0] ?? null : selected;
    if (path == null) return;
    editPath = save.path;
    editDetecting = true;
    try {
      const detected = await savService.detectSaveGameVersion(path);
      editPathNew = path;
      editGame = detected.game ?? "";
      editVersion = detected.version ?? "";
      editGeneration = typeof detected.generation === "number" ? detected.generation : 0;
      editOpen = true;
    } catch (e) {
      const msg = typeof e === "string" ? e : "Impossibile riconoscere il file. Verifica che sia un salvataggio Pokémon valido.";
      reportSystemError({
        type: "Riconoscimento salvataggio fallito",
        detail: formatErrorDetail({ type: "Riconoscimento salvataggio fallito", message: msg, path }),
        toastMessage: msg,
      });
    } finally {
      editDetecting = false;
    }
  }

  /** Conferma modifica: update_sav_entry, eventuale sync se era osservato, ricarica lista. */
  async function confirmEditSave() {
    if (!editPath || !editPathNew) return;
    editAdding = true;
    const wasWatched = watchedPaths.includes(editPath);
    try {
      await savService.updateSavEntry({
        oldPath: editPath,
        newPath: editPathNew,
        game: editGame,
        version: editVersion,
        generation: editGeneration > 0 ? editGeneration : undefined,
      });
      if (wasWatched) {
        addSyncing(editPathNew);
        try {
          await savService.syncSavNow(editPathNew);
        } finally {
          removeSyncing(editPathNew);
        }
      }
      editOpen = false;
      editPath = "";
      editPathNew = "";
      editGame = "";
      editVersion = "";
      editGeneration = 0;
      await loadSaves();
      await emit("sav-entries-changed");
      toast.success("Salvataggio aggiornato.");
    } catch (e) {
      const msg = typeof e === "string" ? e : "Impossibile aggiornare il salvataggio.";
      reportSystemError({
        type: "Aggiornamento salvataggio fallito",
        detail: formatErrorDetail({
          type: "Aggiornamento salvataggio fallito",
          message: msg,
          path: editPathNew,
        }),
        toastMessage: msg,
      });
      console.error("update_sav_entry failed:", e);
    } finally {
      editAdding = false;
    }
  }

  /** Formatta data ISO in locale italiano (gg/mm/aaaa). */
  function formatDate(iso: string): string {
    try {
      return new Date(iso).toLocaleDateString("it-IT", {
        day: "2-digit",
        month: "2-digit",
        year: "numeric",
      });
    } catch {
      return "—";
    }
  }
</script>

{#if saves.length === 0}
  <!-- Empty state: nessun salvataggio configurato -->
  <div class="flex flex-col min-h-[calc(100vh-96px)] w-full">
    <Card
      role="region"
      aria-labelledby="salvataggi-title"
      class="w-full min-w-0 flex flex-1 flex-col min-h-0"
    >
      <CardHeader>
        <CardTitle
          id="salvataggi-title"
          class="text-xl font-semibold min-h-9"
        >
          Salvataggi
        </CardTitle>
      </CardHeader>
      <CardContent class="flex flex-1 flex-col items-center justify-center min-h-0">
        <EmptyState
          title="Aggiungi un salvataggio"
          description="Scegli un file .sav o .dsv da monitorare. L'app rileverà automaticamente gioco e versione."
          icon={FolderPlus}
          ctaLabel="Salvataggio"
          ctaIcon={FolderOpen}
          onCtaClick={startAddSave}
          ariaLabel="Stato vuoto: nessun salvataggio configurato"
        />
      </CardContent>
    </Card>
  </div>
{:else}
  <!-- Tabella salvataggi -->
  <Card
    role="region"
    aria-labelledby="salvataggi-title"
    class="w-full min-w-0"
  >
    <CardHeader>
      <CardTitle
        id="salvataggi-title"
        class="text-xl font-semibold min-h-9"
      >
        Salvataggi
      </CardTitle>
      <CardAction>
        <Button
          type="button"
          variant="outline"
          size="default"
          onclick={startAddSave}
          aria-label="Aggiungi salvataggio"
          class="shrink-0"
        >
          <FolderPlus class="size-4" aria-hidden="true" />
          Salvataggio
        </Button>
      </CardAction>
    </CardHeader>
    <CardContent>
      <div class="overflow-x-auto rounded-md border border-[var(--border-primary)]" role="region" aria-label="Tabella salvataggi">
        <table
          class="w-full min-w-[320px] text-sm"
          aria-label="Elenco salvataggi"
        >
          <thead class="border-b border-[var(--border-primary)] bg-[var(--bg-tertiary)]">
            <tr>
              <th scope="col" class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider">
                Versione
              </th>
              <th scope="col" class="min-w-[20ch] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider">
                Gioco
              </th>
              <th scope="col" class="min-w-[2.5rem] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider">
                Gen
              </th>
              <th scope="col" class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider">
                Aggiornamento
              </th>
              <th scope="col" class="min-w-[2.5rem] px-2 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider">
                Sync
              </th>
              <th scope="col" class="min-w-[6.5rem] px-2 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider">
                Azioni
              </th>
            </tr>
          </thead>
          <tbody>
            {#each saves as save (save.path)}
              {@const isWatched = watchedPaths.includes(save.path)}
              <tr class="border-b border-border last:border-b-0 hover:bg-muted/30">
                <td class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground">
                  {save.version || "—"}
                </td>
                <td class="min-w-[20ch] px-4 py-3 text-center">
                  {save.game || "—"}
                </td>
                <td class="min-w-[2.5rem] px-4 py-3 text-center text-muted-foreground">
                  {(save.generation ?? 0) > 0 ? save.generation : "—"}
                </td>
                <td class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground">
                  {formatDate(save.updated_at)}
                </td>
                <td class="min-w-[2.5rem] px-2 py-3 text-center" role="cell">
                  {#if syncingPathsSet.has(save.path)}
                    <!-- Spinner mentre la sync è in corso -->
                    <div
                      class="inline-flex items-center justify-center size-8"
                      role="status"
                      aria-label="Sincronizzazione in corso"
                    >
                      <Loader2
                        class="size-4 animate-spin text-muted-foreground"
                        aria-hidden="true"
                      />
                    </div>
                  {:else}
                    <!-- Icona CheckCircle cliccabile per toggle watcher -->
                    <Tooltip>
                      <TooltipTrigger>
                        <button
                          type="button"
                          class="inline-flex items-center justify-center rounded-md text-[var(--text-secondary)] hover:bg-[var(--hover-bg)] active:bg-[var(--pressed-bg)] size-8 cursor-pointer disabled:pointer-events-none"
                          aria-label={isWatched ? "Watcher attivo: sta monitorando e aggiornando i dati" : "Attiva watcher per questo salvataggio"}
                          aria-pressed={isWatched}
                          onclick={() => toggleWatcher(save.path)}
                        >
                          <CheckCircle
                            class="size-4 {isWatched ? 'text-[var(--icon-success)]' : 'text-muted-foreground'}"
                            aria-hidden="true"
                          />
                        </button>
                      </TooltipTrigger>
                      <TooltipContent side="top" sideOffset={6}>
                        {isWatched ? "Watcher attivo: sta monitorando e aggiornando i dati" : "Attiva watcher per questo salvataggio"}
                      </TooltipContent>
                    </Tooltip>
                  {/if}
                </td>
                <td class="min-w-[6.5rem] px-2 py-3 text-center" role="cell">
                  <div class="flex items-center justify-center gap-0">
                    <Tooltip>
                      <TooltipTrigger>
                        <Button
                          type="button"
                          variant="ghost-muted"
                          size="icon-sm"
                          aria-label="Modifica salvataggio"
                          onclick={() => startEditSave(save)}
                        >
                          <Pencil class="size-4 text-[var(--text-secondary)]" aria-hidden="true" />
                        </Button>
                      </TooltipTrigger>
                      <TooltipContent side="top" sideOffset={6}>Modifica</TooltipContent>
                    </Tooltip>
                    <Tooltip>
                      <TooltipTrigger>
                        <Button
                          type="button"
                          variant="ghost-muted"
                          size="icon-sm"
                          aria-label="Rimuovi salvataggio"
                          onclick={() => openRemoveDialog(save.path)}
                        >
                          <Trash2 class="size-4 text-[var(--icon-destructive)]" aria-hidden="true" />
                        </Button>
                      </TooltipTrigger>
                      <TooltipContent side="top" sideOffset={6}>Rimuovi</TooltipContent>
                    </Tooltip>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </CardContent>
  </Card>
{/if}

{#if detectingSave || editDetecting}
  <!-- Overlay per analisi file (detect_save_game_version) in aggiunta o modifica -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
    aria-live="polite"
    aria-busy="true"
    role="status"
    aria-label="Analisi salvataggio in corso"
  >
    <div class="flex flex-col items-center gap-4 rounded-lg border border-[var(--border-primary)] bg-[var(--bg-primary)] px-8 py-6 shadow-lg">
      <Loader2 class="size-10 shrink-0 animate-spin text-[var(--text-primary)]" aria-hidden="true" />
      <p class="text-sm font-medium text-[var(--text-primary)]">
        Analisi salvataggio in corso…
      </p>
    </div>
  </div>
{/if}

<!-- Dialog riepilogo: percorso, allenatore, gioco, versione → Aggiungi / Annulla -->
<Dialog bind:open={summaryOpen}>
  <DialogContent class="sm:max-w-md" aria-describedby="summary-desc">
    <DialogHeader>
      <DialogTitle>Riepilogo salvataggio</DialogTitle>
      <DialogDescription id="summary-desc">
        Verifica i dati prima di aggiungere il salvataggio al profilo.
      </DialogDescription>
    </DialogHeader>
    <dl class="mt-4 text-sm space-y-3 min-w-0 overflow-hidden">
      <div class="grid grid-cols-2 gap-x-6 gap-y-3" role="presentation">
        <div>
          <dt class="text-muted-foreground text-xs font-medium uppercase tracking-wider">Gioco</dt>
          <dd class="mt-0.5">{summaryGame || "—"}</dd>
        </div>
        <div>
          <dt class="text-muted-foreground text-xs font-medium uppercase tracking-wider">Versione</dt>
          <dd class="mt-0.5">{summaryVersion || "—"}</dd>
        </div>
        <div>
          <dt class="text-muted-foreground text-xs font-medium uppercase tracking-wider">Generazione</dt>
          <dd class="mt-0.5">{summaryGeneration > 0 ? summaryGeneration : "—"}</dd>
        </div>
        <div>
          <dt class="text-muted-foreground text-xs font-medium uppercase tracking-wider">Allenatore</dt>
          <dd class="mt-0.5">{$activeProfile?.name ?? "—"}</dd>
        </div>
      </div>
      <div class="min-w-0 overflow-hidden">
        <dt class="text-muted-foreground text-xs font-medium uppercase tracking-wider">Percorso</dt>
        <dd class="mt-0.5 font-mono text-xs break-all" title={summaryPath}>{summaryPath}</dd>
      </div>
    </dl>
    <DialogFooter class="mt-6">
      <Button type="button" variant="outline" onclick={() => (summaryOpen = false)}>
        Annulla
      </Button>
      <Button
        type="button"
        onclick={confirmAddSave}
        disabled={summaryAdding}
      >
        {summaryAdding ? "Aggiunta…" : "Aggiungi"}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

<!-- Dialog modifica salvataggio: nuovo path e metadati → Modifica / Annulla -->
<Dialog bind:open={editOpen}>
  <DialogContent class="sm:max-w-md" aria-describedby="edit-desc">
    <DialogHeader>
      <DialogTitle>Modifica salvataggio</DialogTitle>
      <DialogDescription id="edit-desc">
        Sostituisci il file con un altro. Il nuovo file verrà associato a questo profilo.
      </DialogDescription>
    </DialogHeader>
    <dl class="mt-4 text-sm space-y-3 min-w-0 overflow-hidden">
      <div class="grid grid-cols-2 gap-x-6 gap-y-3" role="presentation">
        <div>
          <dt class="text-muted-foreground text-xs font-medium uppercase tracking-wider">Gioco</dt>
          <dd class="mt-0.5">{editGame || "—"}</dd>
        </div>
        <div>
          <dt class="text-muted-foreground text-xs font-medium uppercase tracking-wider">Versione</dt>
          <dd class="mt-0.5">{editVersion || "—"}</dd>
        </div>
        <div>
          <dt class="text-muted-foreground text-xs font-medium uppercase tracking-wider">Generazione</dt>
          <dd class="mt-0.5">{editGeneration > 0 ? editGeneration : "—"}</dd>
        </div>
      </div>
      <div class="min-w-0 overflow-hidden">
        <dt class="text-muted-foreground text-xs font-medium uppercase tracking-wider">Nuovo percorso</dt>
        <dd class="mt-0.5 font-mono text-xs break-all" title={editPathNew}>{editPathNew}</dd>
      </div>
    </dl>
    <DialogFooter class="mt-6">
      <Button type="button" variant="outline" onclick={() => (editOpen = false)}>
        Annulla
      </Button>
      <Button
        type="button"
        onclick={confirmEditSave}
        disabled={editAdding}
      >
        {editAdding ? "Aggiornamento…" : "Modifica"}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

<!-- AlertDialog Rimuovi salvataggio: avviso + conferma (anche offline); Pokedex aggiornato o svuotato. -->
<AlertDialog.Root bind:open={removeDialogOpen}>
  <AlertDialog.Content aria-labelledby="remove-dialog-title" aria-describedby="remove-dialog-desc">
    <AlertDialog.Title id="remove-dialog-title">Rimuovi salvataggio?</AlertDialog.Title>
    <AlertDialog.Description id="remove-dialog-desc">
      Il salvataggio verrà rimosso dalla lista. Il Pokédex sarà aggiornato in base ai salvataggi rimanenti oppure svuotato se è l'ultimo. Questa azione non è reversibile.
    </AlertDialog.Description>
    <AlertDialog.Footer>
      <Button
        type="button"
        variant="outline"
        onclick={closeRemoveDialog}
        disabled={removeSubmitting}
      >
        Annulla
      </Button>
      <Button
        type="button"
        variant="destructive"
        disabled={removeSubmitting}
        onclick={confirmRemoveSave}
      >
        {removeSubmitting ? "Rimozione…" : "Rimuovi"}
      </Button>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
