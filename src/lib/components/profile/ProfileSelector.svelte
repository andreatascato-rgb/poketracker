<script lang="ts">
  import { activeProfile, loadProfiles, profiles, setActiveProfile } from "$lib/stores/profile";
  import { onMount } from "svelte";
  import { User } from "@lucide/svelte";

  let open = $state(false);
  let triggerRef = $state<HTMLDivElement | null>(null);

  onMount(() => {
    loadProfiles();
  });

  function toggle() {
    open = !open;
  }

  function close() {
    open = false;
  }

  async function selectProfile(id: string) {
    await setActiveProfile(id);
    close();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape" && open) close();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="profile-selector-wrap" bind:this={triggerRef}>
  <button
    type="button"
    class="profile-selector-trigger"
    aria-haspopup="listbox"
    aria-expanded={open}
    aria-controls="profile-listbox"
    id="profile-trigger"
    title="Seleziona profilo"
    aria-label="Seleziona profilo"
    onclick={toggle}
  >
    <User class="profile-selector-trigger-icon" aria-hidden="true" />
  </button>

  {#if open}
    <!-- Backdrop: click outside chiude -->
    <div class="profile-selector-backdrop" role="presentation" onclick={close} />
    <div
      id="profile-listbox"
      class="profile-selector-dropdown"
      role="listbox"
      aria-label="Seleziona profilo"
      aria-activedescendant={$activeProfile?.id ?? undefined}
    >
      {#each $profiles as profile (profile.id)}
        <button
          type="button"
          class="profile-selector-option"
          role="option"
          aria-selected={$activeProfile?.id === profile.id}
          onclick={() => selectProfile(profile.id)}
        >
          {profile.name}
        </button>
      {/each}
      {#if $profiles.length === 0}
        <span class="profile-selector-empty">Nessun profilo</span>
      {/if}
    </div>
  {/if}
</div>

<style>
  .profile-selector-wrap {
    position: relative;
    display: inline-flex;
  }

  /* Uguale a .top-bar-btn: solo icona 32×32, stesso hover/focus */
  .profile-selector-trigger {
    width: 32px;
    height: 32px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-primary, #cccccc);
    cursor: pointer;
    transition: background var(--transition-default, 200ms ease-out);
    -webkit-tap-highlight-color: transparent;
  }

  .profile-selector-trigger:hover {
    background: var(--hover-bg, #2a2d2e);
  }

  .profile-selector-trigger:focus-visible {
    outline: 2px solid var(--focus-ring, #007acc);
    outline-offset: 0;
  }

  .profile-selector-trigger-icon :global(svg) {
    width: var(--icon-size-ui-primary, 18px);
    height: var(--icon-size-ui-primary, 18px);
    flex-shrink: 0;
    color: inherit;
  }

  .profile-selector-backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
    background: transparent;
    cursor: default;
  }

  .profile-selector-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    min-width: 160px;
    padding: var(--spacing-xs, 4px);
    background: var(--bg-secondary, #252526);
    border: 1px solid var(--border-primary, #3e3e42);
    border-radius: 4px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    z-index: 50;
    font-size: var(--font-size-ui-primary, 14px);
    font-family: var(--font-primary, "Segoe UI", sans-serif);
    color: var(--text-primary, #cccccc);
  }

  .profile-selector-option {
    display: block;
    width: 100%;
    padding: 6px var(--spacing-md, 12px);
    border: none;
    border-radius: 4px;
    background: transparent;
    color: inherit;
    font-size: inherit;
    font-family: inherit;
    font-weight: var(--font-weight-normal, 400);
    text-align: left;
    cursor: pointer;
    transition: background-color var(--transition-default, 200ms ease-out);
  }

  .profile-selector-option:hover {
    background: var(--hover-bg, #2a2d2e);
  }

  .profile-selector-option[aria-selected="true"] {
    background: var(--active-bg, #094771);
    color: #fff;
  }

  .profile-selector-option:focus-visible {
    outline: 2px solid var(--focus-ring, #007acc);
    outline-offset: -2px;
  }

  .profile-selector-empty {
    display: block;
    padding: 8px var(--spacing-md, 12px);
    color: var(--text-secondary, #858585);
    font-size: inherit;
  }
</style>
