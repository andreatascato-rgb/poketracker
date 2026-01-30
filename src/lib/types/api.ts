/**
 * Tipi condivisi con i command Tauri (Rust). Allineati alle struct serde Serialize/Deserialize.
 * Aggiornare quando si cambiano le signature in Rust (api-contract-standard).
 */

export interface Profile {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
  /** Avatar id (Tauri può restituire camelCase o snake_case). */
  avatar_id?: string | null;
  avatarId?: string | null;
}

export interface PokedexStateEntry {
  species_id: number;
  status: string;
}

export interface SavEntry {
  path: string;
  game: string;
  version: string;
  generation: number;
  updated_at: string;
  /** Data ultima modifica file .sav (mtime), per "ultimo gioco giocato". */
  last_played_at?: string | null;
}

export interface SaveGameVersion {
  game: string;
  version: string;
  generation: number;
  languageIdRaw?: number | null;
}

export interface TrainerData {
  money: number | null;
  playedHours: number | null;
  playedMinutes: number | null;
}
