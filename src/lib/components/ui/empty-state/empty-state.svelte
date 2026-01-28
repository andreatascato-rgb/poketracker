<script lang="ts">
  import type { Component } from "svelte";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardDescription,
    CardFooter,
  } from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";

  interface Props {
    /** Titolo action-oriented, positivo (es. "Aggiungi la prima cartella" non "Nessun salvataggio"). */
    title: string;
    /** Testo che spiega il prossimo passo; max-w-[66ch] per leggibilità. */
    description: string;
    /** Icona sopra il titolo (Lucide o simile), size-12, muted. */
    icon: Component;
    /** Etichetta del pulsante CTA principale. */
    ctaLabel: string;
    /** Icona opzionale nel pulsante principale (stessa size del Button). */
    ctaIcon?: Component;
    /** Callback al click del CTA principale. */
    onCtaClick: () => void;
    /** Etichetta del secondo CTA; se presente, mostra due pulsanti. */
    secondaryCtaLabel?: string;
    /** Icona opzionale nel secondo pulsante. */
    secondaryCtaIcon?: Component;
    /** Callback al click del secondo CTA. */
    onSecondaryCtaClick?: () => void;
    /** Etichetta regione per accessibilità (default derivato da title). */
    ariaLabel?: string;
  }

  let {
    title,
    description,
    icon: Icon,
    ctaLabel,
    ctaIcon: CtaIcon,
    onCtaClick,
    secondaryCtaLabel,
    secondaryCtaIcon: SecondaryCtaIcon,
    onSecondaryCtaClick,
    ariaLabel = `Stato vuoto: ${title}`,
  }: Props = $props();
</script>

<div
  class="flex flex-col items-center justify-center text-center py-12 px-4 w-full"
  role="region"
  aria-label={ariaLabel}
>
  <Icon
    class="size-12 text-muted-foreground/70 mb-4 shrink-0"
    strokeWidth={1.5}
    aria-hidden="true"
  />
  <Card class="w-full max-w-md border border-[var(--border-primary)]">
    <CardHeader class="flex flex-col items-center gap-2 text-center [.border-b]:pb-6">
      <CardTitle class="text-lg font-semibold">{title}</CardTitle>
      <CardDescription class="max-w-[66ch] leading-relaxed">
        {description}
      </CardDescription>
    </CardHeader>
    <CardFooter class="flex justify-center gap-3 [.border-t]:pt-6">
      <Button
        type="button"
        variant="outline"
        size="default"
        onclick={onCtaClick}
        aria-label={ctaLabel}
      >
        {#if CtaIcon}
          <CtaIcon class="size-4" aria-hidden="true" />
        {/if}
        {ctaLabel}
      </Button>
      {#if secondaryCtaLabel && onSecondaryCtaClick}
        <Button
          type="button"
          variant="outline"
          size="default"
          onclick={onSecondaryCtaClick}
          aria-label={secondaryCtaLabel}
        >
          {#if SecondaryCtaIcon}
            <SecondaryCtaIcon class="size-4" aria-hidden="true" />
          {/if}
          {secondaryCtaLabel}
        </Button>
      {/if}
    </CardFooter>
  </Card>
</div>
