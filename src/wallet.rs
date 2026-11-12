// wallet.rs

use secp256k1::{PublicKey, SecretKey};
use sha3::{Keccak256, Digest};

pub struct  Wallet {
    private_key: SecretKey,
    public_key: PublicKey,
    address: String,
}

impl Wallet {
    pub fn new() -> Wallet {
        let (private_key, public_key) =
            secp256k1::generate_keypair(&mut secp256k1::rand::rng());

        let serialized_public_key = public_key.serialize_uncompressed();

        let mut hasher = Keccak256::new();
        hasher.update(&serialized_public_key[1..]);
        let keccak_result = hasher.finalize();

        let address_bytes = &keccak_result[12..];

        let address = format!("0x{}", hex::encode(address_bytes));

        Wallet { private_key, public_key, address }
    }

    pub fn get_private_key(&self) -> String {
        hex::encode(self.private_key.to_secret_bytes())
    }

    pub fn get_public_key(&self) -> String {
        hex::encode(self.public_key.serialize())
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn save_private_key_to_file(&self) -> Result<(), String> {
        let private_key = self.get_private_key();

        std::fs::write("wallet.txt", private_key).map_err(|_| "Could not write the file down".to_string())
    }

    fn read_private_key_from_file() -> Result<SecretKey, String> {
        let private_key = {
            match std::fs::read_to_string("wallet.txt") {
                Ok(value) => { value },
                Err(_) => { return Err("Could not read the file".to_string()) }
            }
        };

        let private_key_decoded: [u8; 32] = match hex::decode(private_key) {
            Ok(key) => {
                match key.try_into() {
                    Ok(value) => { value },
                    Err(_) => { return Err("The length of the private key must be 32 bytes".to_string()) }
                }
            },
            Err(_) => { return Err("Could not convert into hex".to_string()) }
        };

        match SecretKey::from_secret_bytes(private_key_decoded) {
            Ok(value) => { Ok(value) },
            Err(_) => { Err("Invalid private key".to_string()) }
        }
    }

    pub fn load_from_file() -> Result<Wallet, String> {
        let private_key = Self::read_private_key_from_file()?;

        let public_key = PublicKey::from_secret_key(&private_key);

        let serialized_public_key = public_key.serialize_uncompressed();

        let mut hasher = Keccak256::new();
        hasher.update(&serialized_public_key[1..]);
        let keccak_result = hasher.finalize();

        let address_bytes = &keccak_result[12..];

        let address = format!("0x{}", hex::encode(address_bytes));

        Ok(Wallet {
            private_key,
            public_key,
            address
        })
    }

    pub(crate) fn get_private_key_ref(&self) -> &SecretKey {
        &self.private_key
    }
    
    pub(crate) fn get_public_key_ref(&self) -> &PublicKey {
        &self.public_key
    }
}
