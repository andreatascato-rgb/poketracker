import { writable } from "svelte/store";
import { toast } from "$lib/components/ui/sonner";
import { reportSystemError } from "$lib/stores/error-archive";
import * as profileService from "$lib/services/profile";
import type { Profile } from "$lib/types/api";

export type { Profile };

export const profiles = writable<Profile[]>([]);
export const activeProfile = writable<Profile | null>(null);

/** true dopo il primo loadProfiles() completato (successo o errore). Usato per evitare di mostrare "Nessun profilo" prima del caricamento. */
export const profilesLoaded = writable(false);

export async function loadProfiles() {
  try {
    const [list, active] = await Promise.all([
      profileService.getProfiles(),
      profileService.getActiveProfile(),
    ]);
    profiles.set(list);
    activeProfile.set(active ?? null);
  } catch (err) {
    console.error("Profile load failed:", err);
    reportSystemError({
      type: "Caricamento profili fallito",
      detail: err instanceof Error ? err.message : String(err),
    });
    profiles.set([]);
    activeProfile.set(null);
  } finally {
    profilesLoaded.set(true);
  }
}

/** Imposta il profilo attivo; aggiorna lo store dopo il successo del comando. */
export async function setActiveProfile(id: string): Promise<void> {
  try {
    await profileService.setActiveProfile(id);
    await loadProfiles();
    toast.success("Profilo attivo aggiornato.");
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    toast.error(msg);
    reportSystemError({
      type: "Impostazione profilo attivo fallita",
      detail: msg,
    });
  }
}
