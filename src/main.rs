mod game;
mod renderer;

use piston::Button::Keyboard;
use piston::Event::Input;
use piston::Input::Button;
use piston_window::*;
use game::Game;
use game::Action;
use game::direction::Direction;
use crate::renderer::Renderable;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Snake", [640, 480])
        .resizable(false)
        .exit_on_esc(true)
        .fullscreen(false)
        .build().unwrap();

    let mut game = Game::new(32, 24);

    while let Some(event) = window.next() {
        let dt = event.update_args().map_or(0.0, |args| args.dt);
        game.update(dt);
        if let Input(Button(ButtonArgs { state: _, button, scancode: _ }), _) = &event {
            println!("{:?}", button);
            let action = match button {
                Keyboard(Key::Up) => Action::ChangeDirection(Direction::Up),
                Keyboard(Key::Down) => Action::ChangeDirection(Direction::Down),
                Keyboard(Key::Right) => Action::ChangeDirection(Direction::Right),
                Keyboard(Key::Left) => Action::ChangeDirection(Direction::Left),
                _ => Action::None
            };
            game.handle(action);
        }
        window.draw_2d(&event, |context, graphics, _device| {
            game.render(context, graphics);
        });
    }
}
