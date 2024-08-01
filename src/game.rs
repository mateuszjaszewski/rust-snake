pub mod snake;
pub mod area;
pub mod position;
pub mod direction;
mod food;

use piston_window::{clear, Context, G2d};
use area::Area;
use snake::Snake;
use direction::Direction;
use food::Food;
use crate::renderer::{BACKGROUND_COLOR, Renderable};

const FRAME_TIME: f64 = 0.25;
pub struct Game {
    area: Area,
    snake: Snake,
    food: Food,
    current_frame_time: f64,
}

pub enum Action {
    ChangeDirection(Direction),
    None
}

impl Game {
    pub fn new(max_x: i32, max_y: i32) -> Self {
        let area = Area { max_x, max_y };
        let food = Food::new(&area);
        let snake = Snake::new(&area, 3);
        Self { area, food, snake, current_frame_time: 0.0 }
    }

    pub fn update(&mut self, dt: f64) {
        self.current_frame_time += dt;
        if self.current_frame_time > FRAME_TIME {
            self.next_frame();
            self.current_frame_time = 0.0;
        }
    }

    pub fn handle(&mut self, action: Action) {
        match action {
            Action::ChangeDirection(direction) => self.snake.turn(direction),
            _ => {}
        }
    }

    fn next_frame(&mut self) {
        self.snake.do_move(&self.area);
        if self.snake.head() == self.food {
            self.snake.eat();
            self.food = Food::new(&self.area)
        }
    }
}

impl Renderable for Game {
    fn render(&self, context: Context, graphics: &mut G2d) {
        clear(BACKGROUND_COLOR, graphics);
        self.snake.render(context, graphics);
        self.food.render(context, graphics);
    }
}

