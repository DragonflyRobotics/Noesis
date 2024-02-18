use piston_window::{G2d, math};
use crate::render_engine::{Object, ObjectTypes};

pub struct ObjectInventory {
    objects: Vec<ObjectTypes>
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

    pub fn draw(&self, transform: &math::Matrix2d, graphics: &mut G2d) {
        for object in &self.objects {
            object.draw(transform, graphics);
        }
    }

    pub fn step(&mut self, sim_dt: f64) {
        for object in &mut self.objects {
            object.step(sim_dt);
        }
    }
}

