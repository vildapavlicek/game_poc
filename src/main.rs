bracket_terminal::add_wasm_support!();

use bracket_lib::prelude::*;
mod game_state;
use game_state::State;
pub mod map;





// Implement the game loop



fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Bracket Terminal Example - A* Mouse")
        .build()?;
    let gs = State::new();
    main_loop(context, gs)
}
