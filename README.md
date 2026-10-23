# Mini Wallet — Rust

A simple cryptocurrency wallet implemented in Rust for educational purposes.

## Features

- Generation of a private and public key pair
- Ethereum-style address generation using Keccak-256
- Saving the private key to a local file
- Loading a wallet from a file
- Error handling during wallet loading

## Technologies

- Rust
- `secp256k1` — elliptic curve cryptography
- `sha3` — Keccak-256 hashing
- `hex` — hexadecimal encoding and decoding

## Getting Started

### Prerequisites

- Rust and Cargo

### Run

```bash
cargo run
```

## Project Status

This project is under development. Additional features, including digital signatures, are planned.

## Security Notice

This is an educational project and has not been audited for security. Do not use it to store real cryptocurrency or private keys holding real funds.
