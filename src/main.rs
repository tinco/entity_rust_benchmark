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
use sdl2::rect::Rect;

pub struct Hole {}

event!{ start_graphics , }

system!( tick_logger {
	state! { }

	on!( tick, {}, {}) self, data => {
		//println!("Tick!");
	}
});

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn get_centered_rect(screen_width: u32, screen_height: u32, rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            //println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            //println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (screen_width as i32 - w) / 2;
    let cy = (screen_height as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

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
			let mut font = ttf_context.load_font(font_path, 128).unwrap();
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

				let text = format!("Ticks: {}\nFPS: {}", ticks_per_second, frames_per_second);

				 // render a surface, and convert it to a texture bound to the renderer
				let surface = font.render(&text).blended(text_color).unwrap();
				let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
				let TextureQuery { width, height, .. } = texture.query();

				let padding = 64;
				let target = super::get_centered_rect(SCREEN_WIDTH, SCREEN_HEIGHT, width, height, SCREEN_WIDTH - padding, SCREEN_HEIGHT - padding);

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
