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

 - [ ] Network Selection: custom | devnet | testnet | mainnet-beta
  - [ ] RPC Selection?

 - [ ] Portfolio Management
   - [ ] Discover Portfolios (ie: look for balances in derived wallets)
   - [ ] Create Portfolio
   - [ ] Delete Portfolio
   - A portfolio is:
     - A pubkey
     - SOL Balance
     - SPL Balances
     - Network selection
     - Monitored / balance changes generate events
  - Should a portfolio be asset oriented?
  - Portfolio is a bucket of tokens??
  - Tokens are discovered by looking for balances
  - SOL is always listed
  - ^^ this might beyond the scope of this plugin and encroaching on app level worries.
    ... will use these ideas to figure out what functionality the app wants the plugin to provide

** Note everywhere I'm saying "wallet", I basically mean "hot wallet"

** Want to support an "asset first" UX -- ie: show me USDC, then enumerage the wallets / accounts it sits in.

** Rough architectureal thoughts: Monitoring should use pubsub + websocket connections. Keep view state in sqlite. Event processors should update view state (sqlite) then emit window events to let the UI decide what, if anything it should do.

** TBD: figure out where to source tokens from. Probably Solflare Token List (github solflare-wallet/solflare). Maybe bundle a copy? Then download updates with user permission??

## Rough notes on managing dependencies:

* Favor security
* Limit dependencies and supply chain attack vector
* But don't reinvent wheels where community has an established solution; eg:
  * prefer the solana-sdk
  * prefer ultra simplistic UX
  * zero runtime surprises, etc.