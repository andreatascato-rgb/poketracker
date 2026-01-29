/**
 * Servizio profili: wrapper tipati per command Tauri (get_profiles, create_profile, ecc.).
 * I componenti e gli store importano da qui invece di invoke diretto.
 */
import { invoke } from "$lib/services/tauri";
import type { Profile } from "$lib/types/api";

export async function getProfiles(): Promise<Profile[]> {
  return invoke<Profile[]>("get_profiles");
}

export async function getActiveProfile(): Promise<Profile | null> {
  return invoke<Profile | null>("get_active_profile");
}

export async function setActiveProfile(id: string): Promise<void> {
  return invoke("set_active_profile", { id });
}

export async function createProfile(name: string): Promise<Profile> {
  return invoke<Profile>("create_profile", { name });
}

export async function renameProfile(id: string, new_name: string): Promise<void> {
  return invoke("rename_profile", { id, new_name });
}

export async function deleteProfile(id: string): Promise<void> {
  return invoke("delete_profile", { id });
}
