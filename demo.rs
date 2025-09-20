use shuffle_lib::{shuffle, distribute_cards, convert_distributed_cards_to_js, PACK_OF_CARDS};

fn main() {
    println!("🎮 SP1 UNO Shuffle Demo - JavaScript Compatible");
    println!("===============================================\n");

    // Simulate the same inputs as your game
    let seed = 12345u64;
    let num_players = 2u32;
    let cards_per_player = 7u32;

    // Create and shuffle deck (same as SP1 program)
    let mut deck: Vec<u8> = (0..108).collect();
    println!("📦 Original deck: [0, 1, 2, ..., 107] (108 UNO cards)");
    
    shuffle(&mut deck, seed);
    println!("🔀 Shuffled deck: {:?}", &deck[0..10]); // Show first 10 for brevity
    
    // Distribute cards
    let distributed_cards = distribute_cards(&deck, num_players, cards_per_player);
    
    // Convert to JavaScript format
    let js_compatible_cards = convert_distributed_cards_to_js(&distributed_cards);
    
    println!("\n🎯 Distributed Cards (JavaScript Compatible):");
    for (player, cards) in js_compatible_cards.iter().enumerate() {
        println!("Player {}: {:?}", player + 1, cards);
    }
    
    println!("\n🃏 Card Mapping Examples:");
    for i in 0..5 {
        println!("Index {} = \"{}\"", i, PACK_OF_CARDS[i]);
    }
    
    println!("\n✅ This output can be directly used in your JavaScript UNO game!");
    println!("💡 Use convert_distributed_cards_to_js() in your backend service.");
}