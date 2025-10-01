# Changelog

## 1.3.0 (2025-10-01)

### 🔧 Correzione Schema Database

- **Fix**: Rimosso campo `folder_id` dallo struct `FileResult` (Rust)
- **Fix**: Rimosso campo `folder_id` dall'interfaccia `FileResult` (TypeScript)
- **Fix**: Query SQL aggiornata per rimuovere colonna `folder_id`
- **Docs**: Aggiornati DATABASE_CONFIG.md e USAGE_GUIDE.md con schema corretto

### 📊 Schema Database Corretto

```sql
CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    directory TEXT NOT NULL,
    last_modified TIMESTAMP
);
```

## 1.2.0 (2025-10-01)

### 🆕 Aggiunta Colonna `last_modified`

- **Feat**: Aggiunta visualizzazione data ultima modifica file nella tabella risultati
- **Feat**: Aggiunto campo `last_modified` a `FileResult` (backend e frontend)
- **Feat**: Query SQL aggiornata per includere colonna `last_modified`
- **Feat**: Formattazione automatica data in formato `YYYY-MM-DD HH:MM:SS`
- **Feat**: Icona orologio per indicatore visivo data/ora
- **Feat**: Gestione valori NULL con visualizzazione "N/A"
- **Deps**: Aggiunta dipendenza `chrono` per gestione date
- **Docs**: Aggiornati DATABASE_CONFIG.md e USAGE_GUIDE.md con nuovo schema

## 1.1.0 (2025-10-01)

### 🟢 Indicatore Stato Connessione Database

- **Feat**: Aggiunto badge di stato connessione nell'header con pallino colorato
- **Feat**: Test automatico della connessione all'avvio dell'applicazione
- **Feat**: Badge cliccabile per test manuale della connessione
- **Feat**: Pulsante "Testa Connessione" nel modal impostazioni
- **Feat**: Comando Rust `test_connection` per verificare connessione DB
- **Feat**: Stati: Connesso (verde), Disconnesso (rosso), Test (giallo), Sconosciuto (grigio)
- **Docs**: Creato CONNECTION_STATUS.md con guida completa

### ⚙️ Sistema di Impostazioni Database

- **Feat**: Modal impostazioni con configurazione database PostgreSQL
- **Feat**: Campi per Host, Porta, Username, Password, Database
- **Feat**: Salvataggio configurazione in localStorage del browser
- **Feat**: Funzione "Ripristina Default" per reset impostazioni
- **Feat**: Pulsante impostazioni nell'header con icona ingranaggio
- **Feat**: Classe `SettingsState` per gestione stato impostazioni
- **Feat**: Comando `search_files` aggiornato per accettare parametri dinamici
- **UI**: Design coerente con DaisyUI, icone SVG, animazioni smooth

### 🔍 Ricerca File nel Database

- **Feat**: Interfaccia di ricerca con supporto per termini multipli
- **Feat**: Ricerca case-insensitive con operatore PostgreSQL ILIKE
- **Feat**: Visualizzazione risultati in tabella zebra-striped
- **Feat**: Stati di caricamento con spinner
- **Feat**: Gestione errori con alert
- **Feat**: Classe `SearchState` per gestione stato ricerca
- **Feat**: Comando Rust `search_files` per query database
- **Feat**: Limite 100 risultati per termine di ricerca
- **Feat**: Rimozione duplicati nei risultati

### 🎨 Interfaccia Utente

- **UI**: Design moderno con TailwindCSS e DaisyUI
- **UI**: Layout responsive e accessibile
- **UI**: Icone SVG personalizzate per ogni elemento
- **UI**: Alert informativi colorati (success, error, warning, info)
- **UI**: Tabella risultati con hover effects
- **UI**: Pulsante reset per pulire la ricerca

### 🗄️ Database PostgreSQL

- **Feat**: Supporto per database PostgreSQL con tokio-postgres
- **Feat**: Connection pooling con bb8 e bb8-postgres
- **Feat**: Struct `DatabaseConfig` per configurazione tipizzata
- **Feat**: Gestione colonne: `id`, `filename`, `directory`, `last_modified`

### 📚 Documentazione

- **Docs**: DATABASE_CONFIG.md con guida configurazione database
- **Docs**: USAGE_GUIDE.md con istruzioni complete
- **Docs**: CONNECTION_STATUS.md con guida stato connessione
- **Docs**: Schema SQL per tabelle `files` e `folders`

## 1.0.0 (2024-11-15)

- Feat: implement basic ci/cd config
