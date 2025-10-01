# Configurazione Database PostgreSQL

## Configurazione della Connessione tramite UI

🎉 **NOVITÀ**: L'applicazione ora include un'interfaccia grafica per configurare la connessione al database!

### Come configurare il database:

1. **Clicca sull'icona delle impostazioni** (⚙️) nell'angolo in alto a destra dell'applicazione
2. **Inserisci i parametri di connessione:**
   - **Host**: l'indirizzo del server PostgreSQL (es: `localhost`)
   - **Porta**: la porta del server (default: `5432`)
   - **Username**: il nome utente PostgreSQL (es: `postgres`)
   - **Password**: la password dell'utente
   - **Nome Database**: il nome del database (es: `nas_scanner`)
3. **Clicca su "Salva"** per salvare le impostazioni
4. Le impostazioni vengono salvate nel **localStorage** del browser e persisteranno tra le sessioni

### Valori di Default

Se non configuri manualmente la connessione, l'applicazione userà questi valori predefiniti:
- **Host**: `localhost`
- **Porta**: `5432`
- **Username**: `postgres`
- **Password**: (vuota)
- **Database**: `nas_scanner`

## Configurazione Manuale (Opzionale)

Se preferisci modificare i valori di default nel codice, puoi modificare il file:
`src/lib/commands.svelte.ts` alla riga ~85:

```typescript
const DEFAULT_CONFIG: DatabaseConfig = {
	host: 'localhost',
	port: 5432,
	user: 'postgres',
	password: '',
	database: 'nas_scanner'
};
```

## Schema del Database Richiesto

L'applicazione si aspetta che il database abbia la seguente struttura:

### Tabella `files`
```sql
CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    directory TEXT NOT NULL,
    last_modified TIMESTAMP
);
```

## Query di Ricerca

L'applicazione utilizza l'operatore `ILIKE` di PostgreSQL per effettuare ricerche case-insensitive con logica **AND**:

```sql
-- Con più termini (es: "documento report")
SELECT id, filename, directory, last_modified
FROM files 
WHERE filename ILIKE '%documento%' AND filename ILIKE '%report%'
LIMIT 100
```

**Nota**: La ricerca cerca file che contengono **TUTTI** i termini inseriti nel nome del file.

## Come Compilare e Avviare

1. **Assicurati di aver configurato la stringa di connessione corretta**
2. **Installa le dipendenze Node.js:**
   ```bash
   bun install
   ```

3. **Compila l'applicazione Tauri:**
   ```bash
   bun run tauri dev
   ```

## Note di Sicurezza

⚠️ **IMPORTANTE**: In produzione, NON includere mai le credenziali del database direttamente nel codice sorgente. 

Considera l'utilizzo di:
- Variabili d'ambiente
- File di configurazione esterni (non versionati in Git)
- Secret management systems

## Funzionalità Implementate

✅ Ricerca con più termini (separati da spazi o virgole)
✅ Ricerca case-insensitive
✅ Visualizzazione risultati in tabella
✅ Stati di caricamento
✅ Gestione errori
✅ UI responsive con DaisyUI
✅ Limite di 100 risultati per termine di ricerca
✅ **Configurazione database tramite UI con modal**
✅ **Salvataggio impostazioni in localStorage**
✅ **Pulsante per ripristinare impostazioni di default**

## Prossimi Miglioramenti Suggeriti

- [ ] Aggiungere paginazione per gestire grandi quantità di risultati
- [ ] Implementare ordinamento dei risultati
- [ ] Aggiungere filtri avanzati (per tipo file, data, dimensione, ecc.)
- [ ] Implementare connection pooling con bb8-postgres
- [ ] Ricerca full-text con PostgreSQL
- [ ] Test di connessione al database dal modal impostazioni
- [ ] Crittografia delle password salvate
- [ ] Esportazione risultati in CSV/Excel
