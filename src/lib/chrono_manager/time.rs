use piston_window::text;
use crate::chrono_manager::ObjectInventory;
pub struct Time {
    seconds: f64,
    sim_seconds: f64,
    sim_speed: f64, // Radio of real time (s) to simulation time (s)
    pub objects: ObjectInventory
}

impl Time {
    pub fn new(sim_speed: f64, objects: ObjectInventory) -> Time {
        let mut time = Time {
            seconds: 0.0,
            sim_seconds: 0.0,
            sim_speed,
            objects
        };
        return time;
    }

    pub fn step(&mut self, dt: f64) {
        self.seconds += dt;
        self.sim_seconds += dt / self.sim_speed;
        self.objects.step(dt / self.sim_speed);
    }
}