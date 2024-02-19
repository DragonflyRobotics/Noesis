pub struct Box {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub color: [f32; 4],
    pub velocity: f64,
}

impl Box {
    pub fn step(&mut self, dt: f64) {
        self.x += 0.0 * dt;
        self.y += self.velocity * dt;
    }
}