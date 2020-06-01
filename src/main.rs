bracket_terminal::add_wasm_support!();
use bracket_lib::prelude::*;
pub mod game_state;
use game_state::State;
pub mod map;
mod ecs_register;
use ecs_register::init_world;
pub mod components;


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Bracket Terminal Example - A* Mouse")
        .build()?;
    
    let ecs = init_world();
    let gs = State::new(ecs);
    
    main_loop(context, gs)
}
