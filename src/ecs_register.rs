use specs::prelude::*;
use crate::components::*;

pub fn init_world() -> World {
    let mut world = World::new();
    world.register::<Renderable>();
    world.register::<Controlable>();
    world
}