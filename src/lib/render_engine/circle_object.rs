pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: [f32; 4]
}

impl Circle {
    pub fn step(&mut self, dt: f64) {
        self.x += 1.0 * dt;
        self.y += 1.0 * dt;
    }
}