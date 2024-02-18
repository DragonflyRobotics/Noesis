extern crate quantx;

extern crate piston_window;
use piston_window::*;
use quantx::chrono_manager::ObjectInventory;

use crate::quantx::render_engine::{Object, ObjectTypes};
use crate::quantx::render_engine::{Box, Circle, Text};
use crate::quantx::chrono_manager::Time;


fn main() {

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
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
    //     color: [1.0, 0.0, 0.0, 1.0]
    // };
    // let mut circle1 = Circle {
    //     x: 0.0,
    //     y: 100.0,
    //     radius: 100.0,
    //     color: [1.0, 0.0, 0.0, 1.0]
    // };
    // let mut text1 = Text {
    //     position: [0.0, 0.0],
    //     color: [0.0, 0.0, 0.0, 1.0],
    //     size: 32,
    //     text: "Hello, world!".parse().unwrap(),
    //     window: &window
    // };
    //
    // let mut objects: ObjectInventory = ObjectInventory::new();
    // objects.add_object(ObjectTypes::Box(box1));
    // objects.add_object(ObjectTypes::Circle(circle1));
    // let mut time = Time::new(1.0, objects);
    while let Some(event) = window.next() {
    //     window.draw_2d(&event, |context, graphics, _device| {
    //         clear([1.0; 4], graphics);
    //         time.objects.draw(&context.transform, graphics);
    //     });
    //     time.step(0.1);
        window.draw_2d(&event, |c, g, _d| {
            let transform = c.transform.trans(10.0, 100.0);
            // Set a white background
            clear([1.0, 1.0, 1.0, 1.0], g);
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                "1234567890",
                &mut glyphs,
                &c.draw_state,
                transform, g
            );
        });
    }
}