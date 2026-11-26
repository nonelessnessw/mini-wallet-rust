// account_state.rs

use super::AccountState;
use crate::wallet::Wallet;
use crate::transaction::Transaction;

fn get_adress_and_account_state() -> (String, AccountState) {
    let address = Wallet::new().get_address();
    let account_state = AccountState::new();

    (address, account_state)
}

fn get_wallet() -> Wallet {
    Wallet::new()
}

#[test]
fn test_first_nonce() {
    let (address, account_state) = get_adress_and_account_state();

    assert!(account_state.validate_nonce(&address, 0).is_ok());
}

#[test]
fn test_same_nonce() {
    let (address, mut account_state) = get_adress_and_account_state();

    assert!(account_state.validate_nonce(&address, 0).is_ok());

    account_state.update_nonce(&address, 0);

    assert!(account_state.validate_nonce(&address, 0).is_err())
}

#[test]
fn test_next_nonce() {
    let (address, mut account_state) = get_adress_and_account_state();

    assert!(account_state.validate_nonce(&address, 0).is_ok());

    account_state.update_nonce(&address, 0);

    assert!(account_state.validate_nonce(&address, 1).is_ok());
}

#[test]
fn test_skipped_nonce() {
    let (address, mut account_state) = get_adress_and_account_state();

    assert!(account_state.validate_nonce(&address, 0).is_ok());

    account_state.update_nonce(&address, 0);

    assert!(account_state.validate_nonce(&address, 2).is_err());
}

#[test]
fn test_different_addresses() {
    let (address1, mut account_state) = get_adress_and_account_state();
    let address2 = Wallet::new().get_address();

    account_state.update_nonce(&address1, 0);

    assert!(account_state.validate_nonce(&address1, 0).is_err());
    assert!(account_state.validate_nonce(&address2, 0).is_ok());
}

#[test]
fn test_initial_address() {
    let (address1, account_state) = get_adress_and_account_state();

    assert_eq!(account_state.get_balance(&address1), 0);
}

#[test]
fn test_update_balance() {
    let (address1, mut account_state) = get_adress_and_account_state();

    account_state.update_balance(&address1, 100);

    assert_eq!(account_state.get_balance(&address1), 100);
}

#[test]
fn test_multiple_balance_updates() {
    let (address1, mut account_state) = get_adress_and_account_state();

    account_state.update_balance(&address1, 100);
    account_state.update_balance(&address1, 1000);

    assert_eq!(account_state.get_balance(&address1), 1100);
}

#[test]
fn test_sufficient_balance() {
    let (address1, mut account_state) = get_adress_and_account_state();

    account_state.update_balance(&address1, 100);

    assert!(account_state.check_balance(&address1, 100).is_ok());
}

#[test]
fn test_insufficient_balance() {
    let (address1, mut account_state) = get_adress_and_account_state();

    assert!(account_state.check_balance(&address1, 100).is_err());

    account_state.update_balance(&address1, 100);

    assert!(account_state.check_balance(&address1, 101).is_err());
}

#[test]
fn test_successful_withdraw() {
    let (address1, mut account_state) = get_adress_and_account_state();

    account_state.update_balance(&address1, 100);

    assert!(account_state.withdraw(&address1, 100).is_ok());
    assert_eq!(account_state.get_balance(&address1), 0);
}

#[test]
fn test_unsuccessful_withdraw() {
    let (address1, mut account_state) = get_adress_and_account_state();

    account_state.update_balance(&address1, 100);

    assert!(account_state.withdraw(&address1, 101).is_err());
    assert_eq!(account_state.get_balance(&address1), 100);
}

#[test]
fn test_new_address_nonce() {
    let (address1, account_state) = get_adress_and_account_state();

    assert_eq!(account_state.get_next_nonce(&address1), Ok(0));
}

#[test]
fn test_saved_nonce() {
    let (address1, mut account_state) = get_adress_and_account_state();

    account_state.update_nonce(&address1, 5);

    assert_eq!(account_state.get_next_nonce(&address1), Ok(6));
}

#[test]
fn test_nonce_overflow() {
    let (address1, mut account_state) = get_adress_and_account_state();

    account_state.update_nonce(&address1, u32::MAX);

    assert!(account_state.get_next_nonce(&address1).is_err());

}

#[test]
fn test_successful_transaction() {
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

    assert!(account_state.apply_transaction(&transaction, public_key).is_ok());

    assert_eq!(account_state.get_balance(&address2), 99);
    assert_eq!(account_state.get_balance(&wallet1.get_address()), 1);
    assert_eq!(account_state.get_next_nonce(&wallet1.get_address()), Ok(1));
}

#[test]
fn test_unsuccessful_transaction() {
    let (_, mut account_state) = get_adress_and_account_state();
    let wallet1 = get_wallet();
    let public_key = wallet1.get_public_key_ref();
    let wallet2 = get_wallet();
    let address2 = wallet2.get_address();

    account_state.update_balance(&wallet1.get_address(), 0);

    let mut transaction = Transaction::new(
        wallet1.get_address().clone(),
        address2.clone(),
        99,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(transaction.sign_transaction(&wallet1).is_ok());

    assert!(account_state.apply_transaction(&transaction, public_key).is_err());

    assert_eq!(account_state.get_balance(&address2), 0);
    assert_eq!(account_state.get_balance(&wallet1.get_address()), 0);
    assert_eq!(account_state.get_next_nonce(&wallet1.get_address()), Ok(0));
}

#[test]
fn test_transaction_with_missed_nonce() {
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
        1
    ).unwrap();

    assert!(transaction.sign_transaction(&wallet1).is_ok());

    assert!(account_state.apply_transaction(&transaction, public_key).is_err());

    assert_eq!(account_state.get_balance(&address2), 0);
    assert_eq!(account_state.get_balance(&wallet1.get_address()), 100);
    assert_eq!(account_state.get_next_nonce(&wallet1.get_address()), Ok(0));
}

#[test]
fn test_replay_transaction() {
    let (_, mut account_state) = get_adress_and_account_state();
    let wallet1 = get_wallet();
    let address1 = wallet1.get_address();
    let public_key = wallet1.get_public_key_ref();
    let wallet2 = get_wallet();
    let address2 = wallet2.get_address();

    account_state.update_balance(&address1, 200);

    let mut transaction = Transaction::new(
        address1.clone(),
        address2.clone(),
        50,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(transaction.sign_transaction(&wallet1).is_ok());

    assert!(account_state.apply_transaction(&transaction, public_key).is_ok());

    assert!(account_state.apply_transaction(&transaction, public_key).is_err());

    assert_eq!(account_state.get_balance(&address1), 150);
    assert_eq!(account_state.get_balance(&address2), 50);
    assert_eq!(account_state.get_next_nonce(&address1), Ok(1));
}

#[test]
fn test_reused_nonce_with_new_signature() {
    let (_, mut account_state) = get_adress_and_account_state();
    let wallet1 = get_wallet();
    let address1 = wallet1.get_address();
    let public_key = wallet1.get_public_key_ref();
    let wallet2 = get_wallet();
    let address2 = wallet2.get_address();

    account_state.update_balance(&address1, 200);

    let mut transaction = Transaction::new(
        address1.clone(),
        address2.clone(),
        50,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(transaction.sign_transaction(&wallet1).is_ok());

    assert!(account_state.apply_transaction(&transaction, public_key).is_ok());

    let mut transaction = Transaction::new(
        address1.clone(),
        address2.clone(),
        50,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(transaction.sign_transaction(&wallet1).is_ok());

    assert!(account_state.apply_transaction(&transaction, public_key).is_err());

    assert_eq!(account_state.get_balance(&address1), 150);
    assert_eq!(account_state.get_balance(&address2), 50);
    assert_eq!(account_state.get_next_nonce(&address1), Ok(1));
}

#[test]
fn test_balance_overflow() {
    let (_, mut account_state) = get_adress_and_account_state();
    let wallet1 = get_wallet();
    let address1 = wallet1.get_address();
    let public_key = wallet1.get_public_key_ref();
    let wallet2 = get_wallet();
    let address2 = wallet2.get_address();

    account_state.update_balance(&address1, 100);
    account_state.update_balance(&address2, u128::MAX);

    let mut transaction = Transaction::new(
        address1.clone(),
        address2.clone(),
        50,
        "happy birthday".to_string(),
        0
    ).unwrap();

    assert!(transaction.sign_transaction(&wallet1).is_ok());

    assert!(account_state.apply_transaction(&transaction, public_key).is_err());

    assert_eq!(account_state.get_balance(&address1), 100);
    assert_eq!(account_state.get_balance(&address2), u128::MAX);
    assert_eq!(account_state.get_next_nonce(&address1), Ok(0));
}

#[test]
fn test_invalid_signature_does_not_change_state() {
    
}