#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;
extern crate uuid;

pub use entity_rust::{ events, tick };

system!( tick_logger {
	state! { }

	on!( tick, {}, {}) self, data => {
		println!("Tick!");
	}
});

pub fn main() {
    println!("Hello world");
	tick_logger::register();
	entity_rust::run(1);
}

