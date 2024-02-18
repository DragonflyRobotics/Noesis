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
    let ref font = assets.join("Roboto-Regular.ttf");
    let mut glyphs = Glyphs::new(
        font,
        window.create_texture_context(),
        TextureSettings::new(),
    ).unwrap();
    let box1 = Box {
        width: 100.0,
        height: 100.0,
        x: 0.0,
        y: 0.0,
        color: [1.0, 0.0, 0.0, 1.0]
    };
    let mut circle1 = Circle {
        x: 0.0,
        y: 100.0,
        radius: 100.0,
        color: [1.0, 0.0, 0.0, 1.0]
    };
    let mut text1 = Text {
        x: 10.0,
        y: 100.0,
        text: String::from("Hello World"),
        color: [0.0, 0.0, 0.0, 1.0],
        size: 32,
    };
    let mut objects: ObjectInventory = ObjectInventory::new();
    objects.add_object(ObjectTypes::Box(box1));
    objects.add_object(ObjectTypes::Circle(circle1));
    objects.add_object(ObjectTypes::Text(text1));
    let mut time = Time::new(1.0, objects);
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            time.objects.draw(&context, graphics, &mut glyphs);
            glyphs.factory.encoder.flush(_device);
        });
        time.step(0.1);
    }
}