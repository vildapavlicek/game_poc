bracket_terminal::add_wasm_support!();
use bracket_lib::prelude::*;
pub mod game_state;
use game_state::State;
mod ecs_register;
pub mod map;
use ecs_register::*;
pub mod components;
pub mod systems;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Bracket Terminal Example - A* Mouse")
        .build()?;

    let world = init_world();
    let resources = init_resources();
    let gs = State::new(world);

    main_loop(context, gs)
}
