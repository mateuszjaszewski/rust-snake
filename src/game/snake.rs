use std::collections::VecDeque;
use piston_window::{Context, G2d, rectangle};
use crate::game::area::Area;
use crate::game::direction::Direction;
use crate::game::position::Position;
use crate::renderer::Renderable;

pub struct Snake {
    segments: VecDeque<Position>,
    direction: Direction,
    segments_to_add: i32
}

impl Snake {
    pub fn new(area: &Area, length: u8) -> Self {
        let start_position = area.center();
        Self {
            segments: (0..length).map(|i| Position { x: start_position.x, y: start_position.y + i as i32 }).collect(),
            segments_to_add: 0,
            direction:
            Direction::Up,
        }
    }

    pub fn do_move(&mut self, area: &Area) {
        if let Some(head) = &mut self.segments.front() {
            let mut new_head = head.clone();
            match self.direction {
                Direction::Up => new_head.y -= 1,
                Direction::Down => new_head.y += 1,
                Direction::Right => new_head.x += 1,
                Direction::Left => new_head.x -= 1,
            }
            if new_head.x < 0 { new_head.x = area.max_x }
            if new_head.y < 0 { new_head.y = area.max_y }
            if new_head.x > area.max_x { new_head.x = 0 }
            if new_head.y > area.max_y { new_head.y = 0 }
            self.segments.push_front(new_head);
        }
        if self.segments_to_add == 0 {
            self.segments.pop_back();
        } else {
            self.segments_to_add -= 1;
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        if self.direction.is_turn_allowed(direction) {
            self.direction = direction;
        }
    }

    pub fn eat(&mut self) {
        self.segments_to_add += 1;
    }

    pub fn head(&self) -> Position {
        *self.segments.front().unwrap()
    }
}

impl Renderable for Snake {
    fn render(&self, context: Context, graphics: &mut G2d) {
        for segment in self.segments.iter() {
            rectangle(crate::renderer::SNAKE_COLOR,
                      [segment.x as f64 * crate::renderer::SEGMENT_SIZE, segment.y as f64 * crate::renderer::SEGMENT_SIZE, crate::renderer::SEGMENT_SIZE, crate::renderer::SEGMENT_SIZE],
                      context.transform, graphics)
        }
    }
}