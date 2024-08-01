use crate::game::position::Position;

#[derive(Clone)]
pub struct Area {
    pub max_x: i32,
    pub max_y: i32
}

impl Area {
    pub fn center(&self) -> Position {
        Position {x: self.max_x / 2, y: self.max_y / 2}
    }
}