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
    ])
}
