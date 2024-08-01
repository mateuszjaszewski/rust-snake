use piston_window::{Context, G2d};
use piston_window::types::Color;

pub const BACKGROUND_COLOR: Color = [0.05; 4];
pub const SNAKE_COLOR: Color = [247.0 / 255.0, 127.0 / 255.0, 190.0 / 255.0, 1.0];
pub const FOOD_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
pub const SEGMENT_SIZE: f64 = 20.0;

pub trait Renderable {
    fn render(&self, context: Context, graphics: &mut G2d);
}
