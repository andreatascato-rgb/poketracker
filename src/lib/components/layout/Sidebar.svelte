<script lang="ts">
  import { page } from "$app/stores";
  import { Button } from "$lib/components/ui/button";
  import { User, Pencil, BookOpen, Settings, ChevronLeft, ChevronRight } from "@lucide/svelte";

  const items = [
    { label: "Allenatore", href: "/profilo", icon: User },
    { label: "Editor", href: "/editor", icon: Pencil },
    { label: "Wiki", href: "/wiki", icon: BookOpen },
    { label: "Impostazioni", href: "/impostazioni", icon: Settings },
  ] as const;

  let collapsed = $state(false);

  function isActive(href: string): boolean {
    const path = $page.url.pathname;
    return path === href || (path.startsWith(href + "/") && href !== "/");
  }
</script>

<aside
  class="flex shrink-0 flex-col border-r border-sidebar-border bg-sidebar transition-[width] duration-200 {collapsed ? 'w-12' : 'w-52'}"
  aria-label="Navigazione principale"
>
  <div class="flex h-10 items-center justify-end border-b border-sidebar-border px-2">
    <Button
      type="button"
      variant="ghost"
      size="icon"
      onclick={() => (collapsed = !collapsed)}
      aria-expanded={!collapsed}
      aria-label={collapsed ? "Espandi sidebar" : "Comprimi sidebar"}
    >
      {#if collapsed}
        <ChevronRight class="size-4" />
      {:else}
        <ChevronLeft class="size-4" />
      {/if}
    </Button>
  </div>
  <nav class="flex flex-1 flex-col gap-0.5 p-2">
    {#each items as item}
      {@const Icon = item.icon}
      {@const active = isActive(item.href)}
      <a
        href={item.href}
        class="flex items-center gap-3 rounded-md py-2.5 text-sm font-medium transition-colors {collapsed
          ? 'justify-center px-2'
          : 'px-3'} {active
          ? 'bg-sidebar-accent text-sidebar-accent-foreground'
          : 'text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground'}"
        aria-current={active ? "page" : undefined}
        title={collapsed ? item.label : undefined}
      >
        <Icon class="size-4 shrink-0" />
        {#if !collapsed}
          <span>{item.label}</span>
        {/if}
      </a>
    {/each}
  </nav>
</aside>
