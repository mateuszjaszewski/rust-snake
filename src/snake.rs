use std::collections::VecDeque;

pub struct Snake {
    pub segments: VecDeque<Position>,
    direction: Direction,
    segments_to_add: i32,
    max_x: i32,
    max_y: i32,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub fn new(max_x: i32, max_y: i32, length: u8) -> Self {
        let center = Position::new(max_x / 2, max_y / 2);
        Snake {
            segments: (0..length).map(|i| Position { x: center.x, y: center.y + i as i32 }).collect(),
            direction: Direction::Up,
            max_x,
            max_y,
            segments_to_add: 0
        }
    }

    pub fn go_ahead(&mut self) {
        if let Some(head) = &mut self.segments.front() {
            let mut new_head = head.clone();
            match self.direction {
                Direction::Up => new_head.y -= 1,
                Direction::Down => new_head.y += 1,
                Direction::Right => new_head.x += 1,
                Direction::Left => new_head.x -= 1,
            }
            if new_head.x < 0 { new_head.x = self.max_x }
            if new_head.y < 0 { new_head.y = self.max_y }
            if new_head.x > self.max_x { new_head.x = 0 }
            if new_head.y > self.max_y { new_head.y = 0 }
            self.segments.push_front(new_head);
        }
        if self.segments_to_add == 0 {
            self.segments.pop_back();
        } else {
            self.segments_to_add -= 1;
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        if self.direction.is_allowed_turn(direction) {
            self.direction = direction;
        }
    }

    pub fn eat(&mut self) {
        self.segments_to_add += 5;
    }

    pub fn head(&self) -> Position {
        *self.segments.front().unwrap()
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl Direction {
    fn is_allowed_turn(&self, new_direction: Direction) -> bool {
        use Direction::*;
        match self {
            Up | Down => new_direction == Left || new_direction == Right,
            Left | Right => new_direction == Up || new_direction == Down,
        }
    }
}