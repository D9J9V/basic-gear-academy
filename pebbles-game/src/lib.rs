#![no_std]

use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

extern "C" fn init() {
    //Code to initialize the game
    let init = PebblesInit::default();
    unsafe {
        PEBBLES_GAME = Some(GameState {
            pebbles_count: init.pebbles_count,
            max_pebbles_per_turn: init.max_pebbles_per_turn,
            pebbles_remaining: init.pebbles_count,
            difficulty: init.difficulty,
            first_player: init.first_player,
            winner: None,
        });
    }
}

extern "C" fn handle() {
    //Code to handle the game
    let action = PebblesAction::Turn(1); // This line might be replaced depending on the game's input handling
    unsafe {
        let game = PEBBLES_GAME.as_mut().unwrap();
        match action {
            PebblesAction::Turn(pebbles_taken) => {
                if pebbles_taken >= 1
                    && pebbles_taken <= game.max_pebbles_per_turn
                    && pebbles_taken <= game.pebbles_remaining
                {
                    game.pebbles_remaining -= pebbles_taken;
                    if game.pebbles_remaining == 0 {
                        game.winner = Some(game.first_player);
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
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    //Code to get the game state
    unsafe {
        let game = PEBBLES_GAME.as_ref().unwrap();
        let _ = game;
    }
}
