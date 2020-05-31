use bracket_pathfinding::prelude::*;
use bracket_lib::prelude::*;
use crate::game_state::{xy_idx};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub map: Vec<TileType>
}

impl Map {
    pub fn new(player_position: usize) -> Self {
        let mut m = Map { map: vec![TileType::Floor; 80 * 50] };
        for x in 0..80 {
            m.map[xy_idx(x, 0)] = TileType::Wall;
            m.map[xy_idx(x, 49)] = TileType::Wall;
        }
        for y in 0..50 {
            m.map[xy_idx(0, y)] = TileType::Wall;
            m.map[xy_idx(79, y)] = TileType::Wall;
        }

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..1400 {
            let x = rng.range(1, 79);
            let y = rng.range(1, 49);
            let idx = xy_idx(x, y);
            if player_position != idx {
                m.map[idx] = TileType::Wall;
            }
        };

        m
    }

    pub fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > 79 || y < 1 || y > 49 {
            return false;
        }
        let idx = (y * 80) + x;
        self.map[idx as usize] == TileType::Floor
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.map[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = (idx % 80) as i32;
        let y = (idx / 80) as i32;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - 80, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + 80, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - 80) - 1, 1.4));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - 80) + 1, 1.4));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + 80) - 1, 1.4));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + 80) + 1, 1.4));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let p1 = Point::new(idx1 % 80, idx1 / 80);
        let p2 = Point::new(idx2 % 80, idx2 / 80);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(80, 50)
    }
}