//! File system watcher per path sav: quando un file .sav/.dsv monitorato cambia, emette `sav-file-changed`.
//! Usato da set_sav_watched per aggiungere/rimuovere path; il frontend ascolta l’evento e invoca sync_sav_now.

use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, Mutex};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Emitter};

/// Gestisce i path da monitorare e emette `sav-file-changed` quando un file osservato viene modificato.
pub struct SavWatcher {
    watcher: Mutex<RecommendedWatcher>,
    watched: Arc<Mutex<HashSet<String>>>,
    _app: AppHandle,
}

impl SavWatcher {
    /// Crea il watcher e avvia il monitoraggio per i path iniziali.
    pub fn new(app: AppHandle, initial_paths: Vec<String>) -> Result<Self, notify::Error> {
        let watched = Arc::new(Mutex::new(initial_paths.into_iter().collect::<HashSet<_>>()));
        let watched_cb = Arc::clone(&watched);
        let app_emit = app.clone();
        let mut w = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| {
                if let Ok(ev) = res {
                    for p in ev.paths {
                        let s = p.to_string_lossy().into_owned();
                        if watched_cb.lock().map(|g| g.contains(&s)).unwrap_or(false) {
                            eprintln!("[PokeTracker] sav-file-changed: {:?}", s);
                            let _ = app_emit.emit("sav-file-changed", &s);
                        }
                    }
                }
            },
            Config::default(),
        )?;
        for path in watched.lock().map_err(|_| notify::Error::generic("lock"))?.iter() {
            let _ = w.watch(Path::new(path), RecursiveMode::NonRecursive);
        }
        Ok(Self {
            watcher: Mutex::new(w),
            watched,
            _app: app,
        })
    }

    /// Aggiunge un path al watcher.
    pub fn add(&self, path: &str) -> Result<(), notify::Error> {
        let mut w = self.watcher.lock().map_err(|_| notify::Error::generic("lock"))?;
        let mut set = self.watched.lock().map_err(|_| notify::Error::generic("lock"))?;
        if set.insert(path.to_string()) {
            w.watch(Path::new(path), RecursiveMode::NonRecursive)?;
        }
        Ok(())
    }

    /// Rimuove un path dal watcher.
    pub fn remove(&self, path: &str) -> Result<(), notify::Error> {
        let mut w = self.watcher.lock().map_err(|_| notify::Error::generic("lock"))?;
        let mut set = self.watched.lock().map_err(|_| notify::Error::generic("lock"))?;
        if set.remove(path) {
            w.unwatch(Path::new(path))?;
        }
        Ok(())
    }
}
