<script lang="ts">
  import { goto } from "$app/navigation";
  import {
    errorArchiveEntries,
    removeErrorEntry,
    type ErrorArchiveEntry,
  } from "$lib/stores/error-archive";
  import { Button } from "$lib/components/ui/button";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
  } from "$lib/components/ui/card";
  import { Tooltip, TooltipContent, TooltipTrigger } from "$lib/components/ui/tooltip";
  import { EmptyState } from "$lib/components/ui/empty-state";
  import * as Dialog from "$lib/components/ui/dialog";
  import { AlertCircle, Eye, Trash2, Copy, User } from "@lucide/svelte";
  import { toast } from "$lib/components/ui/sonner";

  let detailDialogOpen = $state(false);
  let detailEntry = $state<ErrorArchiveEntry | null>(null);

  function formatDateTime(iso: string): string {
    try {
      return new Date(iso).toLocaleString("it-IT", {
        day: "2-digit",
        month: "2-digit",
        year: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch {
      return "—";
    }
  }

  function openDetail(entry: ErrorArchiveEntry) {
    detailEntry = entry;
    detailDialogOpen = true;
  }

  function closeDetail() {
    detailDialogOpen = false;
    detailEntry = null;
  }

  async function copyDetail() {
    if (!detailEntry) return;
    try {
      await navigator.clipboard.writeText(detailEntry.detail);
      toast.success("Log copiato negli appunti.");
    } catch {
      toast.error("Copia non riuscita.");
    }
  }

  function handleRemove(id: string) {
    removeErrorEntry(id);
    if (detailEntry?.id === id) closeDetail();
  }

  function goToAllenatore() {
    goto("/profilo");
  }
</script>

{#if $errorArchiveEntries.length === 0}
  <div
    class="flex min-h-[calc(100vh-96px)] flex-col items-center justify-center"
    role="region"
    aria-label="Registro errori vuoto"
  >
    <EmptyState
      title="Nessun errore registrato"
      description="Quando l'app segnalerà un problema di sistema, troverai qui l'elenco per visualizzarlo, copiare il log (es. per supporto o assistente AI) ed eliminarlo."
      icon={AlertCircle}
      ctaLabel="Vai all'Allenatore"
      ctaIcon={User}
      onCtaClick={goToAllenatore}
      ariaLabel="Stato vuoto: nessun errore in registro"
    />
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
                class="px-4 py-3 text-left min-w-[12ch] text-muted-foreground text-xs font-semibold uppercase tracking-wider"
              >
                Data
              </th>
              <th
                scope="col"
                class="px-4 py-3 text-left min-w-[18ch] text-muted-foreground text-xs font-semibold uppercase tracking-wider"
              >
                Tipo
              </th>
              <th
                scope="col"
                class="px-4 py-3 text-center min-w-[8rem] text-muted-foreground text-xs font-semibold uppercase tracking-wider"
              >
                Azioni
              </th>
            </tr>
          </thead>
          <tbody>
            {#each $errorArchiveEntries as entry (entry.id)}
              <tr class="border-b border-border last:border-b-0 hover:bg-muted/30">
                <td class="px-4 py-3 text-muted-foreground">
                  {formatDateTime(entry.at)}
                </td>
                <td class="px-4 py-3">
                  {entry.type}
                </td>
                <td class="px-4 py-3 min-w-[8rem]" role="cell">
                  <div class="flex items-center justify-center gap-0">
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
            class="flex-1 overflow-auto rounded-md border border-[var(--border-primary)] bg-[var(--bg-tertiary)] p-4 text-xs text-left whitespace-pre-wrap font-mono"
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
          aria-label="Copia log negli appunti"
        >
          <Copy class="size-4" aria-hidden="true" />
          Copia log
        </Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
{/if}
