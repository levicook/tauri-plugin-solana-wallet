# tauri-plugin-solana-wallet

EXPERIMENTAL Tauri Plugin for securely interacting with and managing a Solana Wallet

The main idea is to see if we can create a "wallet adapter" for tauri based Solana apps.

Goal: Simplify building and delivering "tapps" (Tauri Apps)

## Rough notes on workflows this plugin should support:

Provide features to support the following workflows:
 * Create Wallet
 * Import Wallet
 * Export Wallet
 * Delete Wallet
 * List Wallets
 * Connect Ledger
 * Connect Keystone
 * Grind Vanity Address?

Create Wallet
- store mnemonic, return derivation path & public key

Import Wallet
- store mnemonic, return derivation path & public key

Export Wallet
- pass derivation path & public key, return mnemonic
- pass derivation path & public key, return bytes


Possible signatures?
- write_keypair(Keypair) -> (Pubkey, DerivationPath) // format!("%X", keypair.to_bytes());
- read_keypair(...) -> (Keypair)
- sign_message + try_sign_message()

## Rough notes on managing dependencies:

* Favor security
* Limit dependencies and supply chain attack vector
* But don't reinvent wheels where community has an established solution; eg:
  * prefer the solana-sdk
  * prefer ultra simplistic UX
  * zero runtime surprises, etc.