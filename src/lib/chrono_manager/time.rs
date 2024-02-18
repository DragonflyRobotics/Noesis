use crate::chrono_manager::ObjectInventory;
use crate::render_engine::{Text, ObjectTypes};

pub struct Time {
    seconds: f64,
    sim_seconds: f64,
    sim_speed: f64, // Radio of real time (s) to simulation time (s)
    pub objects: ObjectInventory,
}

impl Time {
    pub fn new(sim_speed: f64, objects: ObjectInventory, screen_width: f64, screen_height: f64) -> Time {
        let mut obj = ObjectInventory{ objects: vec![] };
        let mut text1 = Text {
            x: screen_width * 0.87,
            y: screen_height,
            text: String::from("Hello World"),
            color: [0.0, 0.0, 0.0, 1.0],
            size: 32,
        };
        obj.add_object(ObjectTypes::Text(text1));
        for o in objects.objects {
            obj.add_object(o);
        }
        return Time {
            seconds: 0.0,
            sim_seconds: 0.0,
            sim_speed,
            objects: obj
        };
    }

    pub fn step(&mut self, dt: f64) {
        self.seconds += dt;
        self.sim_seconds += dt / self.sim_speed;
        self.objects.step(dt / self.sim_speed);
        if let ObjectTypes::Text(text) = self.objects.get(0) {
            text.text = ((self.sim_seconds * 100.0).round() / 100.0).to_string();
        }
    }
}