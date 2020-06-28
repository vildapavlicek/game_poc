use crate::components::*;
use crate::map::*;
use crate::game_state::{xy_idx};
use legion::prelude::*;
use bracket_lib::prelude::*;

pub fn init_world() -> World {
    let player_position = xy_idx(40, 25);
    let mut resources = Resources::default();
    let mut world = Universe::new().create_world();

    resources.insert(Map::new(player_position));

    world.insert(
        (),
        vec! [
            (Controlable::new(), Renderable{ bg: RGB::named(BLACK), fg: RGB::named(YELLOW), glyph: '@', render_order: 0,})
        ],        
    );
    

    world
}