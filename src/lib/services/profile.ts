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

/** Aggiorna solo l'avatar del profilo. Restituisce il profilo aggiornato. */
export async function updateProfileAvatar(
  id: string,
  avatarId: string | null
): Promise<Profile> {
  const payload = { id, avatarId: avatarId ?? null };
  // #region agent log
  fetch('http://127.0.0.1:7246/ingest/e155c680-47df-4a07-9855-14bedbe06598',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({location:'profile.ts:updateProfileAvatar',message:'before invoke',data:{payload},timestamp:Date.now(),sessionId:'debug-session',hypothesisId:'H2',runId:'post-fix'})}).catch(()=>{});
  // #endregion
  return invoke<Profile>("update_profile_avatar", payload);
}
