# NAS File Finder - Guida Rapida

## 🚀 Funzionalità

Applicazione desktop per la ricerca di file in un database PostgreSQL con interfaccia moderna e intuitiva.

### Caratteristiche Principali

- 🔍 **Ricerca Intelligente**: Cerca file con più termini contemporaneamente
- ⚙️ **Configurazione Facile**: Gestisci le impostazioni del database tramite interfaccia grafica
- 💾 **Salvataggio Automatico**: Le tue impostazioni vengono salvate automaticamente
- 🎨 **Design Moderno**: Interfaccia pulita con DaisyUI e TailwindCSS
- ⚡ **Veloce e Reattivo**: Backend in Rust con Tauri per prestazioni ottimali

## 📋 Prerequisiti

1. **Node.js** o **Bun** installato
2. **Rust** installato (per compilare Tauri)
3. **PostgreSQL** con il database `nas_scanner` configurato

## 🛠️ Installazione

```bash
# Installa le dipendenze
bun install

# Oppure con npm
npm install
```

## ▶️ Avvio

```bash
# Modalità sviluppo
bun run tauri dev

# Oppure con npm
npm run tauri dev
```

## 🗄️ Configurazione Database

### Schema Database Richiesto

Assicurati che il tuo database PostgreSQL abbia la seguente struttura:

```sql
-- Tabella files (richiesta)
CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    directory TEXT NOT NULL,
    last_modified TIMESTAMP
);
```

### Configurazione Connessione

1. **Avvia l'applicazione**
2. **Clicca sull'icona delle impostazioni** (⚙️) in alto a destra
3. **Inserisci i dati di connessione:**
   - Host: indirizzo del server (es: `localhost`)
   - Porta: porta PostgreSQL (default: `5432`)
   - Username: nome utente del database
   - Password: password del database
   - Database: nome del database (es: `nas_scanner`)
4. **Salva** le impostazioni

Le impostazioni vengono salvate localmente e persisteranno tra le sessioni.

## 📖 Come Usare l'Applicazione

### Ricerca File

1. **Inserisci uno o più termini** nel campo di ricerca
   - Puoi separare i termini con **spazi** o **virgole**
   - Esempio: `documento report 2024` o `foto, immagini, png`

2. **Clicca su "Cerca"** per avviare la ricerca
   - La ricerca è **case-insensitive** (maiuscole/minuscole non contano)
   - Trova file con nomi **simili** ai termini inseriti

3. **Visualizza i risultati** nella tabella
   - ID del file
   - Nome del file
   - Directory del file
   - Data ultima modifica

### Gestione Impostazioni

- **Ripristina Default**: Reset delle impostazioni ai valori predefiniti
- **Annulla**: Chiudi il modal senza salvare
- **Salva**: Salva le modifiche e chiudi il modal

## 🔧 Compilazione per Produzione

```bash
# Build dell'applicazione
bun run tauri build

# L'eseguibile sarà generato in:
# src-tauri/target/release/
```

## 📁 Struttura del Progetto

```
.
├── src/                      # Frontend (Svelte + TypeScript)
│   ├── lib/
│   │   ├── commands.svelte.ts   # State management & API calls
│   │   └── Settings.svelte      # Componente modal impostazioni
│   └── routes/
│       └── +page.svelte         # Pagina principale
│
├── src-tauri/                # Backend (Rust + Tauri)
│   └── src/
│       ├── commands.rs          # Comandi Tauri (DB queries)
│       ├── lib.rs              # Entry point library
│       └── main.rs             # Entry point application
│
└── DATABASE_CONFIG.md        # Documentazione dettagliata DB
```

## 🎨 Tecnologie Utilizzate

- **Frontend**: Svelte 5, TypeScript, TailwindCSS, DaisyUI
- **Backend**: Rust, Tauri 2, tokio-postgres
- **Database**: PostgreSQL
- **Build Tool**: Vite, Bun

## 🐛 Troubleshooting

### Errore di Connessione al Database

1. Verifica che PostgreSQL sia in esecuzione
2. Controlla che le credenziali siano corrette nelle impostazioni
3. Assicurati che il database `nas_scanner` esista
4. Verifica che le tabelle siano state create correttamente

### L'applicazione non si avvia

1. Assicurati di aver installato tutte le dipendenze: `bun install`
2. Verifica che Rust sia installato correttamente: `rustc --version`
3. Prova a pulire e ricompilare: 
   ```bash
   cd src-tauri
   cargo clean
   cd ..
   bun run tauri dev
   ```

## 📝 Note di Sicurezza

⚠️ **IMPORTANTE**: Le credenziali del database vengono salvate in chiaro nel localStorage del browser. 

Per un ambiente di produzione, considera:
- Utilizzo di variabili d'ambiente
- Crittografia delle password
- Autenticazione basata su token
- Secret management systems

## 📄 Licenza

MIT License - vedi file LICENSE per dettagli

## 🤝 Contribuire

Contributi, issues e feature requests sono benvenuti!

---

Creato con ❤️ usando Tauri 2 + Svelte 5
