use axum::http::StatusCode;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use shuffle_lib::{shuffle_and_deal, PACK_OF_CARDS};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize)]
struct ShuffleDealRequest {
    seed: u64,
    players: u32,
    cards_per_player: u32,
    // Return card strings instead of indexes
    as_js_cards: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PublicValues {
    initial_deck_hash: String,
    shuffled_deck_hash: String,
    seed: u64,
    num_players: u32,
    cards_per_player: u32,
    // SHA-256 hash (hex) of each player's dealt hand, matching zkVM script output
    player_card_hashes: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ShuffleDealResponse {
    public_values: PublicValues,
    // Dealt cards, either as indexes (u8) or JS strings, decided at runtime
    distributed_cards: serde_json::Value,
    draw_pile: serde_json::Value,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/shuffle-deal", post(shuffle_deal))
        .layer(cors);

    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3001".to_string());
    tracing::info!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn shuffle_deal(
    Json(req): Json<ShuffleDealRequest>,
) -> Result<Json<ShuffleDealResponse>, (StatusCode, String)> {
    // Build canonical ordered deck indexes [0..108)
    let canonical_deck: Vec<u8> = (0..108).collect();

    // Hash initial deck
    let mut hasher = Sha256::new();
    hasher.update(&canonical_deck);
    let initial_hash = hasher.finalize();

    // Shuffle and deal
    let (distributed, draw_pile) = shuffle_and_deal(req.seed, req.players, req.cards_per_player);

    // Compute per-player SHA-256 over each hand (as raw byte sequence of indexes)
    let player_hashes: Vec<String> = distributed
        .iter()
        .map(|hand| {
            let mut h = Sha256::new();
            h.update(hand);
            format!("0x{}", hex::encode(h.finalize()))
        })
        .collect();

    // Construct the full shuffled deck (dealt first by players order then draw pile)
    let mut shuffled_full: Vec<u8> = Vec::with_capacity(108);
    for hand in &distributed {
        shuffled_full.extend(hand);
    }
    shuffled_full.extend(&draw_pile);

    // Hash shuffled deck
    let mut hasher2 = Sha256::new();
    hasher2.update(&shuffled_full);
    let shuffled_hash = hasher2.finalize();

    let as_js = req.as_js_cards.unwrap_or(false);

    let distributed_cards_value = if as_js {
        let dist_js: Vec<Vec<String>> = distributed
            .iter()
            .map(|hand| {
                hand.iter()
                    .map(|&idx| PACK_OF_CARDS[idx as usize].to_string())
                    .collect()
            })
            .collect();
        serde_json::to_value(dist_js).unwrap()
    } else {
        serde_json::to_value(&distributed).unwrap()
    };

    let draw_pile_value = if as_js {
        let draw_js: Vec<String> = draw_pile
            .iter()
            .map(|&idx| PACK_OF_CARDS[idx as usize].to_string())
            .collect();
        serde_json::to_value(draw_js).unwrap()
    } else {
        serde_json::to_value(&draw_pile).unwrap()
    };

    let resp = ShuffleDealResponse {
        public_values: PublicValues {
            initial_deck_hash: format!("0x{}", hex::encode(initial_hash)),
            shuffled_deck_hash: format!("0x{}", hex::encode(shuffled_hash)),
            seed: req.seed,
            num_players: req.players,
            cards_per_player: req.cards_per_player,
            player_card_hashes: player_hashes,
        },
        distributed_cards: distributed_cards_value,
        draw_pile: draw_pile_value,
    };

    Ok(Json(resp))
}
