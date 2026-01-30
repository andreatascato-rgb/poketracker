import { writable } from "svelte/store";
import { toast } from "$lib/components/ui/sonner";
import * as errorArchiveService from "$lib/services/error-archive";
import type { ErrorArchiveEntry } from "$lib/services/error-archive";

export type { ErrorArchiveEntry };

const entries = writable<ErrorArchiveEntry[]>([]);

export const errorArchiveEntries = { subscribe: entries.subscribe };

/** Carica le voci da backend (chiamare all'apertura Impostazioni → Errori). Lancia in caso di errore. */
export async function loadErrorArchiveEntries(): Promise<void> {
  const list = await errorArchiveService.getErrorArchiveEntries();
  entries.set(list);
}

/** Rimuove una voce (backend + aggiornamento store). Lancia in caso di errore. */
export async function removeErrorEntry(id: string): Promise<void> {
  await errorArchiveService.removeErrorArchiveEntry(id);
  entries.update((list) => list.filter((e) => e.id !== id));
}

/** Campi per detail strutturato (supporto / assistente AI). Vedi notifications-and-error-archive. */
export interface FormatErrorDetailFields {
  type: string;
  message: string;
  path?: string;
  languageIdRaw?: number;
  [key: string]: string | number | undefined;
}

/** Restituisce un blocco detail formattato per archivio (type, at, message, path, ecc.). */
export function formatErrorDetail(fields: FormatErrorDetailFields): string {
  const at = new Date().toISOString();
  const lines: string[] = [
    `type: ${fields.type}`,
    `at: ${at}`,
    `message: ${fields.message}`,
  ];
  if (fields.path != null) lines.push(`path: ${fields.path}`);
  if (fields.languageIdRaw != null) lines.push(`languageIdRaw: ${fields.languageIdRaw}`);
  for (const [k, v] of Object.entries(fields)) {
    if (k === "type" || k === "at" || k === "message" || k === "path" || k === "languageIdRaw")
      continue;
    if (v !== undefined && v !== "") lines.push(`${k}: ${v}`);
  }
  return lines.join("\n");
}

export interface ReportSystemErrorOptions {
  type: string;
  detail: string;
  toastMessage?: string;
}

/** Segnala un errore di sistema: toast + persistenza in Archivio → Errori. Se la pagina Errori è aperta, lo store si aggiorna subito. */
export function reportSystemError(options: ReportSystemErrorOptions): void {
  const { type, detail, toastMessage } = options;
  toast.error(toastMessage ?? type);
  void errorArchiveService
    .addErrorArchiveEntry({ type, detail })
    .then((entry) => {
      entries.update((list) => [entry, ...list]);
    })
    .catch((e) => console.error("add_error_archive_entry failed:", e));
}
