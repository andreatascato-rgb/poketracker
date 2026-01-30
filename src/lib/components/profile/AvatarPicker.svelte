<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Button } from "$lib/components/ui/button";
  import { User } from "@lucide/svelte";
  import {
    AVATAR_IDS,
    AVATAR_LABELS,
    getAvatarSrc,
    isAvatarId,
    type AvatarId,
  } from "$lib/data/avatars";

  interface Props {
    /** Se aperto. */
    open: boolean;
    /** Avatar attualmente selezionato (o null). */
    selectedId: string | null | undefined;
    /** Callback alla selezione: id o null per "nessuno". */
    onSelect: (id: AvatarId | null) => void;
  }

  let { open = $bindable(false), selectedId, onSelect }: Props = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Content
    class="sm:max-w-md"
    aria-labelledby="avatar-picker-title"
    aria-describedby="avatar-picker-desc"
  >
    <Dialog.Header>
      <Dialog.Title id="avatar-picker-title">Scegli avatar</Dialog.Title>
      <Dialog.Description id="avatar-picker-desc">
        Seleziona un personaggio per il tuo profilo allenatore.
      </Dialog.Description>
    </Dialog.Header>

    <div class="grid grid-cols-3 gap-3 py-2">
      <!-- Opzione "Nessuno" -->
      <button
        type="button"
        class="flex flex-col items-center gap-2 rounded-xl border-2 p-3 transition-[border-color,background-color] duration-200 ease-out focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)] focus-visible:ring-offset-2 focus-visible:ring-offset-background {!selectedId || !isAvatarId(selectedId)
          ? 'border-primary bg-[var(--active-bg)]/20'
          : 'border-[var(--border-primary)] hover:border-[var(--border-primary)]/80 hover:bg-[var(--hover-bg)]'}"
        onclick={() => {
          onSelect(null);
          open = false;
        }}
        aria-pressed={!selectedId || !isAvatarId(selectedId)}
      >
        <div
          class="flex size-16 items-center justify-center overflow-hidden rounded-lg border border-[var(--border-primary)]/60 bg-[var(--bg-primary)]"
        >
          <User
            class="text-muted-foreground/50"
            size={32}
            strokeWidth={1.25}
            aria-hidden="true"
          />
        </div>
        <span class="text-xs font-medium text-muted-foreground">Nessuno</span>
      </button>

      <!-- Avatar disponibili (nessun box interno, solo card) -->
      {#each AVATAR_IDS as id}
        <button
          type="button"
          class="flex flex-col items-center gap-2 rounded-xl border-2 p-3 transition-[border-color,background-color] duration-200 ease-out focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)] focus-visible:ring-offset-2 focus-visible:ring-offset-background {selectedId === id
            ? 'border-primary bg-[var(--active-bg)]/20'
            : 'border-[var(--border-primary)] hover:border-[var(--border-primary)]/80 hover:bg-[var(--hover-bg)]'}"
          onclick={() => {
            onSelect(id);
            open = false;
          }}
          aria-pressed={selectedId === id}
          aria-label={AVATAR_LABELS[id]}
        >
          <img
            src={getAvatarSrc(id)}
            alt=""
            class="size-16 object-contain"
          />
          <span class="text-xs font-medium text-muted-foreground">
            {AVATAR_LABELS[id]}
          </span>
        </button>
      {/each}
    </div>

    <Dialog.Footer>
      <Button type="button" variant="outline" onclick={() => (open = false)}>
        Chiudi
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
