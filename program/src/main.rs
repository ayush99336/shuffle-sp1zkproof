//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use sha2::{Digest, Sha256};
use shuffle_lib::{shuffle, ShufflePublicValues};

pub fn main() {
    // Read seed
    let seed = sp1_zkvm::io::read::<u64>();

    // Construct canonical deck
    let mut deck: Vec<u8> = (0..108).collect(); // 108 UNO cards, or match your PACK_OF_CARDS

    // Hash original deck
    let mut hasher = Sha256::new();
    hasher.update(&deck);
    let initial_hash = hasher.finalize().to_vec();

    // Shuffle deterministically
    shuffle(&mut deck, seed);

    // Hash shuffled deck
    let mut hasher2 = Sha256::new();
    hasher2.update(&deck);
    let shuffled_hash = hasher2.finalize().to_vec();

    // Commit public values
    let public = ShufflePublicValues {
        initialDeckHash: initial_hash.into(),
        shuffledDeckHash: shuffled_hash.into(),
        seed,
    };
    let bytes = ShufflePublicValues::abi_encode(&public);
    sp1_zkvm::io::commit_slice(&bytes);
}
