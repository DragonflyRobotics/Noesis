use piston_window::{G2d, Glyphs, math, PistonWindow, TextureSettings, Transformed};
use piston_window::rectangle;
use crate::render_engine::box_object::Box;
use crate::render_engine::circle_object::Circle;
use crate::render_engine::text_object::Text;

pub enum ObjectTypes {
    Box(Box),
    Circle(Circle),
    Text(Text)
}

impl ObjectTypes {
    pub fn draw(&self, transform: &math::Matrix2d, graphics: &mut G2d, glyphs: &mut Glyphs) {
        match self {
            ObjectTypes::Box(box_object) => {
                box_object.draw(transform, graphics);
            },
            ObjectTypes::Circle(circle_object) => {
                circle_object.draw(transform, graphics);
            }
            ObjectTypes::Text(text_object) => {
                text_object.draw(transform, graphics);
            }
        }
    }

    pub fn step(&mut self, dt: f64) {
        match self {
            ObjectTypes::Box(box_object) => {
                box_object.step(dt);
            },
            ObjectTypes::Circle(circle_object) => {
                circle_object.step(dt);
            }
            ObjectTypes::Text(text_object) => {
                text_object.step(dt);
            }
        }
    }
}

pub trait Object {
    fn draw(&self, transform: &math::Matrix2d, graphics: &mut G2d);
    fn step(&mut self, dt: f64);
}

impl Object for Box {
    fn draw(&self, transform: &math::Matrix2d, graphics: &mut G2d) {
        rectangle(self.color, // red
                  [self.x, self.y, self.width, self.height],
                  *transform,
                  graphics);
    }

    fn step(&mut self, dt: f64) {
        self.step(dt);
    }
}

impl Object for Circle {
    fn draw(&self, transform: &math::Matrix2d, graphics: &mut G2d) {
        piston_window::ellipse(self.color, // red
                  [self.x, self.y, self.radius, self.radius],
                  *transform,
                  graphics);
    }

    fn step(&mut self, dt: f64) {
        self.step(dt);
    }
}

impl Object for Text {
    fn draw(&self, transform: &math::Matrix2d, graphics: &mut G2d) {
        // piston_window::text::Text::new_color(self.color, self.size as u32).draw(
        //     &self.text,
        //     &glyphs,
        //     &piston_window::draw_state::DrawState::default(),
        //     transform.trans(self.position[0], self.position[1]),
        //     graphics
        // );
    }

    fn step(&mut self, dt: f64) {
        self.step(dt);
    }
}