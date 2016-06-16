#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;
extern crate uuid;
extern crate sdl2;

pub use entity_rust::{ events, tick };
pub use std::time::{ Duration, Instant };
pub use std::thread;


pub struct Hole {}

event!{ start_graphics , }

system!( tick_logger {
	state! { }

	on!( tick, {}, {}) self, data => {
		//println!("Tick!");
	}
});

const NANOS_PER_SEC: u32 = 1_000_000_000;

system!( graphics {
	state! {
		last_frame: Option<super::Instant>,
		last_tick: Option<super::Instant>,
		ticks_per_second: i64
	}

	on! (tick , {}, {} ) self, data => {
		let now = super::Instant::now();
		let last_tick = self.last_tick.unwrap_or(super::Instant::now());
		let tick_duration = (now - last_tick).subsec_nanos();
		if tick_duration > 0 {
			self.ticks_per_second = (super::NANOS_PER_SEC / tick_duration) as i64;
		}
		self.last_tick = Some(super::Instant::now());
	}

	on! (start_graphics , {}, {} ) self, data => {
		super::thread::spawn(move || {
			extern crate sdl2;
			extern crate sdl2_ttf;

			use sdl2::pixels::{Color};
			use sdl2::event::{Event};
			use sdl2::render::{Renderer, Texture, TextureQuery};
			use std::path::Path;
			use sdl2::rect::Rect;


			let SCREEN_WIDTH = 800;
			let SCREEN_HEIGHT = 600;

			let sdl_context = sdl2::init().expect("Could not initialize sdl2");
			let ttf_context = sdl2_ttf::init().expect("Could not initialize ttf");
			let video = sdl_context.video().expect("Could not set up sdl context");
			
			let window = video
				.window("Entity Rust benchmark", SCREEN_WIDTH, SCREEN_HEIGHT)
				.position_centered()
				.opengl()
				.build()
				.expect("Expected build to be something.");

			let mut renderer = window
				.renderer()
				.accelerated()
				.build()
				.expect("Could not build renderer");

			renderer.set_draw_color(Color::RGB(50, 0, 0));
			renderer.clear();
			renderer.present();

			let font_path = Path::new("res/SourceSansPro-Regular.ttf");
			let mut font = ttf_context.load_font(font_path, 64).unwrap();
			let text_color = Color::RGBA(255, 255, 255, 255);

			let mut events = sdl_context.event_pump().unwrap();

			// loop until we receive a QuitEvent
			'draw_loop : loop {
				for event in events.poll_iter() {
					match event {
						Event::Quit{..} => break 'draw_loop,
						_               => continue
					}
				}

				let last_frame: super::Instant;
				let ticks_per_second: i64;

				{
					let mut state = STATE.write().expect("Graphics lock is corrupted");
					last_frame = state.last_frame.unwrap_or(super::Instant::now());
					state.last_frame = Some(super::Instant::now());
					ticks_per_second = state.ticks_per_second;
				}

				let now = super::Instant::now();
				let frame_duration = now - last_frame;
				let frames_per_second = super::NANOS_PER_SEC / frame_duration.subsec_nanos();

				let text = format!("Ticks: {}  FPS: {}", ticks_per_second, frames_per_second);

				 // render a surface, and convert it to a texture bound to the renderer
				let surface = font.render(&text).blended(text_color).unwrap();
				let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
				let TextureQuery { width, height, .. } = texture.query();

				let padding = 10;
				let target = Rect::new(10, 10, 110, 20);

				renderer.clear();
				renderer.copy(&mut texture, None, Some(target));
				renderer.present();
			}

		});
	}
});

pub fn main() {
	tick_logger::register();
	graphics::register();
	start_graphics::trigger();
	entity_rust::run(60);
}
