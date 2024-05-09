use alloy_dyn_abi::DynSolValue;

mod helpers;
use helpers::{read_whitelist, write_to_output};

mod merkle;
use merkle::{generate_merkle_data, MerkleData};

fn main() {
    let whitelist_users: Vec<DynSolValue> = read_whitelist();
    let merkle: MerkleData = generate_merkle_data(whitelist_users);
    write_to_output(&merkle);
    println!("Root: {:?}", merkle.root);
}
