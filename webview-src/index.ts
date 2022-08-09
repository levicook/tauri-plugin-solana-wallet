import { invoke } from '@tauri-apps/api/tauri'

export async function execute() {
  await invoke('plugin:solana-wallet|execute')
}
