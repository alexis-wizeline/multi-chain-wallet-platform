use std::str::FromStr;

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use ed25519_dalek_bip32::{DerivationPath, ExtendedSigningKey};
use solana_sdk::{signature::Keypair, signer::SeedDerivable};
use uuid::Uuid;

// FIXME: Add errors

pub fn create_wallet() -> (String, Keypair, String) {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let keypair = derive_from_mnemonic(&mnemonic, 0);

    let wallet_id = Uuid::new_v4().to_string();

    (wallet_id, keypair, mnemonic.to_string())
}

pub fn derive_account(base: &Keypair, index: u32) -> Keypair {
    let derivation_path = der_path(index);

    let extended = ExtendedSigningKey::from_seed(base.secret_bytes())
        .unwrap()
        .derive(&derivation_path)
        .unwrap();

    Keypair::from_seed(&extended.signing_key.to_bytes()).unwrap()
}

fn der_path(index: u32) -> DerivationPath {
    let path = format!("m/44'/501'/{}'/0'", index);
    DerivationPath::from_str(&path).unwrap()
}

fn derive_from_mnemonic(mnemonic: &Mnemonic, index: u32) -> Keypair {
    let seed = Seed::new(mnemonic, "");
    let derivation_path = der_path(index);

    let extended = ExtendedSigningKey::from_seed(seed.as_bytes())
        .unwrap()
        .derive(&derivation_path)
        .unwrap();

    Keypair::from_seed(&extended.signing_key.to_bytes()).unwrap()
}
