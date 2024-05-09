# Merkle Proof Generator | Rust

An easy to use script to generate merkle proofs for an list of elements. This script is written in Rust utilizing Alloy's Merkle Tree Crate. It is currently used in production to generate Merkle Proofs for Market Makers and Borrowers who participate in swapping USDC for ib01 tokens on Bloom Protocol.

## Usage

1. Clone the repository

```bash
git clone https://github.com/0xSpraggins/merkle-proof.git
```

2. Install Dependencies

```bash
cargo build
```

3. Update the `whitelist.txt` file with the list of elements you want to generate Merkle Proofs for. Each element should be on a new line.

4. Run the script

```bash
cargo run
```

5. The script will generate a `merkleData.json` file in the output directory of the project. This file will contain the Merkle Root and the Merkle Proof for each element in the `whitelist.txt` file.

## Dependencies

- [Alloy Merkle Tree](https://crates.io/crates/alloy-merkle-tree)
- [Alloy Primitives](https://crates.io/crates/alloy-primitives)
- [Alloy Dyn ABI](https://crates.io/crates/alloy-dyn-abi)
- [Serde](https://crates.io/crates/serde)
- [Serde JSON](https://crates.io/crates/serde_json)

## Help
For any questions or concerns, please reach out on Telegram: @baileyspraggins or twitter/x: @0xSpraggins
