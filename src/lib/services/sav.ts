/**
 * Servizio salvataggi: wrapper tipati per command Tauri (get_sav_entries, add_sav_entry, detect, ecc.).
 */
import { invoke } from "$lib/services/tauri";
import type { SavEntry, SaveGameVersion, TrainerData } from "$lib/types/api";

export async function getSavEntries(): Promise<SavEntry[]> {
  return invoke<SavEntry[]>("get_sav_entries");
}

export async function addSavEntry(payload: {
  path: string;
  game: string;
  version: string;
  generation?: number;
}): Promise<void> {
  return invoke("add_sav_entry", payload);
}

export async function updateSavEntry(payload: {
  oldPath: string;
  newPath: string;
  game: string;
  version: string;
  generation?: number;
}): Promise<void> {
  return invoke("update_sav_entry", {
    oldPath: payload.oldPath,
    newPath: payload.newPath,
    game: payload.game,
    version: payload.version,
    generation: payload.generation,
  });
}

export async function removeSavEntry(path: string): Promise<void> {
  return invoke("remove_sav_entry", { path });
}

export async function setSavWatched(path: string, watched: boolean): Promise<void> {
  return invoke("set_sav_watched", { path, watched });
}

export async function syncSavNow(path: string): Promise<void> {
  return invoke("sync_sav_now", { path });
}

export async function touchSavEntryUpdatedAt(path: string): Promise<void> {
  return invoke("touch_sav_entry_updated_at", { path });
}

export async function detectSaveGameVersion(path: string): Promise<SaveGameVersion> {
  return invoke<SaveGameVersion>("detect_save_game_version", { path });
}

export async function syncAllSavNow(): Promise<void> {
  return invoke("sync_all_sav_now");
}

export async function getTrainerData(path: string): Promise<TrainerData> {
  return invoke<TrainerData>("get_trainer_data", { path });
}
