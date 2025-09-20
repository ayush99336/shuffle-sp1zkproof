//! A program that shuffles a deck of UNO cards and distributes them to players.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use sha2::{Digest, Sha256};
use shuffle_lib::{distribute_cards, index_to_card, shuffle, ShufflePublicValues};

pub fn main() {
    // Read inputs: seed, number of players, cards per player
    let seed = sp1_zkvm::io::read::<u64>();
    let num_players = sp1_zkvm::io::read::<u32>();
    let cards_per_player = sp1_zkvm::io::read::<u32>();

    // Construct canonical deck (108 UNO cards as indexes 0-107)
    // This matches your JavaScript PACK_OF_CARDS array exactly
    let mut deck: Vec<u8> = (0..108).collect();

    // Hash original deck
    let mut hasher = Sha256::new();
    hasher.update(&deck);
    let initial_hash = hasher.finalize().to_vec();

    // Shuffle deterministically using Fisher-Yates
    shuffle(&mut deck, seed);

    // Hash shuffled deck
    let mut hasher2 = Sha256::new();
    hasher2.update(&deck);
    let shuffled_hash = hasher2.finalize().to_vec();

    // Distribute cards to players
    let distributed_cards = distribute_cards(&deck, num_players, cards_per_player);

    // Create hashes for each player's cards
    let mut player_card_hashes = Vec::new();
    for player_cards in &distributed_cards {
        let mut hasher = Sha256::new();
        hasher.update(player_cards);
        let player_hash = hasher.finalize().to_vec();
        player_card_hashes.push(player_hash.into());
    }

    // Commit public values
    let public = ShufflePublicValues {
        initialDeckHash: initial_hash.into(),
        shuffledDeckHash: shuffled_hash.into(),
        playerCardHashes: player_card_hashes,
        seed,
        numPlayers: num_players,
        cardsPerPlayer: cards_per_player,
    };
    let bytes = ShufflePublicValues::abi_encode(&public);
    sp1_zkvm::io::commit_slice(&bytes);
}
