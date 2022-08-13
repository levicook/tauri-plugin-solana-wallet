<script lang="ts">
  import {
    generateMnemonicPhrase,
    importMnemonic,
    fetchMnemonicMetadatas,
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
        console.log(res);
        mnemonicMetadatas = res;
      })
      .catch(fail);
  }

  function _deleteMnemonic(o) {
    console.log('delete:', o);
  }

  _listPubkeys();
</script>

<div>
  <button on:click={_generateMnemonic}>Generate Mnemonic</button>

  {#each mnemonicMetadatas as mmm}
    <li>
      <pre>{mmm.publicKey} | {mmm.languageCode} | {mmm.importedAt}</pre>
      <button on:click={_deleteMnemonic(mmm)}>Delete Mnemonic</button>
    </li>
  {/each}
</div>
