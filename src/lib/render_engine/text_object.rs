pub struct Text {
    pub text: String,
    pub color: [f32; 4],
    pub size: u32,
    pub x: f64,
    pub y: f64,
}

impl Text {
    pub fn step(&mut self, _dt: f64) {
    }
}