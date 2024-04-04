use bip39::{Mnemonic, MnemonicType, Language, Seed};
use sha2::{Digest, Sha256};
use rand_chacha::ChaCha12Rng;
use rand_chacha::rand_core::SeedableRng;

use dusk_pki::SecretSpendKey;
use dusk_bytes::Serializable;

use std::fs::OpenOptions;
use std::io::Write;
use std::io::Result;

pub fn find_vanity_address(prefix: String, case_sensitive: bool, file_path: &str) {
    let mut counter = 0;
    let interval = 10000;

    println!("Beginning search for prefix: {} caseSensitive: {}", prefix, case_sensitive);

    loop {
        let result = generate_addr();
        let addr_string = if case_sensitive {
            result.1.clone().to_lowercase()
        } else {
            result.1.clone()
        };
    
        if addr_string.to_lowercase().starts_with(&prefix) {
            println!("Address found.");
            append_tuple_to_file(result, file_path).expect("error writing to file");
        }
        
        if counter % interval == 0 && counter > 0 {
            println!("{} addresses searched.", counter);
        }

        counter += 1;
    }
}

fn append_tuple_to_file(tuple: (String, String, String, String), file_path: &str) -> Result<()> {
    let combined_string = format!("{},{},{},{}\n", tuple.0, tuple.1, tuple.2, tuple.3);

    // Open the file in append mode, creating it if it does not exist
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    file.write_all(combined_string.as_bytes())?;

    Ok(())
}

fn generate_addr() -> (String, String, String, String) {
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