import { invoke } from '@tauri-apps/api/tauri'

export async function generateMnemonicPhrase(args: {
  wordCount: number,
  languageCode: String
}): Promise<String> {
  return invoke('plugin:solana-wallet|generate_mnemonic_phrase', args)
}

export async function importMnemonic(args: {
  phrase: string,
  languageCode: String
  password?: String
}): Promise<String> {
  return invoke('plugin:solana-wallet|import_mnemonic', args)
}

export async function fetchMnemonicMetadatas(): Promise<{
  publicKey: String,
  languageCode: String,
  importedAt: number
}[]> {
  return invoke('plugin:solana-wallet|fetch_mnemonic_metadatas')
}