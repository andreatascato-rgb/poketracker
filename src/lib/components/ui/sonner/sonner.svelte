<script lang="ts">
	import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
	import InfoIcon from "@lucide/svelte/icons/info";
	import Loader2Icon from "@lucide/svelte/icons/loader-2";
	import OctagonXIcon from "@lucide/svelte/icons/octagon-x";
	import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";

	import { Toaster as Sonner, type ToasterProps as SonnerProps } from "svelte-sonner";

	let {
		position = "bottom-right",
		duration = 4000,
		theme = "dark",
		toastOptions = {},
		...restProps
	}: SonnerProps = $props();

	/** Ombra leggera ma visibile per staccare il toast dallo sfondo; moderna e raffinata, senza esagerare. */
	const defaultToastOptions = $derived({
		...toastOptions,
		style: [toastOptions?.style, "box-shadow: 0 6px 24px rgba(0,0,0,0.5), 0 2px 8px rgba(0,0,0,0.32);"].filter(Boolean).join(" "),
	});
</script>

<!-- Stile design-system Poketrack; larghezza al contenuto (override 356px di default); posizione bottom-right -->
<Sonner
	{position}
	{duration}
	{theme}
	toastOptions={defaultToastOptions}
	class="toaster group poketrack-toaster"
	style="--width: max-content; min-width: 200px; max-width: min(420px, 90vw); --normal-bg: var(--bg-tertiary, #2d2d30); --normal-text: var(--text-primary, #cccccc); --normal-border: var(--border-primary, #3e3e42); font-family: var(--font-primary, 'Segoe UI', sans-serif); font-size: var(--font-size-body, 13px);"
	{...restProps}
	>{#snippet loadingIcon()}
		<Loader2Icon class="size-4 animate-spin text-[var(--text-secondary)]" aria-hidden="true" />
	{/snippet}
	{#snippet successIcon()}
		<CircleCheckIcon class="size-4 text-[var(--icon-success)]" aria-hidden="true" />
	{/snippet}
	{#snippet errorIcon()}
		<OctagonXIcon class="size-4 text-[var(--icon-destructive)]" aria-hidden="true" />
	{/snippet}
	{#snippet infoIcon()}
		<InfoIcon class="size-4 text-[var(--icon-edit)]" aria-hidden="true" />
	{/snippet}
	{#snippet warningIcon()}
		<TriangleAlertIcon class="size-4 text-amber-400" aria-hidden="true" />
	{/snippet}
</Sonner>
