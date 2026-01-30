<script lang="ts">
  import { onMount } from "svelte";
  import {
    errorArchiveEntries,
    loadErrorArchiveEntries,
    removeErrorEntry,
    type ErrorArchiveEntry,
  } from "$lib/stores/error-archive";
  import { Button } from "$lib/components/ui/button";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardDescription,
    CardContent,
  } from "$lib/components/ui/card";
  import { Tooltip, TooltipContent, TooltipTrigger } from "$lib/components/ui/tooltip";
  import { EmptyState } from "$lib/components/ui/empty-state";
  import * as Dialog from "$lib/components/ui/dialog";
  import { AlertCircle, Eye, Trash2, Copy, Loader2 } from "@lucide/svelte";
  import { toast } from "$lib/components/ui/sonner";

  let detailDialogOpen = $state(false);
  let detailEntry = $state<ErrorArchiveEntry | null>(null);
  let loading = $state(true);
  let loadError = $state(false);
  let removingId = $state<string | null>(null);
  let copied = $state(false);

  function formatDateTime(iso: string): string {
    try {
      return new Date(iso).toLocaleString("it-IT", {
        day: "2-digit",
        month: "2-digit",
        year: "numeric",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      });
    } catch {
      return "—";
    }
  }

  function openDetail(entry: ErrorArchiveEntry) {
    detailEntry = entry;
    detailDialogOpen = true;
    copied = false;
  }

  function closeDetail() {
    detailDialogOpen = false;
    detailEntry = null;
    copied = false;
  }

  async function copyDetail() {
    if (!detailEntry || copied) return;
    try {
      await navigator.clipboard.writeText(detailEntry.detail);
      toast.success("Log copiato negli appunti.");
      copied = true;
      setTimeout(() => (copied = false), 1500);
    } catch {
      toast.error("Copia non riuscita.");
    }
  }

  async function handleRemove(id: string) {
    if (removingId) return;
    removingId = id;
    try {
      await removeErrorEntry(id);
      if (detailEntry?.id === id) closeDetail();
    } catch (e) {
      toast.error("Eliminazione non riuscita. Riprova.");
      console.error("remove_error_archive_entry failed:", e);
    } finally {
      removingId = null;
    }
  }

  async function load() {
    loading = true;
    loadError = false;
    try {
      await loadErrorArchiveEntries();
    } catch (e) {
      console.error("get_error_archive_entries failed:", e);
      loadError = true;
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    load();
  });
</script>

<!-- Empty state obbligatorio (ui-patterns-applied): registro errori può essere vuoto. Stessa struttura di Salvataggi: Card di sezione + CardContent con EmptyState (noCard = niente card interna). -->
{#if loading}
  <div class="flex flex-col min-h-[calc(100vh-96px)] w-full">
    <Card
      role="region"
      aria-labelledby="errori-title"
      class="w-full min-w-0 flex flex-1 flex-col min-h-0"
    >
      <CardHeader>
        <CardTitle id="errori-title" class="text-xl font-semibold min-h-9">
          Errori
        </CardTitle>
      </CardHeader>
      <CardContent
        class="flex flex-1 flex-col items-center justify-center min-h-0 gap-3"
        aria-busy="true"
        aria-live="polite"
        aria-label="Caricamento registro errori"
      >
        <Loader2
          class="size-8 shrink-0 animate-spin text-[var(--text-secondary)] motion-reduce:animate-none"
          aria-hidden="true"
        />
        <p class="text-sm text-muted-foreground">Caricamento…</p>
      </CardContent>
    </Card>
  </div>
{:else if loadError}
  <div class="flex flex-col min-h-[calc(100vh-96px)] w-full">
    <Card
      role="region"
      aria-labelledby="errori-title"
      class="w-full min-w-0 flex flex-1 flex-col min-h-0"
    >
      <CardHeader>
        <CardTitle id="errori-title" class="text-xl font-semibold min-h-9">
          Errori
        </CardTitle>
      </CardHeader>
      <CardContent class="flex flex-1 flex-col items-center justify-center min-h-0 gap-4">
        <p class="text-sm text-muted-foreground text-center max-w-[40ch]">
          Impossibile caricare il registro. Riprova o controlla la console per i dettagli.
        </p>
        <Button type="button" variant="outline" onclick={load} aria-label="Riprova caricamento">
          Riprova
        </Button>
      </CardContent>
    </Card>
  </div>
{:else if $errorArchiveEntries.length === 0}
  <div class="flex flex-col min-h-[calc(100vh-96px)] w-full">
    <Card
      role="region"
      aria-labelledby="errori-title"
      class="w-full min-w-0 flex flex-1 flex-col min-h-0"
    >
      <CardHeader>
        <CardTitle id="errori-title" class="text-xl font-semibold min-h-9">
          Errori
        </CardTitle>
      </CardHeader>
      <CardContent class="flex flex-1 flex-col items-center justify-center min-h-0">
        <EmptyState
          title="Nessun errore registrato"
          description="Quando l'app segnalerà un problema di sistema, troverai qui l'elenco per visualizzarlo, copiare il log (es. per supporto o assistente AI) ed eliminarlo."
          icon={AlertCircle}
          ariaLabel="Stato vuoto: nessun errore in registro"
          noCard
        />
      </CardContent>
    </Card>
  </div>
{:else}
  <Card
    role="region"
    aria-labelledby="errori-title"
    class="w-full min-w-0"
  >
    <CardHeader>
      <CardTitle id="errori-title" class="text-xl font-semibold min-h-9">
        Errori
      </CardTitle>
      <CardDescription class="max-w-[66ch]">
        Log di sistema per supporto e assistente AI. Apri una voce per copiare il dettaglio completo. Rimuovi le voci risolte per tenere ordinato il registro.
      </CardDescription>
    </CardHeader>
    <CardContent>
      <div
        class="overflow-x-auto rounded-md border border-[var(--border-primary)]"
        role="region"
        aria-label="Tabella errori registrati"
      >
        <table
          class="w-full min-w-[320px] text-sm"
          aria-label="Elenco errori registrati"
        >
          <thead class="border-b border-[var(--border-primary)] bg-[var(--bg-tertiary)]">
            <tr>
              <th
                scope="col"
                class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider"
              >
                Data
              </th>
              <th
                scope="col"
                class="min-w-[18ch] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider"
              >
                Tipo
              </th>
              <th
                scope="col"
                class="min-w-[8rem] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider"
              >
                Azioni
              </th>
            </tr>
          </thead>
          <tbody>
            {#each $errorArchiveEntries as entry (entry.id)}
              {@const isRemoving = removingId === entry.id}
              <tr class="border-b border-border last:border-b-0 hover:bg-muted/30">
                <td class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground">
                  {formatDateTime(entry.at)}
                </td>
                <td class="min-w-[18ch] px-4 py-3 text-center">
                  {entry.type}
                </td>
                <td class="min-w-[8rem] px-4 py-3 text-center" role="cell">
                  {#if isRemoving}
                    <div class="flex items-center justify-center" aria-busy="true" aria-label="Eliminazione in corso">
                      <Loader2 class="size-4 animate-spin text-[var(--text-secondary)] motion-reduce:animate-none" aria-hidden="true" />
                    </div>
                  {:else}
                    <div class="flex items-center justify-center gap-1">
                      <Tooltip>
                        <TooltipTrigger>
                          <Button
                            type="button"
                            variant="ghost-muted"
                            size="icon-sm"
                            onclick={() => openDetail(entry)}
                            aria-label="Apri dettaglio e copia log"
                          >
                            <Eye class="size-4 text-[var(--text-secondary)]" aria-hidden="true" />
                          </Button>
                        </TooltipTrigger>
                        <TooltipContent side="top" sideOffset={6}>Apri dettaglio (log copiabile)</TooltipContent>
                      </Tooltip>
                      <Tooltip>
                        <TooltipTrigger>
                          <Button
                            type="button"
                            variant="ghost-muted"
                            size="icon-sm"
                            onclick={() => handleRemove(entry.id)}
                            aria-label="Elimina voce dal registro"
                          >
                            <Trash2 class="size-4 text-[var(--icon-destructive)]" aria-hidden="true" />
                          </Button>
                        </TooltipTrigger>
                        <TooltipContent side="top" sideOffset={6}>Elimina dal registro</TooltipContent>
                      </Tooltip>
                    </div>
                  {/if}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </CardContent>
  </Card>

  <Dialog.Root bind:open={detailDialogOpen}>
    <Dialog.Content aria-label="Dettaglio errore" class="max-w-2xl max-h-[85vh] flex flex-col">
      <Dialog.Header>
        <Dialog.Title>{detailEntry?.type ?? "Dettaglio errore"}</Dialog.Title>
        <Dialog.Description>
          Log completo da copiare e incollare (es. per supporto o assistente AI).
        </Dialog.Description>
      </Dialog.Header>
      <div class="flex-1 min-h-0 overflow-hidden flex flex-col gap-3">
        {#if detailEntry}
          <pre
            class="flex-1 min-h-[8rem] overflow-auto rounded-md border border-[var(--border-primary)] bg-[var(--bg-tertiary)] p-4 text-xs text-left whitespace-pre-wrap font-mono"
            aria-label="Contenuto log"
          >{detailEntry.detail}</pre>
        {/if}
      </div>
      <Dialog.Footer>
        <Button type="button" variant="outline" onclick={closeDetail}>
          Chiudi
        </Button>
        <Button
          type="button"
          variant="default"
          onclick={copyDetail}
          disabled={copied}
          aria-label={copied ? "Log copiato" : "Copia log negli appunti"}
        >
          {#if copied}
            Copiato!
          {:else}
            <Copy class="size-4" aria-hidden="true" />
            Copia log
          {/if}
        </Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
{/if}
