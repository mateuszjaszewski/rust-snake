use ButtonState::Press;
use piston::Button::Keyboard;
use piston::Event::Input;
use piston::Input::Button;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Snake", [640, 480])
        .resizable(false)
        .exit_on_esc(true)
        .fullscreen(false)
        .build().unwrap();

    let mut x = 10;
    let mut y = 10;
    let mut direction = (0, 0);

    let frame_time: f64 = 0.2;
    let mut passed: f64 = 0.0;

    while let Some(event) = window.next() {
        let dt = event.update_args().map_or(0.0, |args| args.dt);
        passed += dt;

        if passed > frame_time {
            x += direction.0 * 10;
            y += direction.1 * 10;
            passed = 0.0;
        }

        if let Input(Button(ButtonArgs { state: Press, button, scancode: _ }), _) = &event {
            println!("{:?}", button);
            match button {
                Keyboard(Key::Up) => direction = (0, -1),
                Keyboard(Key::Down) => direction = (0, 1),
                Keyboard(Key::Right) => direction = (1, 0),
                Keyboard(Key::Left) => direction = (-1, 0),
                _ => ()
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [x as f64, y as f64, 10.0, 10.0],
                      context.transform,
                      graphics);
        });
    }
}
