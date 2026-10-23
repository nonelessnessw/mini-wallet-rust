use secp256k1::{PublicKey, SecretKey};
use sha3::{Keccak256, Digest};

struct  Wallet {
    private_key: SecretKey,
    public_key: PublicKey,
    address: String,
}

impl Wallet {
    // Wallet creation
    fn new() -> Wallet {
        // Generate a random private key
        let (private_key, public_key) =
            secp256k1::generate_keypair(&mut secp256k1::rand::rng());

        // Convert the public key to an uncompressed format of bytes
        let serialized_public_key = public_key.serialize_uncompressed();

        // Hash the public key to get the address
        let mut hasher = Keccak256::new();
        hasher.update(&serialized_public_key[1..]);
        let keccak_result = hasher.finalize();

        // Get the last 20 bytes of the hash as the address
        let address_bytes = &keccak_result[12..];

        // Convert the address bytes to a string
        let address = format!("0x{}", hex::encode(address_bytes));
        
        Wallet { private_key, public_key, address }
    }

    fn get_private_key(&self) -> String {
        hex::encode(self.private_key.to_secret_bytes())
    }

    fn get_public_key(&self) -> String {
        hex::encode(self.public_key.serialize())
    }

    fn get_address(&self) -> String {
        self.address.clone()
    }

    fn save_private_key_to_file(&self) -> Result<(), String> {
        let private_key = self.get_private_key();

        std::fs::write("wallet.txt", private_key).map_err(|_| "Не удалось записать файл".to_string())
    }

    fn read_private_key_from_file() -> Result<SecretKey, String> {
        let private_key = {
            match std::fs::read_to_string("wallet.txt") {
                Ok(value) => { value },
                Err(_) => { return Err("Не удалось прочитать файл".to_string()) }
            }
        };

        let private_key_decoded: [u8; 32] = match hex::decode(private_key) {
            Ok(key) => {
                match key.try_into() {
                    Ok(value) => { value },
                    Err(_) => { return Err("Длина приватного ключа должна быть 32 байта".to_string()) }
                }
            },
            Err(_) => { return Err("Не удалось перевести в 16-ную сс".to_string()) }
        };

        match SecretKey::from_secret_bytes(private_key_decoded) {
            Ok(value) => { Ok(value) },
            Err(_) => { Err("32 байта не представляют допустимый секретный ключ".to_string()) }
        }
    }

    fn load_from_file() -> Result<Wallet, String> {
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
}

fn main () {
    let new_wallet: Wallet = Wallet::new();

    println!("Private key: {:?}", new_wallet.get_private_key());
    println!("Public key: {:?}", new_wallet.get_public_key());
    println!("Address: {:?}", new_wallet.get_address());

    match new_wallet.save_private_key_to_file() {
        Ok(_) => {},
        Err(error) => {
            println!("Ошибка: {}", error);
            return;
        }
    };

    let wallet_from_file = match Wallet::load_from_file() {
        Ok(wallet) => { wallet },
        Err(error) => {
            println!("Ошибка: {}", error);
            return;
        }
    };

    println!("Loaded wallet private key: {:?}", wallet_from_file.get_private_key());
    println!("Loaded wallet public key: {:?}", wallet_from_file.get_public_key());
    println!("Loaded wallet address: {:?}", wallet_from_file.get_address());
}
