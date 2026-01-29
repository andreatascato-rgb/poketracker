# Analisi: navigazione "Profilo vs non-profilo" vs "tutto contestato dal profilo"

## Contesto

Dalla lettura di `.cursorrules`, `architecture-overview`, `glossary`, `multi-profile` e dal layout attuale:

- **PokeTracker** = app desktop (Tauri + Svelte) per gestire più **profili/allenatori**, ognuno con Pokedex, salvataggi, statistiche separati. Wiki e archivio errori sono globali; le impostazioni (inclusa gestione profili) sono globali.
- **Ora**: sidebar con sezioni **Allenatore** (Dashboard, Statistiche, Pokedex, Salvataggi), **Editor**, **Wiki**, **Archivio**, **Impostazioni**. Il profilo attivo è in topbar (selettore) e nel breadcrumb come primo elemento; i dati “profilo” sono già filtrati da `activeProfile`.

Domanda: ha più senso **mantenere** la divisione “sezione Profilo vs resto” o **appiattire** la nav e far sì che le voci “profilo” siano voci di primo livello, con Impostazioni eventualmente in topbar?

---

## Cosa dipende davvero dal profilo (dati)

| Area              | Dati per profilo? | Note                                      |
|-------------------|-------------------|-------------------------------------------|
| Dashboard         | Sì                | Landing dati allenatore                   |
| Statistiche       | Sì                | Statistiche del profilo attivo            |
| Pokedex           | Sì                | Pokedex personale del profilo             |
| Salvataggi        | Sì                | Percorsi e save associati al profilo      |
| Editor            | No (o opzionale)  | Apertura file .sav; può essere contestuale|
| Wiki              | No                | Knowledge statico (Pokémon, mosse, nature)|
| Archivio → Errori | No                | Errori app, globali                        |
| Impostazioni      | No                | Gestione profili, cartella main, ecc.     |

Quindi: **a livello dati** la distinzione “profilo / non profilo” è già corretta. La scelta è solo **architettura dell’informazione** (come organizzare le voci in sidebar/topbar).

---

## Opzione A (attuale): "Profilo" come sezione

- Sidebar: Editor | **Allenatore** (Dashboard, Statistiche, Pokedex, Salvataggi) | Wiki | Archivio | Impostazioni.
- Pro: rende esplicito che Dashboard/Statistiche/Pokedex/Salvataggi sono “del profilo”; raggruppa per concetto.
- Contro: due livelli per le funzioni più usate; “Allenatore” è un contenitore senza pagina propria (la dashboard è la landing); un click in più per arrivare alle sottovoci.

---

## Opzione B (proposta): Nav piatta, profilo = contesto; Impostazioni in topbar

- Sidebar: Editor | Dashboard | Statistiche | Pokedex | Salvataggi | Wiki | Archivio.  
  Topbar: breadcrumb (Profilo → …) + selettore profilo + eventuale pulsante Impostazioni (+ watcher, ecc.).
- Le stesse route (`/profilo/dashboard` → `/dashboard` o si tengono i path e si cambia solo la nav): i dati restano filtrati da `activeProfile`.
- Pro: meno livelli, accesso diretto a Dashboard/Pokedex/Statistiche/Salvataggi; il contesto “profilo” è già chiaro da topbar e breadcrumb; Impostazioni come “meta” in topbar è coerente (non è contenuto di gioco).
- Contro: sidebar con più voci; bisogna definire dove e come aprire le sottosezioni Impostazioni (dropdown, drawer, pagina `/impostazioni` raggiungibile da topbar).

---

## Raccomandazione

**Ha più senso la soluzione “piatta” (Opzione B)** per questi motivi:

1. **Il profilo è già il contesto globale**: è in topbar e nel breadcrumb. Non serve ribadirlo con una sezione “Allenatore”: le voci Dashboard, Statistiche, Pokedex, Salvataggi sono ovviamente “del profilo” perché l’app è multi-profilo e il profilo è sempre visibile.
2. **Riduzione di profondità**: le quattro voci profilo sono il cuore dell’uso (dati allenatore). Averle di primo livello riduce click e chiarisce subito “dove trovo Pokedex / Statistiche”.
3. **Impostazioni come meta**: mettere Impostazioni in topbar (es. icona ingranaggio che apre menu/drawer/pagina) le tratta per quello che sono: configurazione app, non una sezione di contenuto alla pari di Wiki o Pokedex. Le sottosezioni (Profili, eventuale cartella main, ecc.) si gestiscono dentro quel contesto (sottopagine o pannelli).
4. **Coerenza con il modello dati**: non stai facendo “tutto dipendente dal profilo” a livello dati (Wiki e Archivio restano globali); stai solo **organizzando la nav** in modo che il profilo sia il contesto implicito per le voci che già lo usano, senza un contenitore “Profilo” ridondante.

**Cosa mantenere invariato**:  
- Filtro dati per `activeProfile` su Dashboard, Statistiche, Pokedex, Salvataggi.  
- Wiki, Archivio, Impostazioni senza filtro profilo.  
- Breadcrumb che inizia con il nome profilo sulle pagine “profilo” (e eventualmente più neutro su Wiki/Archivio/Impostazioni se vuoi).

**Passi concreti suggeriti** (solo direzione, da adattare agli standard di progetto):

1. **Sidebar**: togliere la sezione “Allenatore” e mettere come voci di primo livello Dashboard, Statistiche, Pokedex, Salvataggi (stessi `href` o migrazione a `/dashboard`, `/statistiche`, ecc. se vuoi URL più corti).
2. **Impostazioni**: spostare in topbar (icona + dropdown o link a `/impostazioni`); per le sottosezioni: o una pagina Impostazioni con sub-nav, o un drawer con elenco (Profili, …).
3. **Breadcrumb e titoli**: adattare così che “Profilo” non compaia come sezione intermedia; primo elemento resta il nome profilo dove ha senso.
4. **Route**: puoi mantenere `/profilo/dashboard` ecc. per compatibilità e solo cambiare la nav, oppure migrare a `/dashboard` e reindirizzare i vecchi path.

Se vuoi, il passo successivo può essere uno schema preciso di sidebar/topbar (liste di voci e href) e dove collocare il pulsante Impostazioni e le sue sottosezioni, così da allinearlo agli standard in `docs/standards/` (es. breadcrumb, design system).
