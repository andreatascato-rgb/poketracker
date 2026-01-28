import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";
import { toast } from "$lib/components/ui/sonner";

export interface Profile {
  id: string;
  name: string;
  /** Data primo caricamento (ISO 8601). */
  created_at: string;
  /** Data ultimo aggiornamento (ISO 8601). */
  updated_at: string;
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
  try {
    await invoke("set_active_profile", { id });
    await loadProfiles();
    toast.success("Profilo attivo aggiornato.");
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    toast.error(msg);
    console.error("set_active_profile failed:", err);
  }
}
