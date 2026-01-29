<script lang="ts">
  import { page } from "$app/stores";
  import { Users, AlertCircle, FolderOpen } from "@lucide/svelte";

  let { children } = $props();

  const tabs: { label: string; href: string; icon: typeof Users | typeof AlertCircle | typeof FolderOpen }[] = [
    { label: "Profili", href: "/impostazioni/profili", icon: Users },
    { label: "Errori", href: "/impostazioni/errori", icon: AlertCircle },
    { label: "Backup e dati", href: "/impostazioni/backup-dati", icon: FolderOpen },
  ];

  function isActive(href: string): boolean {
    const path = $page?.url?.pathname ?? "";
    return path === href || path.startsWith(href + "/");
  }

  let tabsListRef: HTMLUListElement | null = $state(null);
  let indicatorLeft = $state(0);
  let indicatorWidth = $state(0);

  $effect(() => {
    const path = $page?.url?.pathname ?? "";
    const list = tabsListRef;
    if (!list) return;
    const active = list.querySelector<HTMLAnchorElement>("a.impostazioni-tab.active");
    if (!active) {
      indicatorLeft = 0;
      indicatorWidth = 0;
      return;
    }
    const listRect = list.getBoundingClientRect();
    const activeRect = active.getBoundingClientRect();
    indicatorLeft = activeRect.left - listRect.left;
    indicatorWidth = activeRect.width;
  });
</script>

<div class="impostazioni-layout">
  <nav class="impostazioni-tabs" aria-label="Sottosezioni Impostazioni">
    <div class="impostazioni-tabs-track">
      <ul class="impostazioni-tabs-list" role="tablist" bind:this={tabsListRef}>
        {#each tabs as tab}
          {@const Icon = tab.icon}
          {@const active = isActive(tab.href)}
          <li role="presentation">
            <a
              href={tab.href}
              class="impostazioni-tab"
              class:active
              role="tab"
              aria-selected={active}
              aria-current={active ? "page" : undefined}
            >
              <span class="impostazioni-tab-icon" aria-hidden="true">
                <Icon class="size-full" aria-hidden="true" />
              </span>
              <span>{tab.label}</span>
            </a>
          </li>
        {/each}
      </ul>
      <div
        class="impostazioni-tab-indicator"
        style="left: {indicatorLeft}px; width: {indicatorWidth}px"
        aria-hidden="true"
      ></div>
    </div>
  </nav>
  <div class="impostazioni-content">
    {@render children()}
  </div>
</div>

<style>
  .impostazioni-layout {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg, 16px);
    min-height: 100%;
  }

  .impostazioni-tabs {
    flex-shrink: 0;
    border-bottom: 1px solid var(--border-primary, #3e3e42);
    margin: calc(-1 * var(--spacing-xl, 24px)) calc(-1 * var(--spacing-xl, 24px)) 0;
    padding: 0 var(--spacing-xl, 24px);
  }

  .impostazioni-tabs-track {
    position: relative;
  }

  .impostazioni-tabs-list {
    display: flex;
    gap: 2px;
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .impostazioni-tab {
    display: inline-flex;
    align-items: center;
    gap: var(--spacing-sm, 8px);
    padding: var(--spacing-md, 12px) var(--spacing-lg, 16px);
    margin-bottom: -1px;
    font-size: var(--font-size-ui-primary, 14px);
    font-weight: var(--font-weight-normal, 400);
    color: var(--text-secondary, #858585);
    text-decoration: none;
    border-radius: 4px 4px 0 0;
    /* Solo hover animato; colore/attivo istantanei */
    transition: background-color var(--transition-default, 200ms ease-out);
  }

  .impostazioni-tab:hover {
    color: var(--text-primary, #cccccc);
    background-color: var(--hover-bg, #2a2d2e);
  }

  .impostazioni-tab:active {
    background-color: var(--pressed-bg, #1f2224);
    transition: none;
  }

  .impostazioni-tab.active {
    color: var(--text-primary, #cccccc);
    font-weight: 500;
  }

  .impostazioni-tab:focus-visible {
    outline: 2px solid var(--focus-ring, #007acc);
    outline-offset: -2px;
  }

  .impostazioni-tab-indicator {
    position: absolute;
    bottom: 0;
    left: 0;
    height: 2px;
    background-color: var(--focus-ring, #007acc);
    border-radius: 2px 2px 0 0;
    pointer-events: none;
    transition: left var(--transition-default, 200ms ease-out),
      width var(--transition-default, 200ms ease-out);
  }

  .impostazioni-tab-icon {
    width: var(--icon-size-ui-primary, 18px);
    height: var(--icon-size-ui-primary, 18px);
    flex-shrink: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: inherit;
  }

  .impostazioni-tab-icon :global(svg) {
    width: 100%;
    height: 100%;
  }

  .impostazioni-content {
    flex: 1;
    min-height: 0;
  }

  @media (prefers-reduced-motion: reduce) {
    .impostazioni-tab {
      transition: none;
    }
    .impostazioni-tab-indicator {
      transition: none;
    }
  }
</style>
