use alloy_sol_types::sol;

sol! {
    struct ShufflePublicValues {
        // Hash of the input deck (to commit to canonical starting state)
        bytes initialDeckHash;
        // Hash of the final shuffled deck (to prove correctness)
        bytes shuffledDeckHash;
        // Array of hashes for each player's cards
        bytes[] playerCardHashes;
        // The seed used for randomness
        uint64 seed;
        // Number of players in the game
        uint32 numPlayers;
        // Number of cards dealt to each player
        uint32 cardsPerPlayer;
    }
}

// UNO card deck with 108 cards
pub const PACK_OF_CARDS: [&str; 108] = [
    "0R", "1R", "1R", "2R", "2R", "3R", "3R", "4R", "4R", "5R", "5R", "6R", "6R", "7R", "7R", "8R",
    "8R", "9R", "9R", "skipR", "skipR", "_R", "_R", "D2R", "D2R", "0G", "1G", "1G", "2G", "2G",
    "3G", "3G", "4G", "4G", "5G", "5G", "6G", "6G", "7G", "7G", "8G", "8G", "9G", "9G", "skipG",
    "skipG", "_G", "_G", "D2G", "D2G", "0B", "1B", "1B", "2B", "2B", "3B", "3B", "4B", "4B", "5B",
    "5B", "6B", "6B", "7B", "7B", "8B", "8B", "9B", "9B", "skipB", "skipB", "_B", "_B", "D2B",
    "D2B", "0Y", "1Y", "1Y", "2Y", "2Y", "3Y", "3Y", "4Y", "4Y", "5Y", "5Y", "6Y", "6Y", "7Y",
    "7Y", "8Y", "8Y", "9Y", "9Y", "skipY", "skipY", "_Y", "_Y", "D2Y", "D2Y", "W", "W", "W", "W",
    "D4W", "D4W", "D4W", "D4W",
];

/// Convert index to card string (for debugging/display)
pub fn index_to_card(index: u8) -> &'static str {
    PACK_OF_CARDS[index as usize]
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

/// Distribute cards to players from the shuffled deck
pub fn distribute_cards(deck: &[u8], num_players: u32, cards_per_player: u32) -> Vec<Vec<u8>> {
    let mut players: Vec<Vec<u8>> = vec![Vec::new(); num_players as usize];

    for i in 0..(num_players * cards_per_player) {
        let player_index = (i % num_players) as usize;
        let card_index = i as usize;

        if card_index < deck.len() {
            players[player_index].push(deck[card_index]);
        }
    }

    players
}
