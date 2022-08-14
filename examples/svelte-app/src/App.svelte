<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { onDestroy, onMount } from "svelte";

  import {
    WalletEvents,
    deleteMnemonic,
    fetchMnemonicMetadatas,
    generateMnemonicPhrase,
    importMnemonic,
    type MnemonicDeleted,
    type MnemonicImported,
  } from "tauri-plugin-solana-wallet-api";

  function _importMnemonic(phrase) {
    let args = { phrase, languageCode: "en" };
    importMnemonic(args).catch(complain);
  }

  function complain(err: any) {
    console.error("boo", err);
  }

  function _generateMnemonic() {
    const args = { wordCount: 12, languageCode: "en" };
    generateMnemonicPhrase(args).then(_importMnemonic).catch(complain);
  }

  function _deleteMnemonic(publicKey) {
    deleteMnemonic(publicKey).catch(complain);
  }

  let mnemonicMetadatas = [];
  function _listPubkeys() {
    fetchMnemonicMetadatas()
      .then((res) => {
        mnemonicMetadatas = res;
      })
      .catch(complain);
  }

  const unlisteners = [];

  onMount(async () => {
    _listPubkeys();
    unlisteners.push(
      appWindow.listen<MnemonicDeleted>(
        WalletEvents.MnemonicDeleted,
        (_event) => {
          _listPubkeys();
        }
      )
    );
    unlisteners.push(
      appWindow.listen<MnemonicImported>(
        WalletEvents.MnemonicImported,
        (_event) => {
          _listPubkeys();
        }
      )
    );
  });

  onDestroy(async () => {
    unlisteners.forEach((unlisten) => {
      unlisten();
    });
  });
</script>

<div>
  <button on:click={_generateMnemonic}>Generate Mnemonic</button>

  <h1>Mnemonic Public Keys {mnemonicMetadatas.length}</h1>
  <ol>
    {#each mnemonicMetadatas as mm}
      <li>
        <pre>{mm.publicKey} | {mm.languageCode} | {mm.importedAt}</pre>
        <button on:click={_deleteMnemonic(mm.publicKey)}>Delete Mnemonic</button
        >
        <button>Export Mnemonic</button>
        <button>Add Derivation</button>
      </li>
    {/each}
  </ol>
</div>
