<script lang="ts">
  /**
   * StatCard — KPI card stile Poketrack (referenza visiva).
   * Icona in alto, niente bordi colorati, card compatte. NumberFlow per numeri animati.
   */
  import type { Component, Snippet } from "svelte";
  import NumberFlow from "@number-flow/svelte";
  import { Card } from "$lib/components/ui/card";
  import { cn } from "$lib/utils";

  interface Progress {
    current: number;
    total: number;
    percentage: number;
  }

  interface Props {
    label: string;
    value?: string | number;
    suffix?: string;
    /** Icona Lucide (ignorata se è fornito emoji). */
    icon?: Component;
    /** Emoji o carattere visivo al posto dell’icona (es. 💰 🏆). */
    emoji?: string;
    muted?: boolean;
    labelId?: string;
    class?: string;
    progress?: Progress;
    /** Se true, barra e testo progresso (es. "x / total") con toni meno accesi. */
    progressMuted?: boolean;
    /** Se true, il suffix (es. "/493") è reso più spento (colore muted). */
    suffixMuted?: boolean;
    valueContent?: Snippet;
  }

  let {
    label,
    value,
    suffix,
    icon: Icon,
    emoji,
    muted = false,
    labelId,
    class: className = "",
    progress,
    progressMuted = false,
    suffixMuted = false,
    valueContent,
  }: Props = $props();

  const showEmoji = $derived(Boolean(emoji));
  const showIcon = $derived(!showEmoji && Boolean(Icon));

  const useNumberFlow = $derived(typeof value === "number" && !muted && value !== undefined && value !== null);
</script>

<Card
  class={cn(
    "overflow-hidden border border-[var(--border-primary)] py-4 px-4 transition-[box-shadow] duration-200 hover:shadow-sm",
    className
  )}
  role="group"
  aria-labelledby={labelId}
>
  <div class="flex flex-col gap-2">
    <!-- Icona o emoji in alto (come referenza Poketrack) -->
    <div class="flex items-center text-foreground" aria-hidden="true">
      {#if showEmoji}
        <span class="text-2xl leading-none" role="img" aria-hidden="true">{emoji}</span>
      {:else if showIcon && Icon}
        <Icon size={22} strokeWidth={1.5} />
      {/if}
    </div>
    <span
      id={labelId}
      class="text-[10px] font-medium uppercase tracking-[0.2em] text-muted-foreground"
    >
      {label}
    </span>
    <div class="min-w-0">
      {#if valueContent}
        {@render valueContent()}
      {:else if useNumberFlow}
        <div class="flex items-baseline gap-0.5 text-2xl font-bold tracking-tight tabular-nums text-foreground" style="--number-flow-char-height: 0.9em;">
          <NumberFlow
            value={value as number}
            suffix={suffixMuted ? "" : (suffix ?? "")}
            format={{ maximumFractionDigits: 0, minimumFractionDigits: 0 }}
          />
          {#if suffixMuted && suffix}
            <span class="text-2xl font-bold tracking-tight tabular-nums text-muted-foreground/60">{suffix}</span>
          {/if}
        </div>
      {:else}
        <p
          class={cn(
            "text-2xl font-bold tabular-nums tracking-tight",
            muted ? "text-muted-foreground" : "text-foreground"
          )}
        >
          {value ?? "—"}
        </p>
      {/if}
    </div>
    {#if progress}
      <div class="stat-progress" class:stat-progress-muted={progressMuted}>
        <div
          class="progress-bar"
          role="progressbar"
          aria-valuenow={progress.current}
          aria-valuemin={0}
          aria-valuemax={progress.total}
          aria-label="{label}: {progress.current} di {progress.total}"
        >
          <div
            class="progress-fill"
            style="width: {progress.percentage}%;"
          ></div>
        </div>
        <div class="progress-text">
          {progress.current} / {progress.total}
        </div>
      </div>
    {/if}
  </div>
</Card>

<style>
  .stat-progress {
    margin-top: var(--spacing-xs);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }
  .progress-bar {
    height: 6px;
    background: var(--bg-primary);
    border-radius: 3px;
    overflow: hidden;
    position: relative;
  }
  .progress-fill {
    height: 100%;
    border-radius: 3px;
    background: var(--border-primary);
    transition: width var(--transition-default);
    animation: fillProgress 0.5s ease-out;
  }
  @keyframes fillProgress {
    from {
      width: 0;
    }
  }
  .progress-text {
    font-size: 11px;
    color: var(--text-muted-foreground);
  }
  .stat-progress-muted .progress-bar {
    background: var(--bg-primary);
  }
  .stat-progress-muted .progress-fill {
    background: var(--border-primary);
    opacity: 0.6;
  }
  .stat-progress-muted .progress-text {
    color: var(--text-secondary);
    opacity: 0.75;
  }
</style>
