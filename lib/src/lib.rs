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

// UNO card deck mapping - matches your JavaScript PACK_OF_CARDS exactly
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

/// Convert index to card string (matches JavaScript side)
pub fn index_to_card(index: u8) -> &'static str {
    PACK_OF_CARDS[index as usize]
}

/// Deterministic Fisher–Yates shuffle - compatible with JavaScript expectations
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

/// Convert card indexes to JavaScript-compatible format
/// This function maps our u8 indexes to the exact string format your JavaScript expects
pub fn convert_indexes_to_js_cards(card_indexes: &[u8]) -> Vec<String> {
    card_indexes
        .iter()
        .map(|&index| PACK_OF_CARDS[index as usize].to_string())
        .collect()
}

/// Convert distributed card indexes to JavaScript format
/// Returns a nested structure matching your JavaScript game expectations
pub fn convert_distributed_cards_to_js(distributed_cards: &[Vec<u8>]) -> Vec<Vec<String>> {
    distributed_cards
        .iter()
        .map(|player_cards| convert_indexes_to_js_cards(player_cards))
        .collect()
}

// ==== COLLEAGUE'S ADVANCED UNO IMPLEMENTATION ====
// (Keeping this for future use, but using simpler approach for JavaScript compatibility)

use std::fmt;

/// Represents the four colors in an UNO deck
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnoColor {
    Red,
    Yellow,
    Green,
    Blue,
}

/// Represents the different types of cards in an UNO deck
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnoCardType {
    Number(u8), // 0-9
    Skip,
    Reverse,
    DrawTwo,
    Wild,
    WildDrawFour,
}

/// Represents a single UNO card
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnoCard {
    pub color: Option<UnoColor>,
    pub card_type: UnoCardType,
}

impl fmt::Display for UnoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.color {
            Some(color) => write!(f, "{:?} {:?}", color, self.card_type),
            None => write!(f, "{:?}", self.card_type),
        }
    }
}

/// Custom error types for shuffle and distribution operations
#[derive(Debug, PartialEq)]
pub enum ShuffleError {
    InvalidPlayerCount(usize),
    InvalidCardsPerPlayer(usize),
    InsufficientCards { requested: usize, available: usize },
}

impl fmt::Display for ShuffleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShuffleError::InvalidPlayerCount(count) => {
                write!(
                    f,
                    "Invalid player count: {}. Must be between 1 and 10",
                    count
                )
            }
            ShuffleError::InvalidCardsPerPlayer(count) => {
                write!(
                    f,
                    "Invalid cards per player: {}. Must be between 1 and 20",
                    count
                )
            }
            ShuffleError::InsufficientCards {
                requested,
                available,
            } => {
                write!(
                    f,
                    "Not enough cards: requested {}, available {}",
                    requested, available
                )
            }
        }
    }
}

impl std::error::Error for ShuffleError {}

/// Result type for shuffle operations
pub type ShuffleResult<T> = Result<T, ShuffleError>;

/// Represents the result of shuffling and distributing cards
#[derive(Debug)]
pub struct GameSetup {
    /// Each player's hand of cards
    pub hands: Vec<Vec<UnoCard>>,
    /// Remaining cards for the draw pile
    pub draw_pile: Vec<UnoCard>,
}

/// Main struct for handling card shuffling and distribution
pub struct CardShuffler;

impl CardShuffler {
    /// Creates a standard UNO deck with 108 cards
    ///
    /// # UNO Deck Composition:
    /// - Number cards (0-9): 76 cards total
    ///   - One 0 card per color (4 cards)
    ///   - Two of each number 1-9 per color (72 cards)
    /// - Action cards: 24 cards total
    ///   - Skip: 2 per color (8 cards)
    ///   - Reverse: 2 per color (8 cards)
    ///   - Draw Two: 2 per color (8 cards)
    /// - Wild cards: 8 cards total
    ///   - Wild: 4 cards
    ///   - Wild Draw Four: 4 cards
    pub fn create_ordered_uno_deck() -> Vec<UnoCard> {
        let mut deck = Vec::with_capacity(108);
        let colors = [
            UnoColor::Red,
            UnoColor::Yellow,
            UnoColor::Green,
            UnoColor::Blue,
        ];

        // Add number cards
        for &color in &colors {
            // One 0 card per color
            deck.push(UnoCard {
                color: Some(color),
                card_type: UnoCardType::Number(0),
            });

            // Two of each number 1-9 per color
            for number in 1..=9 {
                for _ in 0..2 {
                    deck.push(UnoCard {
                        color: Some(color),
                        card_type: UnoCardType::Number(number),
                    });
                }
            }

            // Two of each action card per color
            for _ in 0..2 {
                deck.push(UnoCard {
                    color: Some(color),
                    card_type: UnoCardType::Skip,
                });
                deck.push(UnoCard {
                    color: Some(color),
                    card_type: UnoCardType::Reverse,
                });
                deck.push(UnoCard {
                    color: Some(color),
                    card_type: UnoCardType::DrawTwo,
                });
            }
        }

        // Add wild cards (no color)
        for _ in 0..4 {
            deck.push(UnoCard {
                color: None,
                card_type: UnoCardType::Wild,
            });
            deck.push(UnoCard {
                color: None,
                card_type: UnoCardType::WildDrawFour,
            });
        }

        deck
    }

    /// Validates input parameters for the shuffle and deal operation
    ///
    /// # Arguments
    /// * `players` - Number of players (1-10)
    /// * `cards_per_player` - Cards to deal to each player (1-20)
    /// * `seed` - Random seed string (cannot be empty)
    /// * `deck_size` - Total number of cards available
    ///
    /// # Returns
    /// `Ok(())` if all parameters are valid, otherwise a `ShuffleError`
    fn validate_parameters(
        players: usize,
        cards_per_player: usize,
        deck_size: usize,
    ) -> ShuffleResult<()> {
        // Validate player count (reasonable limits for UNO)
        if players == 0 || players > 10 {
            return Err(ShuffleError::InvalidPlayerCount(players));
        }

        // Validate cards per player (reasonable limits for UNO)
        if cards_per_player == 0 || cards_per_player > 20 {
            return Err(ShuffleError::InvalidCardsPerPlayer(cards_per_player));
        }

        // Check if we have enough cards
        let total_needed = players * cards_per_player;
        if total_needed > deck_size {
            return Err(ShuffleError::InsufficientCards {
                requested: total_needed,
                available: deck_size,
            });
        }

        Ok(())
    }

    /// Shuffles a deck using the Fisher-Yates algorithm with cryptographic randomness
    ///
    /// # Arguments
    /// * `deck` - Mutable reference to the deck to shuffle
    /// * `seed` - seed for deterministic randomization
    ///
    /// # Returns
    /// `Ok(())` on success, `ShuffleError` on failure
    ///
    /// # Algorithm Details
    /// Uses the Fisher-Yates shuffle algorithm
    fn shuffle_deck(deck: &mut [UnoCard], seed: usize) -> ShuffleResult<()> {
        let mut state = seed as u64;
        for i in (1..deck.len()).rev() {
            // simple LCG PRNG
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let j = (state % (i as u64 + 1)) as usize;
            deck.swap(i, j);
        }
        Ok(())
    }

    /// Main function to shuffle and distribute cards to players
    ///
    /// # Arguments
    /// * `players` - Number of players (1-10)
    /// * `cards_per_player` - Number of cards to deal to each player (1-20)
    /// * `seed` - seed for deterministic randomization
    ///
    /// # Returns
    /// `Ok(GameSetup)` containing player hands and draw pile, or `ShuffleError` on failure
    ///
    /// # Example
    /// ```rust
    /// let result = CardShuffler::shuffle_and_deal(4, 7, 123)?;
    /// println!("Player 1 has {} cards", result.hands[0].len());
    /// println!("Draw pile has {} cards", result.draw_pile.len());
    /// ```
    pub fn shuffle_and_deal(
        players: usize,
        cards_per_player: usize,
        seed: usize,
    ) -> ShuffleResult<GameSetup> {
        // Initialize deck
        let mut deck = Self::create_ordered_uno_deck();
        let deck_size = deck.len();

        // Validate all input parameters
        Self::validate_parameters(players, cards_per_player, deck_size)?;

        // Shuffle the deck
        Self::shuffle_deck(&mut deck, seed)?;

        // Distribute cards to players
        let mut hands = Vec::with_capacity(players);
        let total_dealt_cards = players * cards_per_player;

        for player_index in 0..players {
            let start_index = player_index * cards_per_player;
            let end_index = start_index + cards_per_player;

            // Extract cards for this player
            let player_hand = deck[start_index..end_index].to_vec();
            hands.push(player_hand);
        }

        // Remaining cards become the draw pile
        let draw_pile = deck[total_dealt_cards..].to_vec();

        Ok(GameSetup { hands, draw_pile })
    }
}
