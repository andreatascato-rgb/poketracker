/**
 * Servizio Archivio errori: wrapper per command Tauri (get/add/remove).
 * Vedi docs/project/notifications-and-error-archive.md.
 */
import { invoke } from "$lib/services/tauri";

export interface ErrorArchiveEntry {
  id: string;
  at: string;
  type: string;
  detail: string;
}

export async function getErrorArchiveEntries(): Promise<ErrorArchiveEntry[]> {
  return invoke<ErrorArchiveEntry[]>("get_error_archive_entries");
}

export async function addErrorArchiveEntry(payload: {
  type: string;
  detail: string;
}): Promise<ErrorArchiveEntry> {
  return invoke<ErrorArchiveEntry>("add_error_archive_entry", payload);
}

export async function removeErrorArchiveEntry(id: string): Promise<void> {
  return invoke("remove_error_archive_entry", { id });
}
