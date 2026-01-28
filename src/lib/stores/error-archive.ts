import { writable } from "svelte/store";
import { toast } from "$lib/components/ui/sonner";

/** Voce nell'archivio errori: log completo per supporto / assistente AI. */
export interface ErrorArchiveEntry {
  id: string;
  /** Data/ora ISO 8601. */
  at: string;
  /** Tipo breve (es. "Versione non determinata", "Sidecar timeout"). */
  type: string;
  /** Dettaglio completo copiabile (messaggio, contesto, path, stack, ecc.). */
  detail: string;
}

/** Archivio in memoria; in futuro si può persistere su file/DB. */
const entries = writable<ErrorArchiveEntry[]>([]);

export const errorArchiveEntries = { subscribe: entries.subscribe };

/** Aggiunge una voce all'archivio. id deve essere unico (es. crypto.randomUUID()). */
export function addErrorEntry(entry: Omit<ErrorArchiveEntry, "at"> & { at?: string }): void {
  entries.update((list) => [
    {
      ...entry,
      at: entry.at ?? new Date().toISOString(),
    },
    ...list,
  ]);
}

/** Rimuove una voce per id. */
export function removeErrorEntry(id: string): void {
  entries.update((list) => list.filter((e) => e.id !== id));
}

/** Parametri per segnalare un errore di sistema (toast + archivio). Vedi docs/project/notifications-and-error-archive.md (Standard operativo). */
export interface ReportSystemErrorOptions {
  /** Etichetta breve user-facing (es. "Versione non determinata", "Sidecar timeout"). */
  type: string;
  /** Log completo copiabile per supporto/assistente AI. */
  detail: string;
  /** Messaggio toast; se assente si usa type. */
  toastMessage?: string;
}

/** Segnala un errore di sistema: toast + voce in Archivio → Errori. Usare per tutti i casi reali (invoke fallito, sidecar timeout, ecc.). */
export function reportSystemError(options: ReportSystemErrorOptions): void {
  const { type, detail, toastMessage } = options;
  toast.error(toastMessage ?? type);
  addErrorEntry({
    id: crypto.randomUUID(),
    type,
    detail,
  });
}
