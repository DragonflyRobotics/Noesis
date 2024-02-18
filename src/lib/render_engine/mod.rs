pub mod box_object;
pub mod object_parent;
pub mod circle_object;
mod text_object;

pub use box_object::Box;
pub use object_parent::{Object, ObjectTypes};
pub use circle_object::Circle;
pub use text_object::Text;