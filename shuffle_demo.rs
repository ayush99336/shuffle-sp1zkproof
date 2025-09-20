use shuffle_lib::{convert_distributed_cards_to_js, shuffle_and_deal, PACK_OF_CARDS};

fn main() {
    println!("🎯 SP1 Shuffle and Deal Demo");
    println!("============================");

    // Example: 2 players, 7 cards each (standard UNO)
    let seed = 12345;
    let num_players = 2;
    let cards_per_player = 7;

    println!("\n📋 Setup:");
    println!("Seed: {}", seed);
    println!("Players: {}", num_players);
    println!("Cards per player: {}", cards_per_player);

    // Use the unified shuffle_and_deal function
    let (distributed_cards, remaining_deck) = shuffle_and_deal(seed, num_players, cards_per_player);

    println!("\n🎴 Results:");
    println!(
        "Total cards dealt: {}",
        distributed_cards.iter().map(|p| p.len()).sum::<usize>()
    );
    println!("Remaining cards: {}", remaining_deck.len());

    // Convert to JavaScript-compatible format
    let js_distributed_cards = convert_distributed_cards_to_js(&distributed_cards);

    println!("\n🎮 Player Cards (JavaScript format):");
    for (i, player_cards) in js_distributed_cards.iter().enumerate() {
        println!("Player {}: {:?}", i + 1, player_cards);
    }

    println!("\n🃏 Draw Pile Preview (first 10 cards):");
    let draw_pile_js: Vec<String> = remaining_deck
        .iter()
        .take(10)
        .map(|&index| PACK_OF_CARDS[index as usize].to_string())
        .collect();
    println!("{:?}...", draw_pile_js);

    println!("\n✅ Ready for JavaScript integration!");
}
