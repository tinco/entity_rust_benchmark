#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;
extern crate uuid;
extern crate sdl2;
extern crate sdl2_ttf;

mod graphics;
mod fps_tracker;

pub struct Hole {}

pub fn main() {
	graphics::graphics::register();
	fps_tracker::fps_tracker::register();
	graphics::graphics::start_graphics::trigger();
	entity_rust::run(60); // run 60 ticks per second
}
