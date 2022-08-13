export declare function generateMnemonicPhrase(args: {
    wordCount: number;
    languageCode: String;
}): Promise<String>;
export declare function importMnemonic(args: {
    phrase: string;
    languageCode: String;
    password?: String;
}): Promise<String>;
export declare function deleteMnemonic(publicKey: String): Promise<{}>;
export declare function fetchMnemonicMetadatas(): Promise<{
    publicKey: String;
    languageCode: String;
    importedAt: number;
}[]>;
