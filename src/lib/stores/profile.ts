import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

export interface Profile {
  id: string;
  name: string;
}

export const profiles = writable<Profile[]>([]);
export const activeProfile = writable<Profile | null>(null);

/** true dopo il primo loadProfiles() completato (successo o errore). Usato per evitare di mostrare "Nessun profilo" prima del caricamento. */
export const profilesLoaded = writable(false);

export async function loadProfiles() {
  try {
    const [list, active] = await Promise.all([
      invoke<Profile[]>("get_profiles"),
      invoke<Profile | null>("get_active_profile"),
    ]);
    profiles.set(list);
    activeProfile.set(active ?? null);
  } catch (err) {
    console.error("Profile load failed:", err);
    profiles.set([]);
    activeProfile.set(null);
  } finally {
    profilesLoaded.set(true);
  }
}

/** Imposta il profilo attivo; aggiorna lo store dopo il successo del comando. */
export async function setActiveProfile(id: string): Promise<void> {
  await invoke("set_active_profile", { id });
  await loadProfiles();
}
