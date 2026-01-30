<script lang="ts">
  /**
   * Statistiche Pokedex: visti e catturati.
   * Design: raffinato, minimalista, tipografia in primo piano (IBM "less is more",
   * Linear/Stripe refined understated, Tailwind UI "Simple on dark").
   * Nessuna barra dominante; indicatore di progresso ultra-sottile in fondo.
   * Numeri animati con NumberFlow al cambio filtri (rispetta prefers-reduced-motion).
   */
  import NumberFlow from "@number-flow/svelte";

  interface Props {
    caught: number;
    seen: number;
    total: number;
    /** Classi aggiuntive sul contenitore (es. h-full per allineare altezza con card adiacenti). */
    class?: string;
  }

  let { caught, seen, total, class: className = "" }: Props = $props();

  const caughtPct = $derived(
    total > 0 ? Math.min(100, (caught / total) * 100) : 0
  );
  const seenPct = $derived(
    total > 0 ? Math.min(100, (seen / total) * 100) : 0
  );
  const seenOnlyPct = $derived(seenPct - caughtPct);
  const caughtPctInt = $derived(total > 0 ? Math.round(caughtPct) : 0);
  const seenPctInt = $derived(total > 0 ? Math.round(seenPct) : 0);

  const numberFormat = { maximumFractionDigits: 0, minimumFractionDigits: 0 };
</script>

<figure
  class="rounded-xl border border-[var(--border-primary)] bg-[var(--bg-tertiary)] px-6 py-5 {className}"
  role="group"
  aria-label="Statistiche Pokedex: {caught} catturati, {seen} visti su {total}"
>
  <div class="flex items-end justify-between gap-8">
    <!-- Catturati -->
    <div class="min-w-0 flex-1">
      <p class="text-[10px] font-medium uppercase tracking-[0.2em] text-muted-foreground">
        Catturati
      </p>
      <p class="mt-1 flex items-baseline gap-2" aria-hidden="true">
        <span
          class="flex items-baseline gap-0.5 text-4xl font-medium tabular-nums tracking-tight text-foreground"
          style="--number-flow-char-height: 1em;"
        >
          <NumberFlow value={caught} format={numberFormat} />
        </span>
        <span
          class="flex items-baseline gap-0.5 text-sm tabular-nums text-muted-foreground"
          style="--number-flow-char-height: 0.875em;"
        >
          <NumberFlow value={caughtPctInt} format={numberFormat} />%
        </span>
      </p>
    </div>

    <!-- Divisore verticale (Tailwind UI "shared borders") -->
    <div
      class="h-11 w-px shrink-0 bg-[var(--border-primary)]"
      aria-hidden="true"
    ></div>

    <!-- Visti -->
    <div class="min-w-0 flex-1">
      <p class="text-[10px] font-medium uppercase tracking-[0.2em] text-muted-foreground">
        Visti
      </p>
      <p class="mt-1 flex items-baseline gap-2" aria-hidden="true">
        <span
          class="flex items-baseline gap-0.5 text-4xl font-medium tabular-nums tracking-tight text-foreground"
          style="--number-flow-char-height: 1em;"
        >
          <NumberFlow value={seen} format={numberFormat} />
        </span>
        <span
          class="flex items-baseline gap-0.5 text-sm tabular-nums text-muted-foreground"
          style="--number-flow-char-height: 0.875em;"
        >
          <NumberFlow value={seenPctInt} format={numberFormat} />%
        </span>
      </p>
    </div>

    <!-- Denominatore: un solo riferimento (essential, no clutter) -->
    <p
      class="flex shrink-0 items-baseline justify-end gap-0.5 text-right text-xs tabular-nums text-muted-foreground/80"
      aria-hidden="true"
      style="--number-flow-char-height: 0.75em;"
    >
      di <NumberFlow value={total} format={numberFormat} />
    </p>
  </div>

  <!-- Indicatore di progresso: presenza dati senza dominare; width animata al cambio filtri -->
  <div
    class="mt-3 flex h-2.5 w-full overflow-hidden rounded-full bg-[var(--bg-primary)]"
    role="progressbar"
    aria-valuenow={seen}
    aria-valuemin={0}
    aria-valuemax={total}
    aria-label="Visti {seen} di {total}"
  >
    <div
      class="h-full shrink-0 rounded-l-full bg-[var(--icon-success)]/90 transition-[width] duration-200 ease-out"
      style="width: {caughtPct}%;"
    ></div>
    <div
      class="h-full shrink-0 bg-[var(--focus-ring)]/70 transition-[width] duration-200 ease-out"
      style="width: {seenOnlyPct}%;"
    ></div>
  </div>
</figure>
