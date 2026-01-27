# Strategia Testing - PokeTracker

## Obiettivo

Definisce la strategia di testing per l'applicazione PokeTracker (Tauri + Rust + Svelte).

## Best Practice 2026

### Testing Rust

**Unit Tests:**
- Test per ogni modulo/service
- Test isolati con mock quando necessario
- Coverage target: 80%+ per logica critica
- Test in `#[cfg(test)]` modules

**Integration Tests:**
- Test per interazione tra moduli
- Test database con database in-memory o test DB
- Test comandi Tauri con test harness

**Property Testing:**
- Usare `proptest` per test basati su proprietà
- Testare invarianti (es. validazione dati)

**Strumenti:**
- `cargo test` per unit/integration tests
- `criterion` per benchmark
- `mockall` per mocking

### Testing Tauri

**Comandi IPC:**
- Test comandi Tauri in isolamento
- Mock file system quando necessario
- Test error handling

**E2E Testing:**
- Test end-to-end con app reale
- Test interazione frontend-backend
- Test UI con strumenti appropriati

### Testing Svelte

**Component Testing:**
- Test componenti isolati
- Test stores
- Test interazioni utente

**Strumenti:**
- `@testing-library/svelte` per componenti
- `vitest` o `jest` per test runner
- Test manuali per UI complessa

## Strategia Testing

### Livelli Testing

**1. Unit Tests (Rust):**
- Ogni funzione critica
- Logica business
- Validazione dati
- Error handling

**2. Integration Tests (Rust):**
- Interazione moduli
- Database operations
- File system operations
- Sidecar communication

**3. Component Tests (Svelte):**
- Componenti isolati
- Stores
- Servizi frontend

**4. E2E Tests:**
- Flussi completi utente
- Interazione frontend-backend
- Operazioni critiche (save, load, edit)

### Test Critici

**Parser:**
- Test parsing file salvataggio validi
- Test file corrotti
- Test formati diversi
- Test sidecar communication

**Database:**
- Test migrazioni
- Test query complesse
- Test transazioni
- Test recovery da corruzione

**Profili:**
- Test creazione/eliminazione profili
- Test switch profili
- Test isolamento dati

**Editor:**
- Test validazione modifiche
- Test overwrite file
- Test backup automatico

## Coverage

### Target Coverage

- **Logica critica**: 90%+ (parser, database, validazione)
- **Business logic**: 80%+ (services, managers)
- **UI**: 60%+ (componenti principali)
- **Totale**: 75%+

### Strumenti Coverage

- `cargo-tarpaulin` per Rust
- `vitest --coverage` per Svelte
- Report combinati

## CI/CD Testing

### Pipeline Testing

**Pre-commit:**
- Linting (clippy, eslint)
- Format check (rustfmt, prettier)
- Unit tests rapidi

**CI:**
- Tutti i test
- Coverage report
- Integration tests
- Build test

## Note

Questa strategia permette:
- Qualità codice alta
- Rilevamento bug precoce
- Refactoring sicuro
- Documentazione vivente (test come esempi)
