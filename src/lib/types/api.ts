/**
 * Tipi condivisi con i command Tauri (Rust). Allineati alle struct serde Serialize/Deserialize.
 * Aggiornare quando si cambiano le signature in Rust (api-contract-standard).
 */

export interface Profile {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
  avatar_id?: string | null;
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
}

export interface SaveGameVersion {
  game: string;
  version: string;
  generation: number;
  languageIdRaw?: number | null;
}
