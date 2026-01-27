# Database Conoscenza - PokeTracker

## Obiettivo

Definisce come gestire il database di conoscenza (Pokemon, giochi, mosse, nature, etc.) per l'applicazione PokeTracker.

## Analisi Dimensioni Dati

### Numero Totale Pokemon

- **Pokemon totali**: ~1025+ (Gen 1-9, incluse forme alternative)
- **Forme alternative**: Mega, Gigamax, forme regionali, etc.
- **Totale entità**: ~1500+ considerando tutte le varianti

### Stime Dimensioni

**Dati Strutturati (senza immagini):**
- **Singolo Pokemon**: ~2-5 KB (nome, stats, mosse, abilità, evoluzioni, descrizioni)
- **Database Pokemon completo**: ~5-10 MB (solo dati testuali/strutturati)
- **Mosse complete**: ~2-3 MB (tutte le mosse con dettagli)
- **Abilità**: ~500 KB
- **Nature**: ~50 KB (25 nature)
- **Tipi e relazioni**: ~100 KB
- **Giochi e versioni**: ~200 KB
- **Totale dati strutturati**: ~10-15 MB

**Immagini/Asset:**
- **Sprite Pokemon (32x32)**: ~5-10 KB per sprite
- **Tutti sprite Pokemon**: ~50-100 MB
- **Icone tipi**: ~100 KB
- **Immagini ad alta risoluzione**: ~500 KB - 1 MB per Pokemon
- **Tutte immagini HD**: ~500 MB - 1.5 GB

**Wiki Testuale:**
- **Descrizioni Pokemon**: ~1-2 KB per Pokemon
- **Wiki completa (solo testo)**: ~2-5 MB
- **Wiki con immagini**: dipende da risoluzione

### Totale Stimato

**Configurazione Minima (solo dati essenziali):**
- Dati strutturati: ~10-15 MB
- Sprite base: ~50-100 MB
- **Totale**: ~60-115 MB

**Configurazione Completa:**
- Dati strutturati: ~10-15 MB
- Sprite + immagini: ~500 MB - 1.5 GB
- Wiki completa: ~5-10 MB
- **Totale**: ~515 MB - 1.5 GB

## Soluzione Proposta

### Architettura Ibrida

**Database SQLite** per dati strutturati:
- Pokemon (stats, mosse, abilità, etc.)
- Mosse complete
- Abilità
- Nature
- Tipi e relazioni
- Giochi e versioni
- **Dimensione**: ~10-15 MB compresso

**File Assets Separati** per immagini:
- Sprite Pokemon in cartella `resources/sprites/`
- Icone tipi in `resources/icons/`
- Immagini opzionali in `resources/images/`
- **Gestione**: Download opzionale, cache locale

**Wiki Testuale** nel database:
- Descrizioni Pokemon
- Informazioni strategiche
- Dati wiki essenziali
- **Dimensione**: ~2-5 MB

### Strategia Download

**Base (sempre incluso):**
- Database SQLite con tutti i dati strutturati
- Sprite base (32x32 o 64x64)
- **Totale base**: ~60-100 MB

**Opzionale (download on-demand):**
- Immagini ad alta risoluzione
- Wiki completa con immagini
- Asset aggiuntivi

### Vantaggi

✅ **App leggera**: Base ~60-100 MB
✅ **Offline completo**: Tutti i dati essenziali sempre disponibili
✅ **Flessibile**: Download asset opzionali quando servono
✅ **Scalabile**: Aggiungere nuovi asset senza modificare database
✅ **Performance**: Query SQLite veloci per ricerca

## Fonti Dati

### Opzioni

**1. PokeAPI:**
- API pubblica gratuita
- Dati completi e aggiornati
- Formato JSON
- Richiede conversione in SQLite

**2. Database Custom:**
- Creare database da zero
- Controllo completo
- Richiede più lavoro iniziale

**3. Combinazione:**
- Usare PokeAPI come base
- Arricchire con dati custom
- Mantenere aggiornato

### Consigliato: PokeAPI + Custom

- Usare PokeAPI per dati base
- Aggiungere dati custom (strategie, etc.)
- Script per generare database SQLite

## Aggiornamenti

### Sistema Aggiornamenti

- Controllo versioni database all'avvio
- Download aggiornamenti incrementali
- Backup database prima di aggiornare
- Rollback se aggiornamento fallisce

## Note

Questa architettura permette:
- App base leggera e funzionale
- Funzionamento offline completo
- Espandibilità con asset opzionali
- Performance ottimali per ricerca e query
