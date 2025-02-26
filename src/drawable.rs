use raylib::prelude::RaylibDrawHandle;

pub trait Drawable {
    fn draw(&self, handler: &mut RaylibDrawHandle);
}
