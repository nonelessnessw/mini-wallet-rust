// account_state.rs

use std::collections::HashMap;
use secp256k1::PublicKey;
use crate::transaction::Transaction;

pub struct AccountState {
    nonce_data: HashMap<String, u32>,
    balance_data: HashMap<String, u128>
}

impl AccountState {
    pub fn new() -> AccountState {
        AccountState {
            nonce_data: HashMap::new(),
            balance_data: HashMap::new()
        }
    }

    pub fn update_nonce(&mut self, address: &str, nonce: u32) {
        self.nonce_data.insert(address.to_string(), nonce);
    }

    pub fn validate_nonce(&self, address: &str, nonce: u32) -> Result<(), String> {
        if self.get_next_nonce(address)? == nonce {
            Ok(())
        }
        else {
            Err("Incorrect nonce".to_string())
        }
    }

    pub fn get_next_nonce(&self, address: &str) -> Result<u32, String> {
        match self.nonce_data.get(address).copied() {
            None => { Ok(0) },
            Some(value) => { value.checked_add(1).ok_or_else(|| "Nonce overflow".to_string()) }
        }
    }

    pub fn get_balance(&self, address: &str) -> u128 {
        self.balance_data.get(address).copied().unwrap_or(0)
    }

    pub fn update_balance(&mut self, address: &str, amount: u128) -> Result<(), String> {
        let new_amount = self.balance_data.entry(address.to_string()).or_insert(0).checked_add(amount).ok_or_else(|| "Balance overflow".to_string())?;

        self.balance_data.entry(address.to_string()).and_modify(|x| {*x = new_amount}).or_insert(new_amount);
        Ok(())
    }

    pub fn check_balance(&self, address: &str, amount: u128) -> Result<(), String> {
        match self.get_balance(address) >= amount {
            true => { Ok(()) },
            false => { Err("Insufficient amount".to_string())}
        }
    }

    pub fn withdraw(&mut self, address: &str, amount: u128) -> Result<(), String> {
        if amount == 0 { Err("Wrong amount".to_string()) }
        else {
            self.check_balance(address, amount)?;
            *self.balance_data.entry(address.to_string()).or_insert(0) -= amount;
            Ok(())
        }
    }

    pub fn apply_transaction(&mut self, transaction: &Transaction, public_key: &PublicKey) -> Result<(), String> {
        transaction.verify_transaction(public_key)?;
        transaction.validate_nonce(self)?;
        self.check_balance(transaction.get_sender(), transaction.get_amount())?;

        if self.get_balance(transaction.get_receiver()).checked_add(transaction.get_amount()).is_none() {
            return Err("Incorrect balance".to_string())
        }

        self.withdraw(transaction.get_sender(), transaction.get_amount())?;
        self.update_balance(transaction.get_receiver(), transaction.get_amount())?;
        self.update_nonce(transaction.get_sender(), transaction.get_nonce());
        Ok(())
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;