use crate::components::*;
use crate::game_state::xy_idx;
use crate::map::*;
use bracket_lib::prelude::*;
use legion::prelude::*;


pub fn init_world() -> World {
    let mut world = Universe::new().create_world();

    world.insert(
        (),
        vec![
            (
                Controlable::new(),
                Renderable {
                    bg: RGB::named(BLACK),
                    fg: RGB::named(YELLOW),
                    glyph: '1',
                    render_order: 0,
                },
                Position::new(41, 25),
            ),
            (
                Controlable::new(),
                Renderable {
                    bg: RGB::named(BLACK),
                    fg: RGB::named(YELLOW),
                    glyph: '2',
                    render_order: 0,
                },
                Position::new(42, 25),
            ),
            (
                Controlable::new(),
                Renderable {
                    bg: RGB::named(BLACK),
                    fg: RGB::named(YELLOW),
                    glyph: '3',
                    render_order: 0,
                },
                Position::new(43, 25),
            ),
            (
                Controlable::new(),
                Renderable {
                    bg: RGB::named(BLACK),
                    fg: RGB::named(YELLOW),
                    glyph: '4',
                    render_order: 0,
                },
                Position::new(44, 25),
            ),
        ],
    );

    world
}

pub fn init_resources() -> Resources {
    let player_position = xy_idx(40, 25);
    let mut resources = Resources::default();
    resources.insert(Map::new(player_position));
    resources
}
