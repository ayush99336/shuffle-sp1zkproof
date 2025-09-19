use alloy_sol_types::sol;

sol! {
    struct ShufflePublicValues {
        // Hash of the input deck (to commit to canonical starting state)
        bytes initialDeckHash;
        // Hash of the final shuffled deck (to prove correctness)
        bytes shuffledDeckHash;
        // The seed used for randomness
        uint64 seed;
    }
}

/// Deterministic Fisher–Yates shuffle.
pub fn shuffle(deck: &mut [u8], seed: u64) {
    let mut state = seed;
    for i in (1..deck.len()).rev() {
        // simple LCG PRNG
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = (state % (i as u64 + 1)) as usize;
        deck.swap(i, j);
    }
}
