use specs::prelude::*;
use crate::components::*;
use crate::map::*;
use crate::game_state::{xy_idx};

pub fn init_world() -> World {
    let player_position = xy_idx(40, 25);
    let mut world = World::new();
    world.register::<Renderable>();
    world.register::<Controlable>();

    world.insert(Map::new(player_position));

    world
}