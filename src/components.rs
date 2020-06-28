use bracket_lib::prelude::{FontCharType, RGB};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Controlable {
    pub selected: bool
}

impl Controlable {
    pub fn select(&mut self) {
        self.selected = !self.selected;
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
    pub render_order : i32
}