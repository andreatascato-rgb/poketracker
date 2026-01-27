//! Layer database: connessione SQLite, migrazioni, accesso dati.
//! Vedi docs/standards/database-migrations-standard.md, docs/project/project-structure.md.

mod connection;
mod migrations;

pub use connection::open_or_create;
