use piston_window::{Context, G2d, Glyphs};
use crate::render_engine::ObjectTypes;

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

    // check for collisions in all the objects without any other collisions function in the objects
    pub fn check_collisions(&self) {
        for i in 0..self.objects.len() {
            for j in i+1..self.objects.len() {
                let obj1 = &self.objects[i];
                let obj2 = &self.objects[j];

                match (obj1, obj2) {
                    (ObjectTypes::Box(box1), ObjectTypes::Box(box2)) => {
                        if box1.x < box2.x + box2.width &&
                            box1.x + box1.width > box2.x &&
                            box1.y < box2.y + box2.height &&
                            box1.y + box1.height > box2.y {
                                println!("Collision detected between box {} and box {}", i, j);
                        }
                    },
                    // Add more cases for other object types
                    (ObjectTypes::Circle(circle1), ObjectTypes::Circle(circle2)) => {
                        let dx = circle1.x - circle2.x;
                        let dy = circle1.y - circle2.y;
                        let distance = (dx*dx + dy*dy).sqrt();

                        if distance < circle1.radius + circle2.radius {
                            println!("Collision detected between circle {} and circle {}", i, j);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

