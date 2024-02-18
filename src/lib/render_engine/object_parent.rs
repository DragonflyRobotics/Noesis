use piston_window::{clear, Context, G2d, Glyphs, math, PistonWindow, text, TextureSettings, Transformed};
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
    pub fn draw(&self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        match self {
            ObjectTypes::Box(box_object) => {
                box_object.draw(context, graphics, glyphs);
            },
            ObjectTypes::Circle(circle_object) => {
                circle_object.draw(context, graphics, glyphs);
            }
            ObjectTypes::Text(text_object) => {
                text_object.draw(context, graphics, glyphs);
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
    fn draw(&self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs);
    fn step(&mut self, dt: f64);
}

impl Object for Box {
    fn draw(&self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        rectangle(self.color, // red
                  [self.x, self.y, self.width, self.height],
                  context.transform,
                  graphics);
    }

    fn step(&mut self, dt: f64) {
        self.step(dt);
    }
}

impl Object for Circle {
    fn draw(&self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        piston_window::ellipse(self.color, // red
                  [self.x, self.y, self.radius, self.radius],
                  context.transform,
                  graphics);
    }

    fn step(&mut self, dt: f64) {
        self.step(dt);
    }
}

impl Object for Text {
    fn draw(&self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        let transform = context.transform.trans(self.x, self.y);
        text::Text::new_color(self.color, self.size).draw(
            self.text.as_str(),
            glyphs,
            &context.draw_state,
            transform, graphics).expect("TODO: panic message");
    }

    fn step(&mut self, dt: f64) {
        self.step(dt);
    }
}