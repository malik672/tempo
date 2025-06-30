#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! hex = "0.4"
//! sha3 = "0.10"
//! ```

use sha3::{Digest, Keccak256};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <public_key_hex>", args[0]);
        std::process::exit(1);
    }
    
    let public_key_hex = &args[1];
    let public_key_bytes = hex::decode(public_key_hex)
        .expect("Invalid hex string");
    
    // Compute Keccak256 hash
    let mut hasher = Keccak256::new();
    hasher.update(&public_key_bytes);
    let hash = hasher.finalize();
    
    // Take first 20 bytes as address
    let address = &hash[..20];
    
    // Print address as uppercase hex
    println!("{}", hex::encode_upper(address));
}