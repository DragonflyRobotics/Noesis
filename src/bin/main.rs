extern crate quantx;

extern crate piston_window;

use quantx::chrono_manager::Time;
use std::time as stdtime;
use piston_window::*;
use quantx::chrono_manager::ObjectInventory;

use crate::quantx::render_engine::ObjectTypes;
use crate::quantx::render_engine::{Box, Circle};


fn main() {

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("Roboto-Regular.ttf");
    let mut glyphs = Glyphs::new(
        font,
        window.create_texture_context(),
        TextureSettings::new(),
    ).unwrap();
    // let box1 = Box {
    //     width: 100.0,
    //     height: 100.0,
    //     x: 0.0,
    //     y: 0.0,
    //     color: [1.0, 0.0, 0.0, 1.0],
    //     velocity: 10.0
    // };
    // let box2 = Box {
    //     width: 100.0,
    //     height: 100.0,
    //     x: 0.0,
    //     y: 200.0,
    //     color: [1.0, 0.0, 0.0, 1.0],
    //     velocity: 5.0
    // };
    let circle1 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 100.0,
        color: [1.0, 0.0, 0.0, 1.0],
        velocity: 10.0
    };
    let circle2 = Circle {
        x: 0.0,
        y: 200.0,
        radius: 100.0,
        color: [1.0, 0.0, 0.0, 1.0],
        velocity: 5.0
    };
    let mut objects: ObjectInventory = ObjectInventory::new();
    objects.add_object(ObjectTypes::Circle(circle1));
    objects.add_object(ObjectTypes::Circle(circle2));
    let mut time = Time::new(1.0/8.0, objects, 640.0, 480.0);
    let mut tic = stdtime::Instant::now();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            time.objects.draw(&context, graphics, &mut glyphs);
            glyphs.factory.encoder.flush(_device);
        });
        time.step(tic.elapsed().as_secs_f64());
        tic = stdtime::Instant::now();
        time.objects.check_collisions();
    }
}