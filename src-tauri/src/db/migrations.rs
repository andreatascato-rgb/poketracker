//! Migrazioni schema SQLite. Vedi docs/standards/database-migrations-standard.md.

use rusqlite_migration::{Migrations, M};

/// Definisce tutte le migrazioni. Non modificare migrazioni già applicate;
/// aggiungere sempre una nuova voce per cambi schema (database-migrations-standard § best practice).
pub fn migrations() -> Migrations<'static> {
    Migrations::new(vec![
        M::up(
            "CREATE TABLE IF NOT EXISTS profiles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS app_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
        ),
        M::up(
            "INSERT OR IGNORE INTO profiles (id, name) VALUES ('default', 'Default');
             INSERT OR IGNORE INTO app_state (key, value) VALUES ('active_profile_id', 'default');",
        ),
        M::up(
            "DELETE FROM profiles WHERE id = 'default';
             DELETE FROM app_state WHERE key = 'active_profile_id';",
        ),
        // SQLite non ammette DEFAULT (datetime('now')) in ALTER ADD COLUMN → uso valore costante e poi backfill
        M::up("ALTER TABLE profiles ADD COLUMN created_at TEXT NOT NULL DEFAULT '1970-01-01 00:00:00';"),
        M::up("ALTER TABLE profiles ADD COLUMN updated_at TEXT NOT NULL DEFAULT '1970-01-01 00:00:00';"),
        M::up("UPDATE profiles SET created_at = datetime('now'), updated_at = datetime('now') WHERE created_at = '1970-01-01 00:00:00';"),
        M::up("ALTER TABLE profiles ADD COLUMN avatar_id TEXT;"),
        // Pokedex personale: stato visto/catturato per specie e profilo (docs/project/pokedex-personal.md).
        M::up(
            "CREATE TABLE IF NOT EXISTS pokedex_state (
                profile_id TEXT NOT NULL,
                species_id INTEGER NOT NULL,
                status TEXT NOT NULL CHECK (status IN ('unseen', 'seen', 'caught')),
                PRIMARY KEY (profile_id, species_id),
                FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
            );",
        ),
        // Archivio errori (Impostazioni → Errori): log persistente per supporto/assistente AI (notifications-and-error-archive).
        M::up(
            "CREATE TABLE IF NOT EXISTS error_archive (
                id TEXT PRIMARY KEY,
                at TEXT NOT NULL,
                type TEXT NOT NULL,
                detail TEXT NOT NULL
            );",
        ),
    ])
}
