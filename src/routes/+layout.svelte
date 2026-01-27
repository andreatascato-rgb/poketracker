<script lang="ts">
  import { onMount } from "svelte";
  import "../app.css";
  import { page } from "$app/stores";
  import { dev } from "$app/environment";
  import { activeProfile, loadProfiles } from "$lib/stores/profile";
  import ProfileSelector from "$lib/components/profile/ProfileSelector.svelte";
  import { Home, User, Pencil, BookOpen, Settings, ChevronLeft, CheckCircle, FolderOpen, RefreshCw } from "@lucide/svelte";

  let { children } = $props();

  /** Titolo pagina derivato dal path (per top bar). */
  const ROUTE_TITLES: Record<string, string> = {
    "/": "Home",
    "/profilo": "Allenatore",
    "/editor": "Editor",
    "/wiki": "Wiki",
    "/impostazioni": "Impostazioni",
  };

  function pageTitle(path: string): string {
    const exact = ROUTE_TITLES[path];
    if (exact) return exact;
    for (const [route, title] of Object.entries(ROUTE_TITLES)) {
      if (route !== "/" && path.startsWith(route + "/")) return title;
    }
    return "Pagina";
  }

  /** Stato sync placeholder: idle | synced | pending. Per ora sempre synced. */
  let syncStatus = $state<"idle" | "synced" | "pending">("synced");

  function openSaveFolder() {
    // Placeholder: comando Tauri "apri cartella salvataggi" quando disponibile
  }

  const sidebarItems = [
    { label: "Home", href: "/", icon: Home },
    { label: "Allenatore", href: "/profilo", icon: User },
    { label: "Editor", href: "/editor", icon: Pencil },
    { label: "Wiki", href: "/wiki", icon: BookOpen },
    { label: "Impostazioni", href: "/impostazioni", icon: Settings },
  ] as const;

  let sidebarCollapsed = $state(false);
  /** Deferred so first paint completes before sidebar (sidebar render was blocking mount in Tauri webview) */
  let showSidebar = $state(false);

  function isSidebarActive(href: string): boolean {
    const path = $page?.url?.pathname ?? "";
    return path === href || (path.startsWith(href + "/") && href !== "/");
  }

  onMount(() => {
    showSidebar = true;
    loadProfiles();
  });
</script>

<svelte:head>
  <title>PokeTracker</title>
</svelte:head>

<!-- Shell layout: TopBar + Sidebar + Main. Stile Poketrack (font/token); sidebar 220px compatta. -->
<div class="poketrack-layout app">
  <header class="top-bar" aria-label="Intestazione">
    <div class="top-bar-left" aria-label="Contesto">
      <span class="top-bar-app-name">PokeTracker</span>
      <span class="top-bar-sep" aria-hidden="true">·</span>
      <span class="top-bar-page-title">{pageTitle($page?.url?.pathname ?? "/")}</span>
    </div>
    <div class="top-bar-right" role="toolbar" aria-label="Azioni barra superiore">
      <button
        type="button"
        class="top-bar-btn"
        title={syncStatus === "synced" ? "Salvataggi sincronizzati" : syncStatus === "pending" ? "Aggiornamento in corso…" : "Stato sincronizzazione"}
        aria-label={syncStatus === "synced" ? "Salvataggi sincronizzati" : syncStatus === "pending" ? "Aggiornamento in corso" : "Stato sincronizzazione"}
      >
        {#if syncStatus === "pending"}
          <RefreshCw class="top-bar-btn-icon spinning" aria-hidden="true" />
        {:else}
          <CheckCircle class="top-bar-btn-icon status-ok" aria-hidden="true" />
        {/if}
      </button>
      <button
        type="button"
        class="top-bar-btn"
        title="Apri cartella salvataggi"
        aria-label="Apri cartella salvataggi"
        onclick={openSaveFolder}
      >
        <FolderOpen class="top-bar-btn-icon" aria-hidden="true" />
      </button>
      <ProfileSelector />
      {#if dev}
        <button
          type="button"
          class="top-bar-btn"
          title="Ricarica interfaccia (dev) – vedi modifiche senza riavviare tauri dev"
          aria-label="Ricarica interfaccia"
          onclick={() => window.location.reload()}
        >
          <RefreshCw class="top-bar-btn-icon" aria-hidden="true" />
        </button>
      {/if}
    </div>
  </header>

  <div class="app-body">
    <!-- Main first in DOM so content mounts before sidebar (sidebar render was blocking mount in Tauri webview) -->
    <main class="main-content" aria-label="Contenuto principale">
      <div class="main-content-inner">
        {@render children()}
      </div>
    </main>
    {#if showSidebar}
    <aside class="sidebar" class:sidebar-collapsed={sidebarCollapsed} aria-label="Sidebar">
      <div class="sidebar-header">
        <button
          type="button"
          class="sidebar-toggle"
          aria-label={sidebarCollapsed ? "Espandi sidebar" : "Collassa sidebar"}
          aria-expanded={!sidebarCollapsed}
          title={sidebarCollapsed ? "Espandi sidebar" : "Collassa sidebar"}
          onclick={() => (sidebarCollapsed = !sidebarCollapsed)}
        >
          <span class="sidebar-chevron" class:rotated={sidebarCollapsed} aria-hidden="true">
            <ChevronLeft class="sidebar-icon-svg" />
          </span>
        </button>
        <span class="sidebar-trainer-name" title={$activeProfile?.name ?? "Nessun profilo"}>
          {$activeProfile?.name ?? "—"}
        </span>
      </div>
      <nav class="sidebar-nav" aria-label="Navigazione principale">
        {#each sidebarItems as item}
          {@const Icon = item.icon}
          {@const active = isSidebarActive(item.href)}
          <a
            href={item.href}
            class="sidebar-item"
            class:active
            aria-current={active ? "page" : undefined}
            title={sidebarCollapsed ? item.label : undefined}
          >
            <span class="sidebar-item-icon" aria-hidden="true">
              <Icon class="sidebar-icon-svg" />
            </span>
            {#if !sidebarCollapsed}
              <span class="sidebar-item-label">{item.label}</span>
            {/if}
          </a>
        {/each}
      </nav>
    </aside>
    {/if}
  </div>
</div>

<style>
  .poketrack-layout.app {
    width: 100%;
    height: 100vh;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background-color: var(--bg-primary, #1e1e1e);
    color: var(--text-primary, #cccccc);
    font-family: var(--font-primary, 'Segoe UI', 'SF Pro Display', -apple-system, BlinkMacSystemFont, sans-serif);
    font-size: var(--font-size-body, 13px);
  }

  .top-bar {
    height: 48px;
    min-height: 48px;
    background: var(--bg-primary, #1e1e1e);
    border-bottom: 1px solid var(--border-primary, #3e3e42);
    padding: 0 var(--spacing-lg, 16px);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--spacing-md, 12px);
    flex-shrink: 0;
    font-size: var(--font-size-ui-primary, 14px);
  }

  .top-bar-left {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm, 8px);
    min-width: 0;
    flex: 1;
  }

  .top-bar-app-name {
    font-weight: var(--font-weight-medium, 500);
    color: var(--text-primary, #cccccc);
    white-space: nowrap;
  }

  .top-bar-sep {
    color: var(--text-secondary, #858585);
    user-select: none;
  }

  .top-bar-page-title {
    color: var(--text-primary, #cccccc);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .top-bar-right {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs, 4px);
    flex-shrink: 0;
  }

  .top-bar-btn {
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
    text-decoration: none;
    transition: background var(--transition-default, 200ms ease-out);
  }

  .top-bar-btn:hover {
    background: var(--hover-bg, #2a2d2e);
  }

  .top-bar-btn:focus-visible {
    outline: 2px solid var(--focus-ring, #007acc);
    outline-offset: 0;
  }

  .top-bar-btn-icon {
    width: var(--icon-size-ui-primary, 18px);
    height: var(--icon-size-ui-primary, 18px);
    flex-shrink: 0;
    color: inherit;
  }

  .top-bar-btn-icon.status-ok {
    color: var(--text-secondary, #858585);
  }

  .top-bar-btn-icon.spinning {
    animation: top-bar-spin 1s linear infinite;
  }

  @keyframes top-bar-spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .app-body {
    flex: 1;
    display: flex;
    overflow: hidden;
    min-height: 0;
    background: var(--bg-primary, #1e1e1e);
  }

  .sidebar {
    order: -1;
    width: 220px;
    min-width: 220px;
    background: var(--bg-secondary, #252526);
    border-right: 1px solid var(--border-primary, #3e3e42);
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    min-height: 100%;
    font-family: var(--font-primary, 'Segoe UI', 'SF Pro Display', -apple-system, BlinkMacSystemFont, sans-serif);
    transition: width var(--transition-default, 200ms ease-out), min-width var(--transition-default, 200ms ease-out);
  }

  .sidebar.sidebar-collapsed {
    width: 48px;
    min-width: 48px;
  }

  .sidebar-header {
    padding: var(--spacing-sm, 8px);
    border-bottom: 1px solid var(--border-primary, #3e3e42);
    display: flex;
    align-items: center;
    gap: var(--spacing-sm, 8px);
    flex-shrink: 0;
    min-height: 40px;
  }

  .sidebar-trainer-name {
    flex: 1;
    min-width: 0;
    font-size: var(--font-size-ui-primary, 14px);
    font-weight: var(--font-weight-normal, 400);
    color: var(--text-secondary, #858585);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sidebar.sidebar-collapsed .sidebar-trainer-name {
    position: absolute;
    width: 0;
    overflow: hidden;
    opacity: 0;
  }

  .sidebar-toggle {
    width: 32px;
    height: 32px;
    padding: 0;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-primary, #cccccc);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background var(--transition-default, 200ms ease-out);
  }

  .sidebar-toggle:hover {
    background: var(--hover-bg, #2a2d2e);
  }

  .sidebar-toggle:focus-visible {
    outline: 2px solid var(--focus-ring, #007acc);
    outline-offset: 0;
  }

  .sidebar-chevron {
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform var(--transition-default, 200ms ease-out);
  }

  .sidebar-chevron.rotated {
    transform: rotate(180deg);
  }

  /* Icone voci: token --icon-size-ui-primary (stesso di toolbar/Default); chevron toggle resta 16px */
  .sidebar-item-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: var(--icon-size-ui-primary, 18px);
    height: var(--icon-size-ui-primary, 18px);
    flex-shrink: 0;
  }

  .sidebar-item-icon :global(svg) {
    width: var(--icon-size-ui-primary, 18px);
    height: var(--icon-size-ui-primary, 18px);
    color: inherit;
    transition: color var(--transition-default, 200ms ease-out);
  }

  .sidebar-chevron :global(svg) {
    width: 16px;
    height: 16px;
    color: inherit;
    transition: color var(--transition-default, 200ms ease-out);
  }

  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: var(--spacing-sm, 8px);
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    overflow-x: hidden;
  }

  .sidebar-item {
    display: flex;
    align-items: center;
    gap: 10px;
    min-height: var(--nav-item-height, 34px);
    padding: 8px var(--spacing-md, 12px);
    border-radius: 4px;
    position: relative;
    font-size: var(--font-size-ui-primary, 14px);
    font-weight: var(--font-weight-normal, 400);
    font-family: var(--font-primary, 'Segoe UI', 'SF Pro Display', -apple-system, sans-serif);
    color: var(--text-primary, #cccccc);
    text-decoration: none;
    transition: background-color var(--transition-default, 200ms ease-out),
      color var(--transition-default, 200ms ease-out);
  }

  .sidebar.sidebar-collapsed .sidebar-item {
    justify-content: center;
    padding: 8px;
  }

  .sidebar-item:hover {
    background: var(--hover-bg, #2a2d2e);
    color: var(--text-primary, #cccccc);
  }

  .sidebar-item.active {
    background: var(--active-bg, #094771);
    color: #fff;
  }

  .sidebar-item.active .sidebar-item-icon :global(svg) {
    color: #fff;
  }

  .sidebar-item:focus-visible {
    outline: 2px solid var(--focus-ring, #007acc);
    outline-offset: -2px;
  }

  .sidebar-item-label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sidebar.sidebar-collapsed .sidebar-item-label {
    position: absolute;
    width: 0;
    overflow: hidden;
    opacity: 0;
  }

  .main-content {
    order: 0;
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    background: var(--bg-primary, #1e1e1e);
    min-height: 0;
    min-width: 0;
  }

  /** Padding area contenuto: token design system (12–16px voci menu; 24px per pagina). Vedi design-system-standard, ContentArea (p-4 md:p-6). */
  .main-content-inner {
    padding: var(--spacing-xl, 24px);
    min-height: 100%;
    box-sizing: border-box;
  }
</style>
