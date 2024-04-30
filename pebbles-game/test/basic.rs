use gtest::{Program, System};
use pebbles_game_io::*;

#[test]
fn test_pebbles_game() {
    let mut system = System::new();
    let mut program = Program::new();

    system.init(&mut program);
    system.handle(&mut program);
    system.state(&mut program);
}
