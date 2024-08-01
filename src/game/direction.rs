#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn is_turn_allowed(&self, new_direction: Direction) -> bool {
        use Direction::*;
        match self {
            Up | Down => new_direction == Left || new_direction == Right,
            Left | Right => new_direction == Up || new_direction == Down,
        }
    }
}