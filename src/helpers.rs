use crate::merkle::MerkleData;
use alloy_dyn_abi::DynSolValue;
use serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn write_to_output(data: &MerkleData) {
    let serialized_data: String =
        serde_json::to_string_pretty(data).expect("Data serialization failed");
    let mut file: File = File::create("./output/merkleData.json").expect("File creation failed");
    file.write_all(serialized_data.as_bytes())
        .expect("Exporting data failed");
}

pub fn read_whitelist() -> Vec<DynSolValue> {
    let file: File = File::open("whitelist.txt").expect("Whitelist not found");
    let reader: BufReader<File> = BufReader::new(file);

    return reader
        .lines()
        .filter_map(Result::ok)
        .map(|x: String| DynSolValue::from(x))
        .collect();
}

// Convesion from DynSolValue to string creates weird formatting. In order to avoid this we should clean up the string
// before storing in our output file
pub fn trim_dyn_sol_string(x: &str) -> &str {
    &x[3..45]
}
