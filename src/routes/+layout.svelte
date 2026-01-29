<script lang="ts">
  import { onMount } from "svelte";
  import "../app.css";
  import { page } from "$app/stores";
  import { dev } from "$app/environment";
  import { invoke } from "@tauri-apps/api/core";
  import { profiles, profilesLoaded, activeProfile, loadProfiles } from "$lib/stores/profile";
  import { watchedCount, setWatchedCount, canLayoutSetWatchedCount } from "$lib/stores/sync.svelte";
  import OnboardingView from "$lib/components/onboarding/OnboardingView.svelte";
  import ProfileSelector from "$lib/components/profile/ProfileSelector.svelte";
  import { User, Users, Pencil, BookOpen, Settings, ChevronLeft, ChevronRight, CheckCircle, FolderOpen, RefreshCw, LayoutDashboard, AlertCircle, PawPrint, Zap, Leaf, BarChart3, Search } from "@lucide/svelte";
  import { Tooltip, TooltipContent, TooltipTrigger, TooltipProvider } from "$lib/components/ui/tooltip";
  import { Toaster } from "$lib/components/ui/sonner";
  import { Breadcrumb } from "$lib/components/ui/breadcrumb";

  let { children } = $props();

  /** Titolo pagina derivato dal path (per top bar). */
  const ROUTE_TITLES: Record<string, string> = {
    "/profilo": "Allenatore",
    "/profilo/statistiche": "Statistiche",
    "/profilo/pokedex": "Pokedex",
    "/profilo/salvataggi": "Salvataggi",
    "/editor": "Editor",
    "/wiki": "Wiki",
    "/wiki/pokemon": "Wiki · Pokemon",
    "/wiki/mosse": "Wiki · Mosse",
    "/wiki/nature": "Wiki · Nature",
    "/impostazioni": "Impostazioni",
    "/impostazioni/profili": "Impostazioni · Profili",
    "/impostazioni/errori": "Impostazioni · Errori",
    "/impostazioni/backup-dati": "Impostazioni · Backup e dati",
  };

  function pageTitle(path: string): string {
    const exact = ROUTE_TITLES[path];
    if (exact) return exact;
    for (const [route, title] of Object.entries(ROUTE_TITLES)) {
      if (route !== "/" && path.startsWith(route + "/")) return title;
    }
    return "Pagina";
  }

  /** Breadcrumb: items per Top Bar (docs/standards/breadcrumb-standard.md). Primo = nome profilo (mai link, "l'inizio"); ultimo senza href = pagina corrente. */
  type BreadcrumbItem = { label: string; href?: string };
  function getBreadcrumbItems(path: string, profileName: string): BreadcrumbItem[] {
    const root = { label: profileName };
    if (path === "/") return [root];
    if (path === "/profilo") return [root, { label: "Allenatore" }];
    if (path === "/profilo/statistiche") return [root, { label: "Statistiche" }];
    if (path === "/profilo/pokedex") return [root, { label: "Pokedex" }];
    if (path === "/profilo/salvataggi") return [root, { label: "Salvataggi" }];
    if (path === "/editor") return [root, { label: "Editor" }];
    if (path === "/wiki") return [root, { label: "Wiki" }];
    if (path === "/wiki/pokemon") return [root, { label: "Wiki", href: "/wiki" }, { label: "Pokemon" }];
    if (path === "/wiki/mosse") return [root, { label: "Wiki", href: "/wiki" }, { label: "Mosse" }];
    if (path === "/wiki/nature") return [root, { label: "Wiki", href: "/wiki" }, { label: "Nature" }];
    if (path === "/impostazioni") return [root, { label: "Impostazioni" }];
    if (path === "/impostazioni/profili") return [root, { label: "Impostazioni", href: "/impostazioni" }, { label: "Profili" }];
    if (path === "/impostazioni/errori") return [root, { label: "Impostazioni", href: "/impostazioni" }, { label: "Errori" }];
    if (path === "/impostazioni/backup-dati") return [root, { label: "Impostazioni", href: "/impostazioni" }, { label: "Backup e dati" }];
    if (path.startsWith("/profilo/")) return [root, { label: pageTitle(path) }];
    if (path.startsWith("/wiki/")) return [root, { label: "Wiki", href: "/wiki" }, { label: pageTitle(path) }];
    if (path.startsWith("/impostazioni/")) return [root, { label: "Impostazioni", href: "/impostazioni" }, { label: pageTitle(path) }];
    return [root, { label: pageTitle(path) }];
  }


  type SidebarChild = { label: string; href: string; icon: typeof Users | typeof LayoutDashboard | typeof BarChart3 | typeof Search | typeof FolderOpen | typeof AlertCircle | typeof PawPrint | typeof Zap | typeof Leaf };
  type SidebarItem =
    | { label: string; href: string; icon: typeof User | typeof Pencil | typeof Search | typeof FolderOpen | typeof BarChart3; children?: undefined }
    | { label: string; href: string; icon: typeof BookOpen; children: SidebarChild[] };
  const sidebarItems: SidebarItem[] = [
    { label: "Allenatore", href: "/profilo", icon: User },
    { label: "Statistiche", href: "/profilo/statistiche", icon: BarChart3 },
    { label: "Pokedex", href: "/profilo/pokedex", icon: Search },
    { label: "Editor", href: "/editor", icon: Pencil },
    { label: "Salvataggi", href: "/profilo/salvataggi", icon: FolderOpen },
    {
      label: "Wiki",
      href: "/wiki",
      icon: BookOpen,
      children: [
        { label: "Pokemon", href: "/wiki/pokemon", icon: PawPrint },
        { label: "Mosse", href: "/wiki/mosse", icon: Zap },
        { label: "Nature", href: "/wiki/nature", icon: Leaf },
      ],
    },
  ];

  let sidebarCollapsed = $state(false);
  /** Deferred so first paint completes before sidebar (sidebar render was blocking mount in Tauri webview) */
  let showSidebar = $state(false);
  /** Una sola sezione espansa alla volta. Href della sezione aperta; "" = utente ha chiuso; null = nessuna sezione contiene la route. Solo il toggle utente cambia questo stato (la navigazione non lo resetta). */
  let expandedSection = $state<string | null>(null);
  const EXPANDED_NONE = "";

  /** "In sezione" = path è il padre o una sottovoce (o sotto-path). Usato per espansione e toggle. */
  function isInSection(path: string, item: SidebarItem): boolean {
    if (!("children" in item) || !item.children || item.href === "/") return false;
    return (
      path === item.href ||
      path.startsWith(item.href + "/") ||
      item.children.some((c) => path === c.href || path.startsWith(c.href + "/"))
    );
  }

  /** Href della sezione che contiene il path (per inizializzazione). */
  function sectionContaining(path: string): string | null {
    const item = sidebarItems.find(
      (it) => "children" in it && it.children && isInSection(path, it)
    );
    return item ? item.href : null;
  }

  function toggleSection(href: string) {
    const path = $page?.url?.pathname ?? "";
    const item = sidebarItems.find((it) => "children" in it && it.href === href);
    const inSection = item ? isInSection(path, item) : false;
    const isCurrentlyExpanded = expandedSection === href || (expandedSection === null && inSection);
    if (isCurrentlyExpanded) {
      expandedSection = EXPANDED_NONE;
    } else {
      expandedSection = href;
    }
  }

  /** Stato espanso solo utente: la navigazione non lo cambia. Inizializzazione una tantum al primo path. */
  let expandedInitialized = $state(false);
  $effect(() => {
    const path = $page?.url?.pathname ?? "";
    if (!expandedInitialized) {
      expandedInitialized = true;
      expandedSection = sectionContaining(path) ?? null;
    }
  });

  /** Voce attiva: per voci senza figli (flat) solo path === href; per sottovoci anche sotto-path. */
  function isSidebarActive(href: string, exactOnly = false): boolean {
    const path = $page?.url?.pathname ?? "";
    if (exactOnly) return path === href;
    return path === href || (path.startsWith(href + "/") && href !== "/");
  }

  $effect(() => {
    const n = $watchedCount;
    // #region agent log
    if (import.meta.env.DEV) fetch('http://127.0.0.1:7246/ingest/e155c680-47df-4a07-9855-14bedbe06598',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'+layout.svelte:$effect',message:'Layout $watchedCount',data:{watchedCount:n},timestamp:Date.now(),sessionId:'debug-session',hypothesisId:'B'})}).catch(()=>{});
    // #endregion
  });

  onMount(() => {
    showSidebar = true;
    loadProfiles();
    invoke<string[]>("get_sav_watched_paths")
      .then((paths) => {
        const can = canLayoutSetWatchedCount();
        // #region agent log
        if (import.meta.env.DEV) fetch('http://127.0.0.1:7246/ingest/e155c680-47df-4a07-9855-14bedbe06598',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'+layout.svelte:onMount',message:'Layout onMount get_sav_watched_paths',data:{pathsLength:paths?.length??0,canLayoutSet:can,willOverwrite:can},timestamp:Date.now(),sessionId:'debug-session',hypothesisId:'C'})}).catch(()=>{});
        // #endregion
        if (can) setWatchedCount(paths?.length ?? 0);
      })
      .catch(() => {
        if (canLayoutSetWatchedCount()) setWatchedCount(0);
      });
  });
</script>

<svelte:head>
  <title>PokeTracker</title>
</svelte:head>

{#if $profilesLoaded && $profiles.length === 0}
  <OnboardingView />
{:else}
<!-- Shell layout: TopBar + Sidebar + Main. Stile Poketrack (font/token); sidebar 220px compatta. -->
<div class="poketrack-layout app">
  <TooltipProvider delayDuration={300}>
  <header class="top-bar" aria-label="Intestazione">
    <div class="top-bar-left" aria-label="Contesto">
      <Breadcrumb items={getBreadcrumbItems($page?.url?.pathname ?? "/", $activeProfile?.name ?? "—")} class="flex-1 min-w-0" />
    </div>
    <div class="top-bar-right" role="toolbar" aria-label="Azioni barra superiore">
      <Tooltip>
        <TooltipTrigger>
          <button
            type="button"
            class="top-bar-btn"
            aria-label={$watchedCount > 0 ? ($watchedCount === 1 ? "1 watcher attivo" : `${$watchedCount} watcher attivi`) : "Nessun watcher attivo"}
          >
            <CheckCircle class="top-bar-btn-icon {$watchedCount > 0 ? 'status-active' : ''}" aria-hidden="true" />
          </button>
        </TooltipTrigger>
        <TooltipContent side="bottom" sideOffset={6}>
          {$watchedCount > 0
            ? ($watchedCount === 1 ? "1 watcher attivo" : `${$watchedCount} watcher attivi`)
            : "Nessun watcher attivo"}
        </TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger>
          <a
            href="/impostazioni"
            class="top-bar-btn"
            aria-label="Impostazioni"
          >
            <Settings class="top-bar-btn-icon" aria-hidden="true" />
          </a>
        </TooltipTrigger>
        <TooltipContent side="bottom" sideOffset={6}>Impostazioni</TooltipContent>
      </Tooltip>
      <ProfileSelector />
      {#if dev}
        <Tooltip>
          <TooltipTrigger>
            <button
              type="button"
              class="top-bar-btn"
              aria-label="Ricarica interfaccia"
              onclick={() => window.location.reload()}
            >
              <RefreshCw class="top-bar-btn-icon" aria-hidden="true" />
            </button>
          </TooltipTrigger>
          <TooltipContent side="bottom" sideOffset={6}>Ricarica interfaccia (dev) – vedi modifiche senza riavviare tauri dev</TooltipContent>
        </Tooltip>
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
          {#if sidebarCollapsed}
            <Tooltip>
              <TooltipTrigger>
                <button
                  type="button"
                  class="sidebar-toggle"
                  aria-label="Espandi sidebar"
                  aria-expanded={false}
                  onclick={() => (sidebarCollapsed = !sidebarCollapsed)}
                >
                  <span class="sidebar-chevron rotated" aria-hidden="true">
                    <ChevronLeft class="sidebar-icon-svg" />
                  </span>
                </button>
              </TooltipTrigger>
              <TooltipContent side="right" sideOffset={8}>Espandi sidebar</TooltipContent>
            </Tooltip>
          {:else}
            <Tooltip>
              <TooltipTrigger>
<button
                type="button"
                class="sidebar-toggle"
                aria-label="Collassa sidebar"
                aria-expanded={true}
                onclick={() => (sidebarCollapsed = !sidebarCollapsed)}
              >
                  <span class="sidebar-chevron" aria-hidden="true">
                    <ChevronLeft class="sidebar-icon-svg" />
                  </span>
                </button>
              </TooltipTrigger>
              <TooltipContent side="right" sideOffset={8}>Collassa sidebar</TooltipContent>
            </Tooltip>
          {/if}
          <span class="sidebar-trainer-name-wrap">
            <Tooltip>
              <TooltipTrigger>
                <span class="sidebar-trainer-name">
                  {$activeProfile?.name ?? "—"}
                </span>
              </TooltipTrigger>
              <TooltipContent side="right" sideOffset={8}>{$activeProfile?.name ?? "Nessun profilo"}</TooltipContent>
            </Tooltip>
          </span>
        </div>
        <nav class="sidebar-nav" aria-label="Navigazione principale">
          {#each sidebarItems as item}
            {@const Icon = item.icon}
            {@const active = isSidebarActive(item.href, !item.children)}
            {#if item.children}
              {@const inThisSection = isInSection($page?.url?.pathname ?? "", item)}
              {@const isExpanded = expandedSection === item.href || (expandedSection === null && inThisSection)}
              <div class="sidebar-group">
                {#if sidebarCollapsed}
                  <Tooltip>
                    <TooltipTrigger>
                      <button
                        type="button"
                        class="sidebar-item sidebar-parent"
                        aria-expanded={isExpanded}
                        aria-haspopup="true"
                        onclick={() => toggleSection(item.href)}
                      >
                        <span class="sidebar-item-icon" aria-hidden="true">
                          <Icon class="sidebar-icon-svg" />
                        </span>
                      </button>
                    </TooltipTrigger>
                    <TooltipContent side="right" sideOffset={8}>{item.label}</TooltipContent>
                  </Tooltip>
                {:else}
                  <button
                    type="button"
                    class="sidebar-item sidebar-parent"
                    aria-expanded={isExpanded}
                    aria-haspopup="true"
                    onclick={() => toggleSection(item.href)}
                  >
                    <span class="sidebar-item-icon" aria-hidden="true">
                      <Icon class="sidebar-icon-svg" />
                    </span>
                    <span class="sidebar-item-label">{item.label}</span>
                    <span class="sidebar-group-chevron" class:expanded={isExpanded} aria-hidden="true">
                      <ChevronRight class="sidebar-icon-svg" />
                    </span>
                  </button>
                {/if}
                {#if !sidebarCollapsed}
                  <div class="sidebar-children-wrapper" class:expanded={isExpanded} role="group" aria-label="Sottosezioni {item.label}">
                    <div class="sidebar-children">
                      {#each item.children as child}
                        {@const childActive = isSidebarActive(child.href)}
                        {@const ChildIcon = child.icon}
                        <a
                          href={child.href}
                          class="sidebar-item sidebar-subitem"
                          class:active={childActive}
                          aria-current={childActive ? "page" : undefined}
                        >
                          <span class="sidebar-item-icon" aria-hidden="true">
                            <ChildIcon class="sidebar-icon-svg" />
                          </span>
                          <span class="sidebar-item-label">{child.label}</span>
                        </a>
                      {/each}
                    </div>
                  </div>
                {/if}
              </div>
            {:else}
              {#if sidebarCollapsed}
                <Tooltip>
                  <TooltipTrigger>
                    <a
                      href={item.href}
                      class="sidebar-item"
                      class:active
                      aria-current={active ? "page" : undefined}
                    >
                      <span class="sidebar-item-icon" aria-hidden="true">
                        <Icon class="sidebar-icon-svg" />
                      </span>
                    </a>
                  </TooltipTrigger>
                  <TooltipContent side="right" sideOffset={8}>{item.label}</TooltipContent>
                </Tooltip>
              {:else}
<a
                href={item.href}
                class="sidebar-item"
                class:active
                aria-current={active ? "page" : undefined}
              >
                <span class="sidebar-item-icon" aria-hidden="true">
                  <Icon class="sidebar-icon-svg" />
                </span>
                <span class="sidebar-item-label">{item.label}</span>
              </a>
              {/if}
            {/if}
          {/each}
        </nav>
    </aside>
    {/if}
  </div>
  </TooltipProvider>
</div>
{/if}
<Toaster />

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

  /* Top Bar toolbar: gap 2px tra icone (target 32×32 ≥ 24×24 WCAG). */
  .top-bar-right {
    display: flex;
    align-items: center;
    gap: 2px;
    flex-shrink: 0;
  }

  .top-bar-right > * {
    margin: 0;
  }

  /* :global così gli stili si applicano anche al DropdownMenuTrigger (componente figlio) che usa class="top-bar-btn" */
  :global(.top-bar-btn) {
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

  :global(.top-bar-btn:hover) {
    background: var(--hover-bg, #2a2d2e);
  }

  :global(.top-bar-btn:active) {
    background: var(--pressed-bg, #1f2224);
    transition: none;
  }

  :global(.top-bar-btn:focus-visible) {
    outline: 2px solid var(--focus-ring, #007acc);
    outline-offset: 0;
  }

  /* :global: la classe è sull'icona Lucide (child component), lo scope non la raggiunge. Nessun width/height così resta la dimensione default Lucide (24px) come prima. */
  :global(.top-bar-btn-icon) {
    flex-shrink: 0;
    color: inherit;
  }
  :global(.top-bar-btn-icon.status-active) {
    color: var(--icon-success);
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

  .sidebar-trainer-name-wrap {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    display: flex;
    align-items: center;
    min-height: 32px;
  }

  .sidebar-trainer-name {
    display: block;
    font-size: var(--font-size-ui-primary, 14px);
    font-weight: var(--font-weight-normal, 400);
    line-height: 1.25;
    color: var(--text-secondary, #858585);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sidebar.sidebar-collapsed .sidebar-trainer-name-wrap {
    position: absolute;
    width: 0;
    overflow: hidden;
    opacity: 0;
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

  .sidebar-toggle:active {
    background: var(--pressed-bg, #1f2224);
    transition: none;
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

  .sidebar-group-chevron {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: transform var(--transition-default, 200ms ease-out);
  }

  .sidebar-group-chevron.expanded {
    transform: rotate(90deg);
  }

  .sidebar-group-chevron :global(svg) {
    width: 16px;
    height: 16px;
    color: var(--text-secondary, #858585);
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

  .sidebar-group {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .sidebar-parent {
    width: 100%;
    text-align: left;
    border: none;
    background: transparent;
    cursor: pointer;
    font: inherit;
    color: inherit;
  }

  .sidebar-parent:hover {
    background: var(--hover-bg, #2a2d2e);
  }

  .sidebar-parent:active {
    background: var(--pressed-bg, #1f2224);
    transition: none;
  }

  .sidebar-parent:focus-visible {
    outline: 2px solid var(--focus-ring, #007acc);
    outline-offset: -2px;
  }

  .sidebar-parent .sidebar-item-label {
    flex: 1;
    min-width: 0;
  }

  .sidebar-children-wrapper {
    display: grid;
    grid-template-rows: 0fr;
    transition: grid-template-rows var(--transition-default, 200ms ease-out);
  }

  .sidebar-children-wrapper.expanded {
    grid-template-rows: 1fr;
  }

  .sidebar-children-wrapper > .sidebar-children {
    min-height: 0;
    overflow: hidden;
  }

  .sidebar-children {
    margin-inline-start: 28px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    border-inline-start: 2px solid var(--border-primary, #3e3e42);
    padding-inline-start: 10px;
  }

  .sidebar-subitem {
    padding-inline-start: 0;
  }

  .sidebar-subitem .sidebar-item-icon {
    flex-shrink: 0;
  }

  .sidebar-subitem .sidebar-item-label {
    flex: 1;
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

  /* Hover: superficie “raised” riconoscibile (best practice 2025–2026: feedback chiaro ma non invasivo) */
  .sidebar-item:hover {
    background: var(--bg-tertiary, #2d2d30);
    color: var(--text-primary, #cccccc);
  }

  .sidebar-item:active {
    background: var(--pressed-bg, #1f2224);
    transition: none;
  }

  /* Active (voce corrente): barra a sinistra + sfondo lieve, mai solo colore (W3C), non distraente (UX Movement) */
  .sidebar-item.active {
    background: var(--hover-bg, #2a2d2e);
    color: var(--text-primary, #cccccc);
  }

  .sidebar-item.active::before {
    content: "";
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background: var(--focus-ring, #007acc);
    border-radius: 0 2px 2px 0;
    pointer-events: none;
  }

  .sidebar-item.active .sidebar-item-icon :global(svg) {
    color: inherit;
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

  @media (prefers-reduced-motion: reduce) {
    .sidebar {
      transition: none;
    }
    .sidebar-chevron {
      transition: none;
    }
    .sidebar-group-chevron {
      transition: none;
    }
    .sidebar-children-wrapper {
      transition: none;
    }
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
