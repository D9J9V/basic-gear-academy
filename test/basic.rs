use gtest::{Program, System};
use pebbles_game_io::*;

#[test]
fn test_pebbles_game() {
    let mut system = System::new();
    let program = Program::current(&system);

    // Test game initialization
    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
    };

    let res = system.send(program, init_msg);
    assert!(!res.main_failed()); // Ensure initialization did not fail
    assert_eq!(
        res.read_main().expect("Initialization response"),
        "Initialized"
    );

    // Test initial state retrieval
    let res = system.send(program, ());
    let game_state: GameState = res.read_state().expect("Game state");

    assert_eq!(game_state.pebbles_count, 20);
    assert_eq!(game_state.max_pebbles_per_turn, 3);
    assert_eq!(game_state.pebbles_remaining, 20);
    assert!(game_state.winner.is_none());

    // Test user turn
    let turn_msg = PebblesAction::Turn(3);
    let res = system.send(program, turn_msg);
    let event: PebblesEvent = res.read_main().expect("Pebbles event");

    if let PebblesEvent::CounterTurn(pebbles_left) = event {
        assert_eq!(pebbles_left, 17);
    } else {
        panic!("Unexpected event after user turn");
    }

    // Test program give up
    let give_up_msg = PebblesAction::GiveUp;
    let res = system.send(program, give_up_msg);
    let event: PebblesEvent = res.read_main().expect("Pebbles event");

    if let PebblesEvent::Won(player) = event {
        assert_eq!(player, Player::User);
    } else {
        panic!("Unexpected event after give up");
    }

    // Test game restart
    let restart_msg = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 30,
        max_pebbles_per_turn: 4,
    };
    let res = system.send(program, restart_msg);
    let event: PebblesEvent = res.read_main().expect("Pebbles event");

    if let PebblesEvent::Restarted {
        first_player,
        pebbles_count,
    } = event
    {
        assert!(matches!(first_player, Player::User | Player::Program));
        assert_eq!(pebbles_count, 30);
    } else {
        panic!("Unexpected event after restart");
    }

    // Final state check after restart
    let res = system.send(program, ());
    let game_state: GameState = res.read_state().expect("Game state");

    assert_eq!(game_state.pebbles_count, 30);
    assert_eq!(game_state.max_pebbles_per_turn, 4);
    assert_eq!(game_state.pebbles_remaining, 30);
    assert_eq!(game_state.difficulty, DifficultyLevel::Hard);
    assert!(game_state.winner.is_none());
}
