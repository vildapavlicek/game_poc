use crate::components::*;
use crate::map::{Map, TileType};
use bracket_lib::prelude::*;
use legion::prelude::*;

pub struct State {
    pub ecs: World,
    map: Map,
    visible: Vec<bool>,
    mode: Mode,
    path: NavigationPath,
}

impl State {
    pub fn new(world: World) -> State {
        let player_position = xy_idx(40, 25);
        let state = State {
            ecs: world,
            map: Map::new(player_position),
            visible: vec![false; 80 * 50],
            mode: Mode::Waiting,
            path: NavigationPath::new(),
        };

        state
    }
}

impl GameState for State {
    #[allow(non_snake_case)]
    fn tick(&mut self, ctx: &mut BTerm) {
        // We'll use batched drawing
        let mut draw_batch = DrawBatch::new();

        // Set all tiles to not visible
        for v in &mut self.visible {
            *v = false;
        }

        // Obtain the player's visible tile set, and apply it
        //let player_position = self.map.index_to_point2d(self.player_position);
        {
            let query =
                <(Read<Position>, Read<Controlable>)>::query().filter(changed::<Position>());
            for (pos, _) in query.iter(&self.ecs) {
                let fov = field_of_view_set(Point::new(pos.get_x(), pos.get_y()), 8, &self.map);
                // Note that the steps above would generally not be run every frame!
                for idx in &fov {
                    self.visible[xy_idx(idx.x, idx.y)] = true;
                }
            }
        }

        // Clear the screen
        draw_batch.cls();

        // Iterate the map array, incrementing coordinates as we go.
        let mut y = 0;
        let mut x = 0;
        for (i, tile) in self.map.map.iter().enumerate() {
            // Render a tile depending upon the tile type; now we check visibility as well!
            let mut fg;
            let mut glyph = ".";

            match tile {
                TileType::Floor => {
                    fg = RGB::from_f32(1.5, 0.0, 0.0);
                }
                TileType::Wall => {
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                    glyph = "#";
                }
            }
            if !self.visible[i] {
                fg = fg.to_greyscale();
            }
            draw_batch.print_color(
                Point::new(x, y),
                glyph,
                ColorPair::new(fg, RGB::from_f32(0., 0., 0.)),
            );

            // Move the coordinates
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }

        match self.mode {
            Mode::Waiting => {
                let mouse_idx = self.map.point2d_to_index(INPUT.lock().mouse_tile(0));

                let query = <(Read<Position>, Write<Controlable>, Read<Renderable>)>::query();
                for (pos, mut controlable, rend) in query.iter_mut(&mut self.ecs) {
                    
                    if xy_idx(pos.get_x(), pos.get_y()) == mouse_idx && INPUT.lock().is_mouse_button_pressed(0) {
                        controlable.selected = true;
                        println!("Selected unit {}", rend.glyph)
                    }

                    if controlable.selected {
                        self.mode = Mode::Selected;
                    } else {
                        self.mode = Mode::Waiting;
                    }
                }
            }

            Mode::Selected => {                
                // Render a mouse cursor
                let mouse_pos = INPUT.lock().mouse_tile(0);
                let mouse_idx = self.map.point2d_to_index(mouse_pos);

                // for ECS
                let query = <(Read<Position>, Read<Controlable>)>::query();
                for (pos, con) in query.iter_mut(&mut self.ecs) {
                    if con.selected {
                        if self.map.map[mouse_idx as usize] != TileType::Wall {
                            let path = a_star_search(
                                xy_idx(pos.get_x(), pos.get_y()),
                                mouse_idx,
                                &self.map,
                            );
                            if path.success {
                                for loc in path.steps.iter().skip(1) {
                                    let x = (loc % 80) as i32;
                                    let y = (loc / 80) as i32;
                                    draw_batch.print_color(
                                        Point::new(x, y),
                                        "*",
                                        ColorPair::new(
                                            RGB::from_f32(1., 0., 0.),
                                            RGB::from_f32(0., 0., 0.),
                                        ),
                                    );
                                }

                                if INPUT.lock().is_mouse_button_pressed(0) {
                                    self.mode = Mode::Moving;
                                    self.path = path;
                                }
                            }
                        }
                    }
                }
            }

            Mode::Moving => {
                let query = <(Write<Position>, Read<Controlable>)>::query();
                for (mut pos, con) in query.iter_mut(&mut self.ecs) {
                    if con.selected {
                        let (x, y) = idx_xy(self.path.steps[0] as usize);
                        pos.set_position(x, y);
                    }
                }
                self.path.steps.remove(0);
                if self.path.steps.is_empty() {
                    self.mode = Mode::Waiting;
                }
            }
        }

        let player_chars = <(Read<Position>, Read<Renderable>, Read<Controlable>)>::query();
        for (pos, renderable, _) in player_chars.iter(&self.ecs) {
            draw_batch.print_color(
                Point::from_tuple(pos.get_position()),
                renderable.glyph,
                ColorPair::new(renderable.fg, renderable.bg),
            );
        }

        // Submit the rendering
        draw_batch.submit(0).expect("Batch error");
        render_draw_buffer(ctx).expect("Render error");
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Waiting,
    Selected,
    Moving,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % 80, idx as i32 / 80)
}
