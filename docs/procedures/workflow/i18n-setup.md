# Procedure: Internazionalizzazione (i18n) e Traduzioni

## Obiettivo

Definisce come introdurre o estendere i18n in PokeTracker in linea con [i18n-standard](../../standards/i18n-standard.md).

## Quando Usare Questa Procedure

- Query: "traduci", "aggiungi lingua", "i18n", "localizzazione", "stringhe tradotte", "nuova lingua", "internazionalizzazione"
- Quando si deve aggiungere o modificare supporto multilingua o file di traduzione
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **i18n Standard**: `docs/standards/i18n-standard.md:1-55`
   - Scelta approccio (solo frontend vs condiviso), struttura chiavi: righe 7-22
   - Librerie, dove mettere stringhe, locale: righe 24-42

2. **Struttura progetto**: `docs/project/project-structure.md`
   - Dove collocare `lib/i18n/` o `lang/`

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/i18n-standard.md:7-22` — Decidere se solo frontend o condiviso; chiavi con namespacing (es. `common.save`, `profile.title`)
2. [ ] Struttura file: `src/lib/i18n/locales/<locale>.json` (o per-namespace) come da `i18n-standard.md:34-38`
3. [ ] Lingua di default e fallback; salvataggio locale in preferences/config (`i18n-standard.md:40-42`)
4. [ ] Se si usa sveltekit-i18n o typesafe-i18n: config loaders, load in +layout, uso `$t()` in componenti (`i18n-standard.md:24-30`)
5. [ ] Per “aggiungi traduzione”: aggiungere chiave nei file di ogni locale supportato; mantenere fallback se chiave mancante
6. [ ] Per “nuova lingua”: nuovo file `<locale>.json` (o cartella) e registrare il locale nella config della libreria scelta

## Riferimenti Standard

- `docs/standards/i18n-standard.md:1-55` — Approccio, chiavi, librerie, locale
- `docs/project/project-structure.md` — Posizione file

## Note

- Per Tauri, risorse condivise (file in bundle) sono opzionali; vedi [i18n-standard](../../standards/i18n-standard.md) sezione “Frontend + risorse condivise”.
