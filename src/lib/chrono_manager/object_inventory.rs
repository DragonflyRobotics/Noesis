use piston_window::{Context, G2d, Glyphs, math};
use crate::render_engine::{Object, ObjectTypes};
use crate::render_engine::ObjectTypes::Text;

pub struct ObjectInventory {
    pub(crate) objects: Vec<ObjectTypes>
}

impl ObjectInventory {
    pub fn new() -> Self {
        ObjectInventory {
            objects: Vec::new()
        }
    }

    pub fn add_object(&mut self, object: ObjectTypes) {
        self.objects.push(object);
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        for object in &self.objects {
            object.draw(context, graphics, glyphs);
        }
    }

    pub fn step(&mut self, sim_dt: f64) {
        for object in &mut self.objects {
            object.step(sim_dt);
        }
    }

    pub fn get(&mut self, at: usize)  -> &mut ObjectTypes {
        return &mut self.objects[at];
    }
}

