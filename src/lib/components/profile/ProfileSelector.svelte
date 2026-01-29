<script lang="ts">
  import { get } from "svelte/store";
  import { activeProfile, loadProfiles, profiles, profilesLoaded, setActiveProfile } from "$lib/stores/profile";
  import { onMount } from "svelte";
  import { User, Check } from "@lucide/svelte";
  import { Tooltip, TooltipContent, TooltipTrigger } from "$lib/components/ui/tooltip";
  import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
  } from "$lib/components/ui/dropdown-menu";

  let loaded = $state(get(profilesLoaded));
  let profilesList = $state(get(profiles));
  let active = $state(get(activeProfile));
  $effect(() => {
    const unsubLoaded = profilesLoaded.subscribe((v) => { loaded = v; });
    const unsubProfiles = profiles.subscribe((v) => { profilesList = v; });
    const unsubActive = activeProfile.subscribe((v) => { active = v; });
    return () => {
      unsubLoaded();
      unsubProfiles();
      unsubActive();
    };
  });

  onMount(() => {
    loadProfiles();
  });

  async function selectProfile(id: string) {
    await setActiveProfile(id);
  }
</script>

<div class="profile-selector-wrap">
  <DropdownMenu>
    <Tooltip>
      <TooltipTrigger>
        <DropdownMenuTrigger
          class="profile-selector-trigger"
          aria-label="Seleziona profilo"
        >
          <User class="profile-selector-trigger-icon" aria-hidden="true" />
        </DropdownMenuTrigger>
      </TooltipTrigger>
      <TooltipContent side="bottom" sideOffset={6}>
        {#if active}
          Profilo attivo: {active.name}
        {:else}
          Seleziona profilo
        {/if}
      </TooltipContent>
    </Tooltip>
    <DropdownMenuContent
      class="profile-dropdown-content"
      side="bottom"
      align="end"
      sideOffset={4}
    >
      {#if !loaded}
        <div class="profile-selector-loading" role="status" aria-live="polite">
          Caricamento…
        </div>
      {:else if profilesList.length === 0}
        <div class="profile-selector-empty">Nessun profilo</div>
      {:else}
        {#each profilesList as profile (profile.id)}
          <DropdownMenuItem
            class="profile-selector-item {active?.id === profile.id ? 'profile-selector-item-selected' : ''}"
            onclick={() => selectProfile(profile.id)}
          >
            {#if active?.id === profile.id}
              <Check class="profile-selector-check" aria-hidden="true" />
            {:else}
              <span class="profile-selector-check-placeholder" aria-hidden="true"></span>
            {/if}
            <span class="profile-selector-item-label">{profile.name}</span>
          </DropdownMenuItem>
        {/each}
      {/if}
    </DropdownMenuContent>
  </DropdownMenu>
</div>

<style>
  .profile-selector-wrap {
    position: relative;
    display: inline-flex;
  }

  /* Allineato a .top-bar-btn: icona 32×32, stesso hover/focus (interaction-states-standard). Transizione solo su hover; :active istantaneo. */
  :global(.profile-selector-trigger) {
    width: 32px;
    height: 32px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-primary, #cccccc);
    cursor: pointer;
    transition: background-color var(--transition-default, 200ms ease-out);
    -webkit-tap-highlight-color: transparent;
  }

  :global(.profile-selector-trigger:hover) {
    background-color: var(--hover-bg, #2a2d2e);
  }

  :global(.profile-selector-trigger:active) {
    background-color: var(--pressed-bg, #1f2224);
    transition: none;
  }

  :global(.profile-selector-trigger:focus-visible) {
    outline: 2px solid var(--focus-ring, #007acc);
    outline-offset: 0;
  }

  :global(.profile-selector-trigger-icon svg) {
    width: var(--icon-size-ui-primary, 18px);
    height: var(--icon-size-ui-primary, 18px);
    flex-shrink: 0;
    color: inherit;
  }

  :global(.profile-dropdown-content) {
    min-width: 160px;
    padding: var(--spacing-xs, 4px);
    background: var(--bg-secondary, #252526) !important;
    border: 1px solid var(--border-primary, #3e3e42) !important;
    border-radius: 4px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    font-size: var(--font-size-ui-primary, 14px);
    font-family: var(--font-primary, "Segoe UI", sans-serif);
    color: var(--text-primary, #cccccc);
  }

  /* Voci dropdown: hover con transizione, :active istantaneo (interaction-states-standard). Colori con background-color per transizione pulita. */
  :global(.profile-selector-item) {
    padding: 8px var(--spacing-md, 12px);
    border-radius: 4px;
    font-weight: var(--font-weight-normal, 400);
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    background-color: transparent;
    color: inherit;
    transition: background-color var(--transition-default, 200ms ease-out);
  }

  :global(.profile-selector-item:hover),
  :global(.profile-selector-item[data-highlighted]) {
    background-color: var(--hover-bg, #2a2d2e) !important;
  }

  :global(.profile-selector-item:active) {
    background-color: var(--pressed-bg, #1f2224) !important;
    transition: none;
  }

  :global(.profile-selector-item-selected),
  :global(.profile-selector-item-selected:hover),
  :global(.profile-selector-item-selected[data-highlighted]) {
    background-color: var(--active-bg, #094771) !important;
    color: #fff;
  }

  /* Selected + active: feedback pressione leggermente più scuro */
  :global(.profile-selector-item-selected:active) {
    background-color: var(--pressed-bg, #1f2224) !important;
    transition: none;
  }

  :global(.profile-selector-check svg) {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
  }

  .profile-selector-check-placeholder {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
    display: inline-block;
  }

  .profile-selector-item-label {
    flex: 1;
    text-align: left;
  }

  .profile-selector-loading,
  .profile-selector-empty {
    padding: 8px var(--spacing-md, 12px);
    color: var(--text-secondary, #858585);
    font-size: var(--font-size-ui-primary, 14px);
  }

  .profile-selector-loading {
    font-style: italic;
  }
</style>
