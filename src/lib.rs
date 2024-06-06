#![no_std]

use gstd::{exec, msg};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[no_mangle]
extern "C" fn init() {
    // Load PebblesInit message
    let init: PebblesInit = msg::load().expect("Initialization data not loaded");

    // Determine first player
    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    // Initialize GameState
    let state = GameState {
        pebbles_count: init.pebbles_count,
        max_pebbles_per_turn: init.max_pebbles_per_turn,
        pebbles_remaining: init.pebbles_count,
        difficulty: init.difficulty,
        first_player,
        winner: None,
    };

    unsafe {
        PEBBLES_GAME = Some(state);
    }

    // Respond indicating successful initialization
    msg::reply_bytes("Initialized", 0).expect("Failed to send initialization response");
}

#[no_mangle]
extern "C" fn handle() {
    // Load PebblesAction message
    let action: PebblesAction = msg::load().expect("Action not loaded");

    let result: PebblesEvent;

    unsafe {
        let game = PEBBLES_GAME.as_mut().expect("Game state not loaded");

        result = match action {
            PebblesAction::Turn(pebbles_taken) => {
                if pebbles_taken >= 1
                    && pebbles_taken <= game.max_pebbles_per_turn
                    && pebbles_taken <= game.pebbles_remaining
                {
                    game.pebbles_remaining -= pebbles_taken;

                    if game.pebbles_remaining == 0 {
                        game.winner = Some(game.first_player.clone());
                        PebblesEvent::Won(game.first_player.clone())
                    } else {
                        game.first_player = match game.first_player {
                            Player::User => Player::Program,
                            Player::Program => Player::User,
                        };
                        PebblesEvent::CounterTurn(game.pebbles_remaining)
                    }
                } else {
                    PebblesEvent::CounterTurn(game.pebbles_remaining) // Event to handle invalid turn if needed
                }
            }
            PebblesAction::GiveUp => {
                game.winner = match game.first_player {
                    Player::User => Some(Player::Program),
                    Player::Program => Some(Player::User),
                };
                PebblesEvent::Won(game.winner.clone().unwrap())
            }
            PebblesAction::Restart {
                difficulty,
                pebbles_count,
                max_pebbles_per_turn,
            } => {
                game.pebbles_count = pebbles_count;
                game.max_pebbles_per_turn = max_pebbles_per_turn;
                game.pebbles_remaining = pebbles_count;
                game.difficulty = difficulty;
                game.first_player = if get_random_u32() % 2 == 0 {
                    Player::User
                } else {
                    Player::Program
                };
                game.winner = None;

                PebblesEvent::CounterTurn(pebbles_count) // Can be another event to denote restart if defined
            }
        };
    }

    msg::reply(result, 0).expect("Failed to send game event reply");
}

#[no_mangle]
extern "C" fn state() {
    // Return the GameState using msg::reply
    unsafe {
        let game = PEBBLES_GAME.as_ref().expect("Game state not loaded");
        msg::reply(game.clone(), 0).expect("Failed to send game state");
    }
}
