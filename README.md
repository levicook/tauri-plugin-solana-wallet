# tauri-plugin-solana-wallet

EXPERIMENTAL Tauri Plugin for securely interacting with and managing a Solana Wallet

The main idea is to see if we can create a "wallet adapter" for tauri based Solana apps.

Goal: Simplify building and delivering "tapps" (Tauri Apps)

## Rough notes on workflows this plugin should support:

Provide features to support the following workflows:
 - [x] Create Wallet
 - [x] Import Wallet
 - [x] Delete Wallet
 - [x] List Wallets
 - [ ] Export Wallet
 - [ ] Connect Ledger?
 - [ ] Connect Keystone?
 - [ ] Grind Vanity Address?
 - [ ] Sign Messages

 - [ ] Portfolio Tracking
   - [ ] List Portfolios
   - [ ] Discover Portfolios from Mnemonic? (look for balances in derived wallets)
   - [ ] Create Portfolio
   - [ ] Delete Portfolio
   - A portfolio is:
     - A pubkey
     - SOL Balance
     - SPL Balances
     - Network selection
     - Monitored / balance changes generate events


 - [ ] Network Selection: custom | devnet | testnet | mainnet-beta
  - [ ] RPC Selection?


 ** Note everywhere I'm saying "wallet", I basically mean "hot wallet"

 ** Want to support an "asset first" UX -- ie: show me USDC, then enumerage the wallets / accounts it sits in.

## Rough notes on managing dependencies:

* Favor security
* Limit dependencies and supply chain attack vector
* But don't reinvent wheels where community has an established solution; eg:
  * prefer the solana-sdk
  * prefer ultra simplistic UX
  * zero runtime surprises, etc.