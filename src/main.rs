mod wallet;
use wallet::Wallet;

fn main () {
    let new_wallet: Wallet = Wallet::new();

    let new_wallet_private_key = new_wallet.get_private_key();
    let new_wallet_public_key = new_wallet.get_public_key();
    let new_wallet_address = new_wallet.get_address();

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

    let wallet_from_file_private_key = wallet_from_file.get_private_key();
    let wallet_from_file_public_key = wallet_from_file.get_public_key();
    let wallet_from_file_address = wallet_from_file.get_address();

    // comparison with an error
    assert_eq!(new_wallet_private_key, wallet_from_file_private_key);
    assert_eq!(new_wallet_public_key, wallet_from_file_public_key);
    assert_eq!(new_wallet_address, wallet_from_file_address);
}
