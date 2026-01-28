/**
 * Stato globale per icona sync TopBar e spinner inline in tabella salvataggi.
 *
 * watchedCount: writable store così il layout si sottoscrive con $watchedCount
 * e si ri-renderizza quando Salvataggi aggiorna (cross-module reactivity).
 * Layout fa fetch iniziale; non sovrascrive se Salvataggi ha già scritto (canLayoutSetWatchedCount).
 */

import { writable } from "svelte/store";

/** Numero di watcher attivi. Quando > 0, icona TopBar verde. Sottoscrizione in layout: $watchedCount. */
export const watchedCount = writable(0);

/** True dopo che loadSaves (Salvataggi) ha aggiornato watchedCount. Layout non sovrascrive dopo. */
let watchedCountFromSalvataggi = false;

/** Set dei path attualmente in sync (per spinner inline in tabella salvataggi). */
let syncingPaths = $state<Set<string>>(new Set());

/** Getter per uso reattivo in componenti (Salvataggi usa $derived(getSyncingPaths())). */
export function getSyncingPaths() {
  return syncingPaths;
}

/** Aggiunge path al set sync. */
export function addSyncing(path: string) {
  syncingPaths = new Set(syncingPaths).add(path);
}

/** Rimuove path dal set sync. */
export function removeSyncing(path: string) {
  const next = new Set(syncingPaths);
  next.delete(path);
  syncingPaths = next;
}

/** Setta numero watcher (solo layout, e solo se canLayoutSetWatchedCount()). */
export function setWatchedCount(n: number) {
  // #region agent log
  if (typeof import.meta !== "undefined" && import.meta.env?.DEV) fetch('http://127.0.0.1:7246/ingest/e155c680-47df-4a07-9855-14bedbe06598',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'sync.svelte.ts:setWatchedCount',message:'store write (layout)',data:{n},timestamp:Date.now(),sessionId:'debug-session',hypothesisId:'C'})}).catch(()=>{});
  // #endregion
  watchedCount.set(n);
}

/** Aggiorna watchedCount da Salvataggi (loadSaves). Marca origine così il layout non sovrascrive. */
export function setWatchedCountFromSalvataggi(n: number) {
  watchedCountFromSalvataggi = true;
  // #region agent log
  if (typeof import.meta !== "undefined" && import.meta.env?.DEV) fetch('http://127.0.0.1:7246/ingest/e155c680-47df-4a07-9855-14bedbe06598',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'sync.svelte.ts:setWatchedCountFromSalvataggi',message:'store write (Salvataggi)',data:{n},timestamp:Date.now(),sessionId:'debug-session',hypothesisId:'A'})}).catch(()=>{});
  // #endregion
  watchedCount.set(n);
}

/** True se possiamo ancora usare il fetch del layout per il conteggio iniziale. */
export function canLayoutSetWatchedCount(): boolean {
  return !watchedCountFromSalvataggi;
}
