<script lang="ts">
  import {
    deleteMnemonic,
    fetchMnemonicMetadatas,
    generateMnemonicPhrase,
    importMnemonic,
  } from "tauri-plugin-solana-wallet-api";

  let mnemonicMetadatas = [];

  function _importMnemonic(phrase) {
    importMnemonic({
      phrase,
      languageCode: "en",
    })
      .then((_) => {
        _listPubkeys();
      })
      .catch(fail);
  }

  function fail(err: any) {
    console.error("boo", err);
  }

  function _generateMnemonic() {
    const args = { wordCount: 12, languageCode: "en" };
    generateMnemonicPhrase(args).then(_importMnemonic).catch(fail);
  }

  function _listPubkeys() {
    fetchMnemonicMetadatas()
      .then((res) => {
        mnemonicMetadatas = res;
      })
      .catch(fail);
  }

  function _deleteMnemonic(publicKey) {
    deleteMnemonic(publicKey)
      .then(() => {
        _listPubkeys();
      })
      .catch(fail);
  }

  _listPubkeys();
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
      </li>
    {/each}
  </ol>
</div>