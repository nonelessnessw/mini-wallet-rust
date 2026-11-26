use sha3::{Digest, Keccak256};
use std::time::{SystemTime, UNIX_EPOCH};
use secp256k1::{ecdsa, Message, PublicKey};
use crate::wallet::Wallet;
use crate::account_state::AccountState;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: u128,
    message: String,
    timestamp: u128,
    nonce: u32,
    signature: Option<ecdsa::Signature>
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u128, message: String, nonce: u32) -> Result<Transaction, String> {
        let timestamp = match Self::get_current_time() {
            Ok(value) => { value },
            Err(error) => {return Err(error)}
        };

        if Self::check_address_validity(&sender) && Self::check_address_validity(&receiver) && sender != receiver && amount > 0 && timestamp > 0 {
            Ok(Transaction {
                sender,
                receiver,
                amount,
                message,
                timestamp,
                nonce,
                signature: None
            })
        }
        else {
            Err("Addresses must contain only hex digits, not be empty or equal, both amount and timestamp must be more than 0".to_string())
        }


    }

    pub fn get_current_time() -> Result<u128, String> {
        let unix_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(value) => { value },
            Err(_) => { return Err("Wrong time epoch".to_string()) }
        };

        Ok(unix_time.as_millis())
    }

    pub fn check_address_validity(address: &str) -> bool {
        address.starts_with("0x") && address.len() == 42
            && address[2..].chars().all(|x| x.is_ascii_hexdigit())
    }

    pub fn serialize(&self) -> Vec<u8> {
        let sender = self.sender.as_bytes();
        let receiver = self.receiver.as_bytes();
        let amount = self.amount.to_string();
        let amount = amount.as_bytes();
        let message = self.message.as_bytes();
        let timestamp = self.timestamp.to_string();
        let timestamp = timestamp.as_bytes();
        let nonce = self.nonce.to_string();
        let nonce = nonce.as_bytes();

        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&(sender.len() as u32).to_le_bytes());
        vec.extend_from_slice(sender);
        vec.extend_from_slice(&(receiver.len() as u32).to_le_bytes());
        vec.extend_from_slice(receiver);
        vec.extend_from_slice(&(amount.len() as u32).to_le_bytes());
        vec.extend_from_slice(amount);
        vec.extend_from_slice(&(message.len() as u32).to_le_bytes());
        vec.extend_from_slice(message);
        vec.extend_from_slice(&(timestamp.len() as u32).to_le_bytes());
        vec.extend_from_slice(timestamp);
        vec.extend_from_slice(&(nonce.len() as u32).to_le_bytes());
        vec.extend_from_slice(nonce);

        vec
    }

    pub fn hash_transaction(&self) -> [u8; 32] {
        let data_serialized = self.serialize();

        let mut hasher = Keccak256::new();
        hasher.update(&data_serialized);
        let hash: [u8; 32] = hasher.finalize().into();

        hash
    }

    pub fn sign_transaction(&mut self, wallet: &Wallet) -> Result<(), String>{
        if self.sender != wallet.get_address() {
            return Err("Wrong sender address".to_string())
        }

        let hash = self.hash_transaction();
        let message = Message::from_digest(hash);
        let transaction_signature = ecdsa::sign(message, wallet.get_private_key_ref());

        self.signature = Some(transaction_signature);

        Ok(())
    }

    pub fn verify_transaction(&self, public_key: &PublicKey) -> Result<(), String> {
        let transaction_signature = match self.signature.as_ref() {
            None => { return Err("Transaction was not signed".to_string()) },
            Some(value) => { value }
        };
        let transaction_hash = self.hash_transaction();
        let message = Message::from_digest(transaction_hash);
        let serialized_public_key = public_key.serialize_uncompressed();

        let mut hasher = Keccak256::new();
        hasher.update(&serialized_public_key[1..]);
        let keccak_result = hasher.finalize();

        let address_bytes = &keccak_result[12..];

        let address = format!("0x{}", hex::encode(address_bytes));

        if self.sender != address {
            Err("Incorrect sender".to_string())
        }
        else {
            match ecdsa::verify(&transaction_signature, message, public_key) {
                Ok(_) => { Ok(()) },
                Err(_) => { Err("Incorrect signature".to_string()) }
            }
        }
    }

    pub fn validate_nonce(&self, account_state: &AccountState) -> Result<(), String> {
        account_state.validate_nonce(&self.sender, self.nonce)
    }

    pub fn get_receiver(&self) -> &str {
        &self.receiver
    }

    pub fn get_amount(&self) -> u128 {
        self.amount
    }

    pub fn get_sender(&self) -> &str {
        &self.sender
    }

    pub fn get_nonce(&self) -> u32 {
        self.nonce
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;