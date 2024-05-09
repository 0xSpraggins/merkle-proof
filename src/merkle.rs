use crate::helpers::trim_string;
use alloy_dyn_abi::DynSolValue;
use alloy_merkle_tree::standard_binary_tree::StandardMerkleTree;
use alloy_primitives::FixedBytes;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserProofs {
    address: String,
    proof: Vec<FixedBytes<32>>,
    validation: bool,
}

#[derive(Serialize)]
pub struct MerkleData {
    pub root: FixedBytes<32>,
    proofs: Vec<UserProofs>,
}

pub fn generate_merkle_data(whitelist: Vec<DynSolValue>) -> MerkleData {
    let tree: StandardMerkleTree = StandardMerkleTree::of(whitelist.clone());
    let root: FixedBytes<32> = tree.root();

    let proofs: Vec<UserProofs> = whitelist
        .into_iter()
        .map(|x| {
            let dyn_sol_str: &str = x.as_str().unwrap();
            let address: String = trim_string(dyn_sol_str).to_string();
            let proof: Vec<FixedBytes<32>> = tree.get_proof(&x);
            UserProofs {
                address,
                proof: proof.clone(),
                validation: tree.verify_proof(x, proof),
            }
        })
        .collect();

    MerkleData { proofs, root }
}
