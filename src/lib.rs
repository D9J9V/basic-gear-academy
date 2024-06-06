#![no_std]

use gstd::{exec, msg};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

fn get_random_u32() -> u32 {
    //* There are two difficulty levels in the game: DifficultyLevel::Easy and DifficultyLevel::Hard. Program should choose the pebbles count to be removed randomly at the easy level, and find the best pebbles count (find a winning strategy) at the hard level.
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[no_mangle]
extern "C" fn init() {
    { /* Write init() function that:

         Receives PebblesInit using the msg::load function;
         Checks input data for validness;
         Chooses the first player using the exec::random function;
         Processes the first turn if the first player is Program.
         Fills the GameState structure.*/
    }
    let init: PebblesInit = msg::load().expect("Not Loaded");

    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

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
        msg::reply_bytes("Initialized", 0).expect("Not initialized");
    }
}

#[no_mangle]
extern "C" fn handle() {
    { /*Write the handle() function that:

         Receives PebblesAction using msg::load function;
         Checks input data for validness;
         Processes the User's turn and check whether they win;
         Processes the Program turn and check whether it wins;
         Send a message to the user with the correspondent PebblesEvent; */
    }
    let action: PebblesAction = msg::load().expect("Action not loaded");

    unsafe {
        let game = PEBBLES_GAME.as_mut().expect("Game state not loaded");
        let result: PebblesEvent = match action {
            PebblesAction::Turn(pebbles_taken) => {
                if pebbles_taken >= 1
                    && pebbles_taken <= game.max_pebbles_per_turn
                    && pebbles_taken <= game.pebbles_remaining
                {
                    game.pebbles_remaining -= pebbles_taken;
                    if game.pebbles_remaining == 0 {
                        game.winner = Some(game.first_player.clone());
                    } else {
                        game.first_player = match game.first_player {
                            Player::User => Player::Program,
                            Player::Program => Player::User,
                        };
                    }
                }
            }
            PebblesAction::GiveUp => {
                game.winner = match game.first_player {
                    Player::User => Some(Player::Program),
                    Player::Program => Some(Player::User),
                };
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
            }
        };
    };
    msg::reply(result, 0).expect("Failed to encode/reply event");
}

#[no_mangle]
extern "C" fn state() {
    //Write the state() function that returns the GameState structure using the msg::reply function.
    unsafe {
        let game = PEBBLES_GAME.take().expect("Game state not loaded");
        msg::reply(game, 0).expect("State not shared")
    }
}
