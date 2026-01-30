/**
 * Avatar personaggi disponibili (cartella static/avatar).
 * Le immagini hanno dimensioni diverse; usiamo object-contain per uniformare.
 */
export const AVATAR_IDS = [
  "alcide",
  "armonio",
  "brendon",
  "lucas",
  "rosso",
] as const;

export type AvatarId = (typeof AVATAR_IDS)[number];

/** Nome visuale per la UI (es. picker) */
export const AVATAR_LABELS: Record<AvatarId, string> = {
  alcide: "Alcide",
  armonio: "Armonio",
  brendon: "Brendon",
  lucas: "Lucas",
  rosso: "Rosso",
};

export function getAvatarSrc(id: AvatarId): string {
  return `/avatar/${id}.png`;
}

export function isAvatarId(value: string | null | undefined): value is AvatarId {
  return value != null && AVATAR_IDS.includes(value as AvatarId);
}

/** Restituisce avatar_id dal profilo (Tauri può usare camelCase o snake_case). */
export function getProfileAvatarId(profile: { avatar_id?: string | null; avatarId?: string | null } | null | undefined): string | null | undefined {
  if (!profile) return undefined;
  return profile.avatarId ?? profile.avatar_id ?? undefined;
}
