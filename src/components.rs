use specs::prelude::*;
use specs_derive::*;
use bracket_lib::prelude::{FontCharType, RGB};

#[derive(Component)]
pub struct Controlable {
    pub selected: bool
}

impl Controlable {
    pub fn select(&mut self) {
        self.selected = !self.selected;
    }
}

#[derive(Component, Clone)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order : i32
}