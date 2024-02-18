use crate::render_engine::Circle;

pub struct Box {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub color: [f32; 4]
}

impl Box {
    pub fn step(&mut self, dt: f64) {
        self.x += 10.0 * dt;
        self.y += 0.0 * dt;
    }
}