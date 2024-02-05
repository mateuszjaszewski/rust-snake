mod snake;

use crate::snake::{Direction, Position, Snake};

use ButtonState::Press;
use piston::Button::Keyboard;
use piston::Event::Input;
use piston::Input::Button;
use piston_window::*;
use rand::prelude::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Snake", [640, 480])
        .resizable(false)
        .exit_on_esc(true)
        .fullscreen(false)
        .build().unwrap();

    let frame_time: f64 = 0.25;
    let mut passed: f64 = 0.0;

    let mut rng = thread_rng();
    let mut snake = Snake::new(32, 24, 3);
    let mut food = Position::new(rng.gen_range(0..32), rng.gen_range(0..24));

    while let Some(event) = window.next() {
        let dt = event.update_args().map_or(0.0, |args| args.dt);
        passed += dt;

        if passed > frame_time {
            snake.go_ahead();
            passed = 0.0;
        }

        if let Input(Button(ButtonArgs { state: Press, button, scancode: _ }), _) = &event {
            println!("{:?}", button);
            match button {
                Keyboard(Key::Up) => snake.turn(Direction::Up),
                Keyboard(Key::Down) => snake.turn(Direction::Down),
                Keyboard(Key::Right) => snake.turn(Direction::Right),
                Keyboard(Key::Left) => snake.turn(Direction::Left),
                _ => ()
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.05; 4], graphics);
            for segment in snake.segments.iter() {
                rectangle([247.0/255.0, 127.0/255.0, 190.0/255.0, 1.0],
                          [segment.x as f64 * 20.0, segment.y as f64 * 20.0, 20.0, 20.0],
                          context.transform,
                          graphics)
            }
            rectangle([1.0, 0.0, 0.0, 1.0],
                      [food.x as f64 * 20.0, food.y as f64 * 20.0, 20.0, 20.0],
                      context.transform,
                      graphics)
        });

        if snake.head() == food {
            snake.eat();
            food.x = rng.gen_range(0..32);
            food.y = rng.gen_range(0..24);
        }
    }
}
