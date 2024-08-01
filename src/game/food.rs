use piston_window::{Context, G2d, rectangle};
use rand::{Rng, thread_rng};
use crate::game::area::Area;
use crate::game::position::Position;
use crate::renderer::{FOOD_COLOR, Renderable, SEGMENT_SIZE};

pub type Food = Position;

impl Food {
    pub fn new(area: &Area) -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(0..area.max_x),
            y: rng.gen_range(0..area.max_y),
        }
    }
}

impl Renderable for Food {
    fn render(&self, context: Context, graphics: &mut G2d) {
        rectangle(FOOD_COLOR,
                  [self.x as f64 * SEGMENT_SIZE, self.y as f64 * SEGMENT_SIZE, SEGMENT_SIZE, SEGMENT_SIZE],
                  context.transform,
                  graphics)
    }
}