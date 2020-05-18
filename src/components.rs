#[derive(Component)]
struct Controlable {
    selected: bool;
};

impl Controlable {
    pub fn select(&self) {
        self.selected = true;
    }

    pub fn deselect(&self) {
        self.selected = false;
    }
}