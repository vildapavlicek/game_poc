use bracket_lib::prelude::*;
use crate::map::{TileType, Map};
use specs::prelude::*;

pub struct State {
    pub ecs: World,
    map: Map,
    player_position: usize,
    selected: bool,
    visible: Vec<bool>,
    mode: Mode,
    path: NavigationPath,
}

impl State {
    pub fn new(world: World) -> State {
        let player_position = xy_idx(40, 25);        
        let state = State {
            ecs: world,
            player_position: player_position,
            map: Map::new(player_position),
            selected: false,
            visible: vec![false; 80 * 50],
            mode: Mode::Waiting,
            path: NavigationPath::new(),
        };

        state
    }

    fn select(&mut self) {
        self.selected = !self.selected;
    }

    fn is_selected(&self) -> bool {
        self.selected
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
        let player_position = self.map.index_to_point2d(self.player_position);
        let fov = field_of_view_set(player_position, 8, &self.map);

        // Note that the steps above would generally not be run every frame!
        for idx in &fov {
            self.visible[xy_idx(idx.x, idx.y)] = true;
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
                if mouse_idx == self.player_position && INPUT.lock().is_mouse_button_pressed(0) {
                    self.select();
                }

                if self.is_selected() {
                    self.mode = Mode::Selected;
                } else {
                    self.mode = Mode::Waiting;
                }
                
            },

            Mode::Selected => {
                    // Render a mouse cursor
                let mouse_pos = INPUT.lock().mouse_tile(0);
                let mouse_idx = self.map.point2d_to_index(mouse_pos);
                draw_batch.print_color(
                    mouse_pos,
                    "X",
                    ColorPair::new(RGB::from_f32(0.0, 1.0, 1.0), RGB::from_f32(0.0, 1.0, 1.0)),
                );
                if self.map.map[mouse_idx as usize] != TileType::Wall {
                    let path = a_star_search(self.player_position, mouse_idx, &self.map);
                    if path.success {
                        for loc in path.steps.iter().skip(1) {
                            let x = (loc % 80) as i32;
                            let y = (loc / 80) as i32;
                            draw_batch.print_color(
                                Point::new(x, y),
                                "*",
                                ColorPair::new(RGB::from_f32(1., 0., 0.), RGB::from_f32(0., 0., 0.)),
                            );
                        }

                        if INPUT.lock().is_mouse_button_pressed(0) {
                            self.mode = Mode::Moving;
                            self.path = path;
                        }
                    }
                }
            },
            Mode::Moving => {
                    self.player_position = self.path.steps[0] as usize;
                    self.path.steps.remove(0);
                    if self.path.steps.is_empty() {
                    self.mode = Mode::Waiting;
                }
            },

        }
        // Render the player @ symbol
        let ppos = idx_xy(self.player_position);
        draw_batch.print_color(
            Point::from_tuple(ppos),
            "@",
            ColorPair::new(RGB::from_f32(1.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.)),
        );

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
