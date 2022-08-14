use std::fs::create_dir_all;

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use rusqlite::{params, Connection};
use serde::{ser::Serializer, Serialize};
use solana_sdk::{signature::keypair_from_seed, signer::Signer};
use tauri::{
    async_runtime::Mutex,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, State, Window,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Keyring(#[from] keyring::Error),

    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),

    #[error(transparent)]
    Tauri(#[from] tauri::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type Result<T> = std::result::Result<T, Error>;

const PLUGIN_NAME: &str = "solana-wallet";

const SQL_CREATE_MNEMONIC_METADATA: &str = "
    CREATE TABLE IF NOT EXISTS mnemonic_metadata (
        public_key TEXT PRIMARY KEY,
        language_code TEXT,
        imported_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
    );
";

const SQL_INSERT_MNEMONIC_METADATA: &str = "
    INSERT INTO mnemonic_metadata (
        public_key, language_code
    ) VALUES (
        ?1, ?2
    );
";

const SQL_DELETE_MNEMONIC_METADATA: &str = "
    DELETE FROM mnemonic_metadata WHERE public_key = ?1;
";

const SQL_SELECT_MNEMONIC_METADATA: &str = "
    SELECT public_key, language_code, unixepoch(imported_at) FROM mnemonic_metadata ORDER by imported_at;
";

struct Database {
    conn: Mutex<rusqlite::Connection>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MnemonicMetadata {
    public_key: String,
    language_code: String,
    imported_at: u64,
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
            generate_mnemonic_phrase,
            import_mnemonic,
            export_mnemonic,
            delete_mnemonic,
            fetch_mnemonic_metadatas,
        ])
        .setup(|app| {
            let app_path = app
                .path_resolver()
                .app_dir()
                .expect("failed to get app_dir");

            create_dir_all(app_path.clone())?;

            let path = app_path.join("solana-wallet.db");

            let conn = Connection::open(path).expect("failed to open db");
            init_db(&conn)?;
            let db = Database {
                conn: Mutex::new(conn),
            };
            app.manage(db);
            // TODO figure out when I can close the connection
            Ok(())
        })
        .build()
}

fn init_db(conn: &Connection) -> Result<()> {
    // conn.execute("DROP TABLE IF EXISTS mnemonic_metadata", [])?;
    conn.execute(SQL_CREATE_MNEMONIC_METADATA, [])?;
    Ok(())
}

#[tauri::command]
fn generate_mnemonic_phrase(word_count: usize, language_code: String) -> Result<String> {
    // let language_code = language_code.unwrap_or_else(|| String::from("en"));
    let mnemonic_type = MnemonicType::for_word_count(word_count)?;
    let language = Language::from_language_code(&language_code);
    let mnemonic = Mnemonic::new(mnemonic_type, language.unwrap_or_default());
    Ok(mnemonic.into_phrase())
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MnemonicImported {
    public_key: String,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MnemonicDeleted {
    public_key: String,
}

#[tauri::command]
async fn import_mnemonic<R: Runtime>(
    window: Window<R>,
    state: State<'_, Database>,
    phrase: String,
    language_code: String,
    password: Option<String>,
) -> Result<String> {
    // let language_code = language_code.unwrap_or_else(|| String::from("en"));
    let language = Language::from_language_code(&language_code);
    let mnemonic = Mnemonic::from_phrase(&phrase, language.unwrap_or_default())?;

    let password = password.unwrap_or_else(|| String::from(""));
    let seed = Seed::new(&mnemonic, &password);
    let keypair = keypair_from_seed(seed.as_bytes()).unwrap();
    let public_key = keypair.pubkey();

    // save public_key and language in sqlite
    let conn = state.conn.lock().await;
    let mut stmt = conn.prepare_cached(SQL_INSERT_MNEMONIC_METADATA)?;
    stmt.execute(params![public_key.to_string(), language_code])?;

    // save bip39 seed password in keyring
    let entry = format!("{}-seed", public_key.to_string());
    let entry = keyring::Entry::new(&PLUGIN_NAME, &entry);
    entry.set_password(&password).unwrap();

    // save mnemonic in keyring
    let entry = format!("{}-phrase", public_key.to_string());
    let entry = keyring::Entry::new(&PLUGIN_NAME, &entry);
    entry.set_password(&mnemonic.phrase()).unwrap();

    window.emit(
        &format!("{}://mnemonic-imported", PLUGIN_NAME),
        MnemonicImported {
            public_key: public_key.to_string(),
        },
    )?;

    Ok(public_key.to_string())
}

#[tauri::command]
async fn delete_mnemonic<R: Runtime>(
    window: Window<R>,
    state: State<'_, Database>,
    public_key: String,
) -> Result<()> {
    // let public_key: Pubkey = public_key.parse()?; // TODO?

    // delete public_key and language from sqlite
    let conn = state.conn.lock().await;
    let mut stmt = conn.prepare_cached(SQL_DELETE_MNEMONIC_METADATA)?;
    stmt.execute(params![public_key.to_string()])?;

    // delete bip39 seed password from keyring
    let entry = format!("{}-seed", &public_key);
    let entry = keyring::Entry::new(&PLUGIN_NAME, &entry);
    entry.delete_password()?;

    // delete mnemonic from keyring
    let entry = format!("{}-phrase", &public_key);
    let entry = keyring::Entry::new(&PLUGIN_NAME, &entry);
    entry.delete_password()?;

    window.emit(
        &format!("{}://mnemonic-deleted", PLUGIN_NAME),
        MnemonicDeleted { public_key },
    )?;

    Ok(())
}

#[tauri::command]
fn export_mnemonic(_state: State<Database>, public_key: String) -> Result<String> {
    // let public_key: Pubkey = public_key.parse()?; // TODO?
    let _ = public_key;

    Ok("todo".to_string())
}

#[tauri::command]
async fn fetch_mnemonic_metadatas(state: State<'_, Database>) -> Result<Vec<MnemonicMetadata>> {
    let mut results = vec![];
    let conn = state.conn.lock().await;
    let mut stmt = conn.prepare(SQL_SELECT_MNEMONIC_METADATA)?;
    let iter = stmt.query_map([], |row| {
        Ok(MnemonicMetadata {
            public_key: row.get(0)?,
            language_code: row.get(1)?,
            imported_at: row.get(2)?,
        })
    })?;
    for result in iter {
        results.push(result?);
    }
    Ok(results)
}
