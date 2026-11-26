// test.rs

use super::Transaction;
use crate::wallet::Wallet;
use crate::account_state::AccountState;

fn create_test_transaction() -> Result<Transaction, String> {
    let address1 = Wallet::new().get_address();
    let address2 = Wallet::new().get_address();

    Transaction::new(
        address1.clone(),
        address2.clone(),
        128,
        "happy birthday".to_string(),
        123
    )
}

fn get_wallet() -> Wallet {
    Wallet::new()
}

fn get_adress_and_account_state() -> (String, AccountState) {
    let address = Wallet::new().get_address();
    let account_state = AccountState::new();

    (address, account_state)
}

#[test]
fn test_same_transaction_hashes() {
    let new_transaction1 = create_test_transaction().unwrap();

    let new_transaction2 = new_transaction1.clone();

    assert_eq!(new_transaction1.hash_transaction(), new_transaction2.hash_transaction());
}

#[test]
fn test_different_transaction_hashes() {
    let new_transaction1 = create_test_transaction().unwrap();

    let mut new_transaction2 = new_transaction1.clone();

    new_transaction2.amount = 2;

    assert_ne!(new_transaction1.hash_transaction(), new_transaction2.hash_transaction())
}

#[test]
fn test_amount() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let new_transaction1 = Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        0,
        "happy birthday".to_string(),
        0
    );

    assert!(new_transaction1.is_err());
}

#[test]
fn test_invalid_address() {
    let wallet1 = Wallet::new();

    let new_transaction1 = Transaction::new(
        "1x1234567890".to_string(),
        wallet1.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    );

    assert!(new_transaction1.is_err());

    let wallet2 = Wallet::new();

    let new_transaction2 = Transaction::new(
        wallet2.get_address().clone(),
        "qwertyzxcvbn".to_string(),
        1,
        "happy birthday".to_string(),
        0
    );

    assert!(new_transaction2.is_err());

}

#[test]
fn test_same_address() {
    let wallet1 = Wallet::new();

    let new_transaction1 = Transaction::new(
        wallet1.get_address().clone(),
        wallet1.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    );

    assert!(new_transaction1.is_err())
}

#[test]
fn test_correct_timestamp() {
    let new_transaction1 = create_test_transaction().unwrap();

    assert_ne!(new_transaction1.timestamp, 0);
}

#[test]
fn test_correct_signature() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let mut new_transaction1 = Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(new_transaction1.sign_transaction(&wallet1).is_ok());
    assert!(new_transaction1.verify_transaction(wallet1.get_public_key_ref()).is_ok());
}

#[test]
fn test_wrong_wallet_cannot_sign() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let mut new_transaction1 = Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(new_transaction1.sign_transaction(&wallet2).is_err());
}

#[test]
fn test_modified_transaction() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let mut new_transaction1 = Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(new_transaction1.sign_transaction(&wallet1).is_ok());
    assert!(new_transaction1.verify_transaction(wallet1.get_public_key_ref()).is_ok());

    new_transaction1.amount = 2;

    assert!(new_transaction1.verify_transaction(wallet1.get_public_key_ref()).is_err());
}

#[test]
fn test_wrong_wallet_cannot_verify() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let mut new_transaction1 = Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(new_transaction1.sign_transaction(&wallet1).is_ok());
    assert!(new_transaction1.verify_transaction(wallet2.get_public_key_ref()).is_err());
}

#[test]
fn test_same_data() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let mut new_transaction1 = match Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    ) {
        Ok(value) => { value },
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    new_transaction1.timestamp = 1;

    let mut new_transaction2 = match Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    ) {
        Ok(value) => { value },
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    new_transaction2.timestamp = 1;

    assert_eq!(new_transaction1, new_transaction2);
}

#[test]
fn test_changed_data() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let new_transaction = match Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    ) {
        Ok(value) => { value },
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    let mut new_transaction1 = new_transaction.clone();

    new_transaction1.sender = "0x123".to_string();

    assert_ne!(new_transaction, new_transaction1);

    new_transaction1.sender = new_transaction.sender.clone();
    new_transaction1.receiver = "0x123".to_string();

    assert_ne!(new_transaction, new_transaction1);

    new_transaction1.receiver = new_transaction.receiver.clone();
    new_transaction1.message = "happy halloween".to_string();

    assert_ne!(new_transaction, new_transaction1);

    new_transaction1.message = new_transaction.message.clone();
    new_transaction1.nonce = 1;

    assert_ne!(new_transaction, new_transaction1);

    new_transaction1.nonce = new_transaction.nonce.clone();
    new_transaction1.amount = 2;

    assert_ne!(new_transaction, new_transaction1);

    new_transaction1.amount = new_transaction.amount;
    new_transaction1.timestamp = 1;

    assert_ne!(new_transaction, new_transaction1);
}

#[test]
fn unsigned_transaction_cannot_verify() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let new_transaction1 = Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(new_transaction1.verify_transaction(wallet1.get_public_key_ref()).is_err());
}

#[test]
fn test_resigned_signature() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let mut new_transaction1 = Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    ).unwrap();

    new_transaction1.sign_transaction(&wallet1).unwrap();

    let signature = new_transaction1.signature.clone();

    new_transaction1.sign_transaction(&wallet1).unwrap();

    assert_eq!(signature, new_transaction1.signature);
}

#[test]
fn test_resigned_signature_with_changes() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let mut new_transaction1 = Transaction::new(
        wallet1.get_address().clone(),
        wallet2.get_address().clone(),
        1,
        "happy birthday".to_string(),
        0
    ).unwrap();

    new_transaction1.sign_transaction(&wallet1).unwrap();

    let signature = new_transaction1.signature.clone();

    new_transaction1.amount = 2;
    new_transaction1.sign_transaction(&wallet1).unwrap();

    assert_ne!(signature, new_transaction1.signature);
}

#[test]
fn test_changed_transaction_data() {
    let (_, mut account_state) = get_adress_and_account_state();
    let wallet1 = get_wallet();
    let public_key = wallet1.get_public_key_ref();
    let wallet2 = get_wallet();
    let address2 = wallet2.get_address();

    account_state.update_balance(&wallet1.get_address(), 100);

    let mut transaction = Transaction::new(
        wallet1.get_address().clone(),
        address2.clone(),
        99,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(transaction.sign_transaction(&wallet1).is_ok());

    transaction.amount = 5;

    assert!(account_state.apply_transaction(&transaction, public_key).is_err());

    assert_eq!(account_state.get_balance(&address2), 0);
    assert_eq!(account_state.get_balance(&wallet1.get_address()), 100);
    assert_eq!(account_state.get_next_nonce(&wallet1.get_address()), Ok(0));
}

