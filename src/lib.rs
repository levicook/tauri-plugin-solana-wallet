use bip39::{Language, Mnemonic, MnemonicType, Seed};
use rusqlite::{params, Connection};
use serde::{ser::Serializer, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::keypair_from_seed, signer::Signer};
use tauri::{
    async_runtime::Mutex,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, State,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Keyring(#[from] keyring::Error),

    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
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
        languate_code TEXT
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

// #[derive(Default)]
struct Database {
    conn: Mutex<rusqlite::Connection>,
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
            generate_mnemonic_phrase,
            import_mnemonic,
            export_mnemonic,
            delete_mnemonic,
            list_mnemonic_pubkeys,
        ])
        .setup(|app| {
            let path = app
                .path_resolver()
                .app_dir()
                .expect("failed to get app_dir")
                .join("solana-wallet.db");
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

#[tauri::command]
async fn import_mnemonic(
    state: State<'_, Database>,
    phrase: String,
    language_code: String,
    password: Option<String>,
) -> Result<Pubkey> {
    // let language_code = language_code.unwrap_or_else(|| String::from("en"));
    let language = Language::from_language_code(&language_code);
    let mnemonic = Mnemonic::from_phrase(&phrase, language.unwrap_or_default())?;

    let password = password.unwrap_or_else(|| String::from(""));
    let seed = Seed::new(&mnemonic, &password);
    let keypair = keypair_from_seed(seed.as_bytes()).unwrap();
    let pubkey = keypair.pubkey();

    // save pubkey and language in sqlite
    let conn = state.conn.lock().await;
    let mut stmt = conn.prepare_cached(SQL_INSERT_MNEMONIC_METADATA)?;
    stmt.execute(params![pubkey.to_string(), language_code])?;

    // save bip39 seed password in keyring
    let entry = format!("{}-seed", pubkey.to_string());
    let entry = keyring::Entry::new(&PLUGIN_NAME, &entry);
    entry.set_password(&password).unwrap();

    // save mnemonic in keyring
    let entry = format!("{}-phrase", pubkey.to_string());
    let entry = keyring::Entry::new(&PLUGIN_NAME, &entry);
    entry.set_password(&mnemonic.phrase()).unwrap();

    Ok(pubkey)
}

#[tauri::command]
async fn delete_mnemonic(state: State<'_, Database>, pubkey: Pubkey) -> Result<()> {
    // TODO validate pubkey is a pubkey?

    // delete pubkey and language from sqlite
    let conn = state.conn.lock().await;
    let mut stmt = conn.prepare_cached(SQL_DELETE_MNEMONIC_METADATA)?;
    stmt.execute(params![pubkey.to_string()])?;

    // delete bip39 seed password from keyring
    let entry = format!("{}-seed", &pubkey);
    let entry = keyring::Entry::new(&PLUGIN_NAME, &entry);
    entry.delete_password()?;

    // delete mnemonic from keyring
    let entry = format!("{}-phrase", &pubkey);
    let entry = keyring::Entry::new(&PLUGIN_NAME, &entry);
    entry.delete_password()?;

    Ok(())
}

#[tauri::command]
fn export_mnemonic(_state: State<Database>, _pubkey: Pubkey) -> Result<String> {
    // TODO validate pubkey is a pubkey
    Ok("todo".to_string())
}

#[tauri::command]
fn list_mnemonic_pubkeys(_state: State<Database>) -> Result<Vec<Pubkey>> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    /*
    use crate::{generate_mnemonic_phrase, Database};
    use rusqlite::Connection;
    use tauri::async_runtime::Mutex;

    #[test]
    fn it_works() {
        let conn = Connection::open_in_memory().unwrap();
        let db = Database {
            conn: Mutex::new(conn),
        };

        let word_count = 12;
        let language_code = String::from("en");
        let mnemonic_phrase = generate_mnemonic_phrase(word_count, language_code).unwrap();
    }

    // use crate::new_mnemonic_phrase;
    #[test]
    fn it_works() {
        let word_count = 12;
        let language_code = String::from("en");
        println!( "{}", new_mnemonic_phrase(word_count, language_code.clone()).unwrap());
    }

    //  use crate::{create_keypair, delete_keypair, read_keypair};
    //  use bip39::{Mnemonic, Seed};
    //  use solana_sdk::{signature::keypair_from_seed, signer::Signer};
    #[test]
    fn it_works() {
        let mnemonic_type = bip39::MnemonicType::Words24;
        let language = bip39::Language::English;
        let mnemonic = Mnemonic::new(mnemonic_type, language);
        let passphrase = "";

        let seed = Seed::new(&mnemonic, &passphrase);
        let kp_a = keypair_from_seed(seed.as_bytes()).unwrap();

        let pk_a = create_keypair(&kp_a).unwrap();
        assert_eq!(pk_a, kp_a.pubkey());

        let kp_b = read_keypair(&pk_a).unwrap();
        assert_eq!(kp_a, kp_b);

        let r = delete_keypair(&kp_b.pubkey());
        assert!(r.is_ok());

        let r = delete_keypair(&kp_b.pubkey());
        assert!(r.is_err());
    }
    */
}
