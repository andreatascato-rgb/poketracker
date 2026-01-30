<script lang="ts">
  import { onMount } from "svelte";
  import { profiles, activeProfile, loadProfiles } from "$lib/stores/profile";
  import type { Profile } from "$lib/stores/profile";
  import * as profileService from "$lib/services/profile";
  import * as savService from "$lib/services/sav";
  import { reportSystemError } from "$lib/stores/error-archive";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
    CardAction,
  } from "$lib/components/ui/card";
  import { Tooltip, TooltipContent, TooltipTrigger } from "$lib/components/ui/tooltip";
  import { EmptyState } from "$lib/components/ui/empty-state";
  import { toast } from "$lib/components/ui/sonner";
  import { UserPlus, Pencil, Trash2, CheckCircle } from "@lucide/svelte";

  /** Numero salvataggi del profilo attivo (get_sav_entries). */
  let savCount = $state(0);

  let createDialogOpen = $state(false);
  let createNome = $state("");
  let createError = $state("");
  let createSubmitting = $state(false);

  let renameDialogOpen = $state(false);
  let renameProfileId = $state<string | null>(null);
  let renameNome = $state("");
  let renameError = $state("");
  let renameSubmitting = $state(false);

  let deleteDialogOpen = $state(false);
  let deleteProfile = $state<Profile | null>(null);
  let deleteTypedName = $state("");
  let deleteError = $state("");
  let deleteSubmitting = $state(false);

  const canDeleteConfirm = $derived(
    deleteProfile !== null && deleteTypedName.trim() === deleteProfile.name
  );

  function openCreate() {
    createNome = "";
    createError = "";
    createDialogOpen = true;
  }

  function closeCreateDialog() {
    createDialogOpen = false;
    createError = "";
  }

  async function submitCreate(e: Event) {
    e.preventDefault();
    createError = "";
    const trimmed = createNome.trim();
    if (!trimmed) {
      createError = "Il nome non può essere vuoto.";
      return;
    }
    createSubmitting = true;
    try {
      await profileService.createProfile(trimmed);
      createDialogOpen = false;
      await loadProfiles();
      toast.success("Profilo creato.");
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      createError = msg;
      toast.error(msg);
      reportSystemError({ type: "Creazione profilo fallita", detail: msg });
    } finally {
      createSubmitting = false;
    }
  }

  function openRename(p: Profile) {
    renameProfileId = p.id;
    renameNome = p.name;
    renameError = "";
    renameDialogOpen = true;
  }

  function closeRenameDialog() {
    renameDialogOpen = false;
    renameError = "";
    renameProfileId = null;
  }

  async function submitRename(e: Event) {
    e.preventDefault();
    if (!renameProfileId) return;
    renameError = "";
    const trimmed = renameNome.trim();
    if (!trimmed) {
      renameError = "Il nome non può essere vuoto.";
      return;
    }
    renameSubmitting = true;
    try {
      await profileService.renameProfile(renameProfileId, trimmed);
      renameDialogOpen = false;
      await loadProfiles();
      toast.success("Profilo rinominato.");
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      renameError = msg;
      toast.error(msg);
      reportSystemError({ type: "Rinomina profilo fallita", detail: msg });
    } finally {
      renameSubmitting = false;
    }
  }

  function openDelete(p: Profile) {
    deleteProfile = p;
    deleteTypedName = "";
    deleteError = "";
    deleteDialogOpen = true;
  }

  function closeDelete() {
    deleteDialogOpen = false;
    deleteProfile = null;
    deleteTypedName = "";
    deleteError = "";
  }

  async function submitDelete() {
    if (!deleteProfile || !canDeleteConfirm) return;
    deleteError = "";
    deleteSubmitting = true;
    try {
      await profileService.deleteProfile(deleteProfile.id);
      closeDelete();
      await loadProfiles();
      toast.success("Profilo eliminato.");
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      deleteError = msg;
      toast.error(msg);
      reportSystemError({ type: "Eliminazione profilo fallita", detail: msg });
    } finally {
      deleteSubmitting = false;
    }
  }

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

  async function loadSavCount() {
    try {
      const list = await savService.getSavEntries();
      savCount = list?.length ?? 0;
    } catch {
      savCount = 0;
    }
  }

  onMount(() => {
    loadSavCount();
  });
</script>

<!-- Empty state obbligatorio (ui-patterns-applied): lista profili può essere vuota. -->
{#if $profiles.length === 0}
  <div class="flex min-h-[calc(100vh-96px)] flex-col items-center justify-center w-full" role="region" aria-label="Stato vuoto: nessun profilo">
    <EmptyState
      title="Crea il tuo primo allenatore"
      description="Aggiungi un profilo per iniziare a tracciare salvataggi e Pokedex. Potrai creare più profili (es. uno per gioco) e attivare quello in uso."
      icon={UserPlus}
      ctaLabel="Crea nuovo allenatore"
      ctaIcon={UserPlus}
      onCtaClick={openCreate}
      ariaLabel="Stato vuoto: nessun profilo"
    />
  </div>
{:else}
<Card
    role="region"
    aria-labelledby="profili-title"
    class="w-full min-w-0"
  >
    <CardHeader>
      <CardTitle id="profili-title" class="text-xl font-semibold min-h-9">
        Profili
      </CardTitle>
      <CardAction>
        <Button
          type="button"
          variant="outline"
          size="default"
          onclick={openCreate}
          aria-label="Crea nuovo allenatore"
          class="shrink-0"
        >
          <UserPlus class="size-4" aria-hidden="true" />
          Nuovo allenatore
        </Button>
      </CardAction>
    </CardHeader>
    <CardContent>
      <div class="overflow-x-auto rounded-md border border-[var(--border-primary)]" role="region" aria-label="Tabella profili">
        <table
          class="w-full min-w-[320px] text-sm"
          aria-label="Elenco profili"
        >
          <thead class="border-b border-[var(--border-primary)] bg-[var(--bg-tertiary)]">
            <tr>
              <th scope="col" class="min-w-[14ch] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider">
                Nome
              </th>
              <th scope="col" class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider">
                Salvataggi
              </th>
              <th scope="col" class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider">
                Caricato
              </th>
              <th scope="col" class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider">
                Aggiornato
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
            {#each $profiles as profile (profile.id)}
            <tr class="border-b border-border last:border-b-0 hover:bg-muted/30">
              <td class="min-w-[14ch] px-4 py-3 text-center">
                {profile.name}
              </td>
              <td class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground" aria-label={profile.id === $activeProfile?.id ? `${savCount} salvataggi` : "Salvataggi"}>
                {profile.id === $activeProfile?.id ? savCount : "—"}
              </td>
              <td class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground">
                {formatDate(profile.created_at)}
              </td>
              <td class="min-w-[12ch] px-4 py-3 text-center text-muted-foreground">
                {formatDate(profile.updated_at)}
              </td>
              <td class="min-w-[2.5rem] px-2 py-3 text-center" aria-label={profile.id === $activeProfile?.id ? "Attivo" : "Sync"}>
                <Tooltip>
                  <TooltipTrigger>
                    <span class="inline-flex items-center justify-center size-8" aria-hidden="true">
                      {#if profile.id === $activeProfile?.id}
                        <CheckCircle class="size-4 text-muted-foreground" />
                      {:else}
                        <span class="text-muted-foreground">—</span>
                      {/if}
                    </span>
                  </TooltipTrigger>
                  <TooltipContent side="top" sideOffset={6}>
                    {profile.id === $activeProfile?.id ? "Attivo" : "—"}
                  </TooltipContent>
                </Tooltip>
              </td>
              <td class="min-w-[6.5rem] px-2 py-3 text-center" role="cell">
                <div class="flex items-center justify-center gap-0">
                  <Tooltip>
                    <TooltipTrigger>
                      <Button
                        type="button"
                        variant="ghost-muted"
                        size="icon-sm"
                        onclick={() => openRename(profile)}
                        aria-label="Modifica profilo {profile.name}"
                      >
                        <Pencil class="size-4 text-[var(--text-secondary)]" aria-hidden="true" />
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent side="top" sideOffset={6}>Modifica profilo {profile.name}</TooltipContent>
                  </Tooltip>
                  <Tooltip>
                    <TooltipTrigger>
                      <Button
                        type="button"
                        variant="ghost-muted"
                        size="icon-sm"
                        onclick={() => openDelete(profile)}
                        aria-label="Elimina profilo {profile.name}"
                      >
                        <Trash2 class="size-4 text-[var(--icon-destructive)]" aria-hidden="true" />
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent side="top" sideOffset={6}>Elimina profilo {profile.name}</TooltipContent>
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

<!-- Dialog Nuovo allenatore (minimale) -->
<Dialog.Root bind:open={createDialogOpen}>
  <Dialog.Content aria-label="Nuovo allenatore">
    <form id="impostazioni-create-trainer-form" class="grid gap-4" onsubmit={submitCreate}>
      <div class="grid gap-2">
        <Label for="impostazioni-nome">Nome allenatore</Label>
        <Input
          id="impostazioni-nome"
          type="text"
          bind:value={createNome}
          required
          aria-required="true"
          aria-invalid={!!createError}
          aria-describedby={createError ? "impostazioni-nome-error" : undefined}
          placeholder="Es. Rosso"
          disabled={createSubmitting}
          autofocus
        />
        {#if createError}
          <p id="impostazioni-nome-error" class="text-sm text-destructive" role="alert">
            {createError}
          </p>
        {/if}
      </div>
    </form>
    <Dialog.Footer>
      <Button type="button" variant="outline" onclick={closeCreateDialog} disabled={createSubmitting}>
        Annulla
      </Button>
      <Button
        type="submit"
        form="impostazioni-create-trainer-form"
        variant="default"
        disabled={createSubmitting}
      >
        {createSubmitting ? "Creazione…" : "Crea"}
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Dialog Rinomina profilo (minimale) -->
<Dialog.Root bind:open={renameDialogOpen}>
  <Dialog.Content aria-label="Rinomina profilo">
    <form id="impostazioni-rename-form" class="grid gap-4" onsubmit={submitRename}>
      <div class="grid gap-2">
        <Label for="impostazioni-rename-nome">Nome allenatore</Label>
        <Input
          id="impostazioni-rename-nome"
          type="text"
          bind:value={renameNome}
          required
          aria-required="true"
          aria-invalid={!!renameError}
          aria-describedby={renameError ? "impostazioni-rename-error" : undefined}
          placeholder="Es. Rosso"
          disabled={renameSubmitting}
          autofocus
        />
        {#if renameError}
          <p id="impostazioni-rename-error" class="text-sm text-destructive" role="alert">
            {renameError}
          </p>
        {/if}
      </div>
    </form>
    <Dialog.Footer>
      <Button type="button" variant="outline" onclick={closeRenameDialog} disabled={renameSubmitting}>
        Annulla
      </Button>
      <Button
        type="submit"
        form="impostazioni-rename-form"
        variant="default"
        disabled={renameSubmitting}
      >
        {renameSubmitting ? "Salvataggio…" : "Salva"}
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- AlertDialog Elimina profilo -->
<AlertDialog.Root bind:open={deleteDialogOpen}>
  <AlertDialog.Content aria-labelledby="delete-dialog-title" aria-describedby="delete-dialog-desc">
    <AlertDialog.Title id="delete-dialog-title">Elimina profilo?</AlertDialog.Title>
    <AlertDialog.Description id="delete-dialog-desc">
      Il profilo «{deleteProfile?.name ?? ""}» e tutti i dati ad esso associati saranno eliminati definitivamente. Questa azione non è reversibile. Per confermare, scrivi il nome del profilo qui sotto.
    </AlertDialog.Description>
    <div class="grid gap-2 py-2">
      <Label for="impostazioni-delete-confirm">Nome del profilo</Label>
      <Input
        id="impostazioni-delete-confirm"
        type="text"
        bind:value={deleteTypedName}
        placeholder={deleteProfile?.name ?? ""}
        disabled={deleteSubmitting}
        aria-invalid={!!deleteError}
        aria-describedby={deleteError ? "delete-dialog-error" : undefined}
        autofocus
      />
      {#if deleteError}
        <p id="delete-dialog-error" class="text-sm text-destructive" role="alert">
          {deleteError}
        </p>
      {/if}
    </div>
    <AlertDialog.Footer>
      <Button
        type="button"
        variant="outline"
        onclick={closeDelete}
        disabled={deleteSubmitting}
      >
        Annulla
      </Button>
      <Button
        type="button"
        variant="destructive"
        disabled={!canDeleteConfirm || deleteSubmitting}
        onclick={submitDelete}
      >
        {deleteSubmitting ? "Eliminazione…" : "Elimina"}
      </Button>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
