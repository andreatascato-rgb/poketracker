/**
 * Servizio Pokedex: wrapper tipati per command Tauri (get_pokedex_state, sync, watched paths).
 */
import { invoke } from "$lib/services/tauri";
import type { PokedexStateEntry } from "$lib/types/api";

export async function getPokedexState(profileId: string): Promise<PokedexStateEntry[]> {
  return invoke<PokedexStateEntry[]>("get_pokedex_state", { profileId });
}

export async function syncPokedexFromWatchedSavsNow(): Promise<void> {
  return invoke("sync_pokedex_from_watched_savs_now");
}

export async function getSavWatchedPaths(): Promise<string[]> {
  return invoke<string[]>("get_sav_watched_paths");
}
