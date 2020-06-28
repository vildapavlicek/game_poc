use bracket_lib::prelude::RGB;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Controlable {
    pub selected: bool,
}

impl Controlable {
    pub fn select(&mut self) {
        self.selected = !self.selected;
        println!("Select activated!")
    }

    pub fn new() -> Self {
        Controlable { selected: false }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub glyph: char,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn add_to_position(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}
