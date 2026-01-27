<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { profiles, profilesLoaded, loadProfiles } from "$lib/stores/profile";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Dialog from "$lib/components/ui/dialog";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardDescription,
    CardFooter,
  } from "$lib/components/ui/card";
  import { UserPlus } from "@lucide/svelte";

  let dialogOpen = $state(false);
  let nome = $state("");
  let error = $state("");
  let submitting = $state(false);

  function createTrainer() {
    nome = "";
    error = "";
    dialogOpen = true;
  }

  async function submitCreate(e: Event) {
    e.preventDefault();
    error = "";
    const trimmed = nome.trim();
    if (!trimmed) {
      error = "Il nome non può essere vuoto.";
      return;
    }
    submitting = true;
    try {
      await invoke("create_profile", { name: trimmed });
      dialogOpen = false;
      await loadProfiles();
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      submitting = false;
    }
  }

  function closeDialog() {
    dialogOpen = false;
    error = "";
  }
</script>

{#if $profilesLoaded && $profiles.length === 0}
  <div
    class="flex min-h-[calc(100vh-96px)] flex-col items-center justify-center"
    role="region"
    aria-label="Stato vuoto: nessun profilo"
  >
    <Card class="w-full max-w-md text-center">
      <div class="flex justify-center px-6" aria-hidden="true">
        <UserPlus
          class="size-12 text-muted-foreground/70"
          strokeWidth={1.5}
          aria-hidden="true"
        />
      </div>
      <CardHeader class="gap-2 pb-2">
        <CardTitle class="text-xl font-semibold tracking-tight">
          Crea il tuo primo allenatore
        </CardTitle>
        <CardDescription class="mx-auto max-w-[66ch] leading-relaxed">
          Aggiungi un profilo per iniziare a tracciare i tuoi salvataggi Pokémon.
        </CardDescription>
      </CardHeader>
      <CardFooter class="justify-center pt-2">
        <Button
          type="button"
          variant="outline"
          size="default"
          onclick={createTrainer}
          aria-label="Crea nuovo allenatore"
        >
          <UserPlus class="size-4" aria-hidden="true" />
          Crea nuovo allenatore
        </Button>
      </CardFooter>
    </Card>
  </div>
{:else}
  <!-- Area main vuota quando ci sono profili -->
{/if}

<Dialog.Root bind:open={dialogOpen}>
  <Dialog.Content>
    <form id="create-trainer-form" class="grid gap-4" onsubmit={submitCreate}>
      <div class="grid gap-2">
        <Label for="nome">Nome allenatore</Label>
        <Input
          id="nome"
          type="text"
          bind:value={nome}
          required
          aria-required="true"
          aria-invalid={!!error}
          aria-describedby={error ? "nome-error" : undefined}
          placeholder="Es. Rosso"
          disabled={submitting}
          autofocus
        />
        {#if error}
          <p id="nome-error" class="text-sm text-destructive" role="alert">
            {error}
          </p>
        {/if}
      </div>
    </form>
    <Dialog.Footer>
      <Button type="button" variant="outline" onclick={closeDialog} disabled={submitting}>
        Annulla
      </Button>
      <Button
        type="submit"
        form="create-trainer-form"
        variant="default"
        disabled={submitting}
      >
        {submitting ? "Creazione…" : "Crea"}
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
