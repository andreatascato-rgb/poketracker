<script lang="ts">
  import { cn } from "$lib/utils.js";
  import { ChevronRight } from "@lucide/svelte";

  interface BreadcrumbItem {
    label: string;
    href?: string;
  }

  interface Props {
    items: BreadcrumbItem[];
    class?: string;
    /** aria-label per il nav (default: "Breadcrumb") */
    ariaLabel?: string;
  }

  let { items, class: className, ariaLabel = "Breadcrumb" }: Props = $props();
</script>

<nav aria-label={ariaLabel} class={cn("flex items-center min-w-0", className)}>
  <ol class="flex flex-wrap items-center gap-0 text-[var(--font-size-body)] font-[var(--font-primary)] list-none p-0 m-0">
    {#each items as item, i}
      {@const isCurrent = i === items.length - 1}
      {#if i > 0}
        <li class="flex items-center shrink-0 px-1" aria-hidden="true">
          <ChevronRight class="size-3.5 text-[var(--text-secondary)]" />
        </li>
      {/if}
      <li class="flex items-center shrink-0">
        {#if item.href != null}
          <a
            href={item.href}
            class="inline-flex items-center min-h-11 min-w-11 py-2 px-3 -my-2 -mx-3 rounded text-[var(--text-secondary)] no-underline transition-[background-color,color] duration-200 ease-out hover:bg-[var(--hover-bg)] hover:text-[var(--text-primary)] active:bg-[var(--pressed-bg)] active:transition-none focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-[var(--focus-ring)]"
          >
            {item.label}
          </a>
        {:else}
          <span
            aria-current={isCurrent ? "page" : undefined}
            class="truncate max-w-[200px] sm:max-w-none {isCurrent
              ? 'text-[var(--text-primary)] font-medium'
              : 'text-[var(--text-secondary)]'}"
          >
            {item.label}
          </span>
        {/if}
      </li>
    {/each}
  </ol>
</nav>
