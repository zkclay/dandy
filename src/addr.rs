use bip39::{Mnemonic, MnemonicType, Language, Seed};
use sha2::{Digest, Sha256};
use rand_chacha::ChaCha12Rng;
use rand_chacha::rand_core::SeedableRng;

use dusk_pki::SecretSpendKey;
use dusk_bytes::Serializable;

pub fn generate_addr() -> (String, String, String, String) {
    let seed_info = create_seed_from_memonmic();
    let keys = create_spend_keys_from_seed(&seed_info.0, 0);
    (seed_info.1, keys.0, keys.1, keys.2)
}   

fn create_seed_from_memonmic() -> ([u8; 64], String) {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let phrase: &str = mnemonic.phrase();
    
    let seed = Seed::new(&mnemonic, "");

    let seed_bytes: &[u8] = seed.as_bytes();
    let result: [u8; 64] = seed_bytes.try_into().expect("Slice with incorrect length");
    (result, String::from(phrase))
}

fn create_spend_keys_from_seed(seed: &[u8; 64], index: u64) -> (String, String, String) {
    let mut hash = Sha256::new();

    hash.update(seed);
    hash.update(index.to_le_bytes());
    hash.update(b"SSK");

    let hash = hash.finalize().into();
    let mut rng = ChaCha12Rng::from_seed(hash);

    let ssk = SecretSpendKey::random(&mut rng);
    let addr = ssk.public_spend_key();
    let view_key = ssk.view_key();

    let ssk_string =  bs58::encode(ssk.to_bytes()).into_string();
    let addr_string = bs58::encode(addr.to_bytes()).into_string();
    let view_key_string = bs58::encode(view_key.to_bytes()).into_string();

    (addr_string, view_key_string, ssk_string)
}