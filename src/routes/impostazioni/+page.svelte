<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { profiles, activeProfile, loadProfiles } from "$lib/stores/profile";
  import type { Profile } from "$lib/stores/profile";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Dialog from "$lib/components/ui/dialog";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardDescription,
    CardContent,
  } from "$lib/components/ui/card";
  import { UserPlus, Pencil, Trash2 } from "@lucide/svelte";

  let createDialogOpen = $state(false);
  let createNome = $state("");
  let createError = $state("");
  let createSubmitting = $state(false);

  let renameDialogOpen = $state(false);
  let renameProfileId = $state<string | null>(null);
  let renameNome = $state("");
  let renameError = $state("");
  let renameSubmitting = $state(false);

  function openCreate() {
    createNome = "";
    createError = "";
    createDialogOpen = true;
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
      await invoke("create_profile", { name: trimmed });
      createDialogOpen = false;
      await loadProfiles();
    } catch (err) {
      createError = err instanceof Error ? err.message : String(err);
    } finally {
      createSubmitting = false;
    }
  }

  function closeCreateDialog() {
    createDialogOpen = false;
    createError = "";
  }

  function openRename(p: Profile) {
    renameProfileId = p.id;
    renameNome = p.name;
    renameError = "";
    renameDialogOpen = true;
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
      await invoke("rename_profile", { id: renameProfileId, newName: trimmed });
      renameDialogOpen = false;
      await loadProfiles();
    } catch (err) {
      renameError = err instanceof Error ? err.message : String(err);
    } finally {
      renameSubmitting = false;
    }
  }

  function closeRenameDialog() {
    renameDialogOpen = false;
    renameError = "";
    renameProfileId = null;
  }

  async function deleteProfile(p: Profile) {
    if ($profiles.length <= 1) return;
    if (!confirm(`Eliminare il profilo "${p.name}"? L'operazione non può essere annullata.`)) return;
    try {
      await invoke("delete_profile", { id: p.id });
      await loadProfiles();
    } catch (err) {
      alert(err instanceof Error ? err.message : String(err));
    }
  }
</script>

<div class="flex flex-col gap-6">
  <Card
    role="region"
    aria-labelledby="impostazioni-profili-title"
    class="w-full"
  >
    <CardHeader class="!flex !flex-col gap-1.5">
      <div class="flex w-full flex-nowrap items-center justify-between gap-4">
        <CardTitle
          id="impostazioni-profili-title"
          class="flex min-h-8 shrink-0 items-center text-lg font-semibold"
        >
          Profili
        </CardTitle>
        <Button
          type="button"
          variant="outline"
          size="sm"
          onclick={openCreate}
          aria-label="Crea nuovo allenatore"
          class="shrink-0"
        >
          <UserPlus class="size-4" aria-hidden="true" />
          Nuovo allenatore
        </Button>
      </div>
      <CardDescription class="text-sm text-muted-foreground">
        Gestisci i profili allenatore. Ogni profilo ha dati e cartella salvataggi separati. Il profilo attivo si cambia dalla barra in alto.
      </CardDescription>
    </CardHeader>
    <CardContent>
      <div class="overflow-x-auto rounded-md border">
        <table class="w-full min-w-[320px] text-left text-sm" aria-label="Elenco profili">
          <thead>
            <tr class="border-b bg-muted/50">
              <th scope="col" class="px-4 py-3 font-medium">Nome</th>
              <th scope="col" class="px-4 py-3 font-medium">Stato</th>
              <th scope="col" class="px-4 py-3 font-medium text-right">Azioni</th>
            </tr>
          </thead>
          <tbody>
            {#each $profiles as profile (profile.id)}
              <tr class="border-b last:border-b-0 hover:bg-muted/30">
                <td class="px-4 py-3 font-medium">{profile.name}</td>
                <td class="px-4 py-3 text-muted-foreground">
                  {profile.id === $activeProfile?.id ? "Attivo" : "—"}
                </td>
                <td class="px-4 py-3 text-right">
                  <div class="flex items-center justify-end gap-1">
                    <Button
                      type="button"
                      variant="ghost"
                      size="icon-sm"
                      onclick={() => openRename(profile)}
                      aria-label="Modifica profilo {profile.name}"
                    >
                      <Pencil class="size-4" aria-hidden="true" />
                    </Button>
                    <Button
                      type="button"
                      variant="ghost"
                      size="icon-sm"
                      class="text-destructive hover:text-destructive"
                      onclick={() => deleteProfile(profile)}
                      disabled={$profiles.length <= 1}
                      aria-label="Elimina profilo {profile.name}"
                      title={$profiles.length <= 1 ? "Deve restare almeno un profilo" : "Elimina profilo"}
                    >
                      <Trash2 class="size-4" aria-hidden="true" />
                    </Button>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </CardContent>
  </Card>
</div>

<!-- Dialog Nuovo allenatore -->
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

<!-- Dialog Rinomina profilo -->
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
