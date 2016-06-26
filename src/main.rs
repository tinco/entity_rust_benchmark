#[macro_use]
extern crate entity_rust;
#[macro_use]
extern crate lazy_static;
extern crate shared_mutex;
extern crate uuid;
extern crate sdl2;
extern crate sdl2_ttf;

pub struct Hole {}

system!( graphics {
	use std::thread;
	use sdl2;
	use sdl2::pixels::{Color};
	use sdl2::event::{Event};
	use sdl2_ttf;
	use sdl2::render::Renderer;

	pub struct Context<'a> {
		pub renderer: &'a mut Renderer<'static>,
		pub sdl: &'a sdl2::Sdl,
		pub ttf: &'a sdl2_ttf::Sdl2TtfContext
	}
	
	event!{ start_graphics , }

	sync_event!{ draw, context: &'a mut super::Context<'b>}

	state! {}

	on start_graphics , {}, {}, (self, data) => {
		thread::spawn(move || {
			const SCREEN_WIDTH : u32 = 800;
			const SCREEN_HEIGHT : u32 = 600;

			let sdl_context = sdl2::init().expect("Could not initialize sdl2");
			let ttf_context = sdl2_ttf::init().expect("Could not initialize ttf");
			let video = sdl_context.video().expect("Could not set up sdl video context");

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

			let mut events = sdl_context.event_pump().unwrap();

			// loop until we receive a QuitEvent
			'draw_loop : loop {
				for event in events.poll_iter() {
					match event {
						Event::Quit{..} => break 'draw_loop,
						_               => continue
					}
				}
				renderer.clear();
				{
					let mut context = Context {
						renderer: &mut renderer,
						sdl: &sdl_context,
						ttf: &ttf_context
					};
					draw::trigger(&mut context);
				}
				renderer.present();
			}

		});
	}
});

system!( fps_tracker {
	use entity_rust::tick as game_tick;
	use std::time::{ Instant };
	use sdl2::pixels::{ Color };
	use sdl2::render::{ TextureQuery };
	use std::path::Path;
	use sdl2::rect::Rect;

	use super::graphics::draw;

	const NANOS_PER_SEC: u32 = 1_000_000_000;

	state! {
		last_frame: Option<Instant>,
		last_tick: Option<Instant>,
		ticks_per_second: i64,
		frames_per_second: i64,
		ticks_since_fps_update: i64
	}

	on game_tick , {}, {}, (self, data) => {
		self.ticks_since_fps_update += 1;
		let last_tick = self.last_tick.unwrap_or(Instant::now());
		let now = Instant::now();
		let tick_duration = (now - last_tick).subsec_nanos();
		if tick_duration > 0 && self.ticks_since_fps_update > 5 {
			self.ticks_per_second = (NANOS_PER_SEC / tick_duration) as i64;
		}
		self.last_tick = Some(Instant::now());
	}

	on_sync draw, (self, context) => {
		let last_frame: Instant;
		let ticks_per_second: i64;
		last_frame = self.last_frame.unwrap_or(Instant::now());
		self.last_frame = Some(Instant::now());
		ticks_per_second = self.ticks_per_second;

		let mut frames_per_second : i64;

		if self.ticks_since_fps_update > 5 {
			self.ticks_since_fps_update = 0;
			
			let now = Instant::now();
			let frame_duration = now - last_frame;
			frames_per_second = (NANOS_PER_SEC / frame_duration.subsec_nanos()) as i64;
			self.frames_per_second = frames_per_second;
		}

		frames_per_second = self.frames_per_second;

		let font_path = Path::new("res/SourceSansPro-Regular.ttf");
		let font = context.ttf.load_font(font_path, 64).unwrap();
		let text_color = Color::RGBA(255, 255, 255, 255);
		let text = format!("Ticks: {}  FPS: {}", ticks_per_second, frames_per_second);

		 // render a surface, and convert it to a texture bound to the renderer
		let surface = font.render(&text).blended(text_color).unwrap();
		let mut texture = context.renderer.create_texture_from_surface(&surface).unwrap();
		let TextureQuery { width, height, .. } = texture.query();

		let padding = 10;
		let target = Rect::new(10, 10, 110, 20);

		let ref mut renderer = context.renderer;

		renderer.copy(&mut texture, None, Some(target));
	}
} );

pub fn main() {
	graphics::register();
	fps_tracker::register();
	graphics::start_graphics::trigger();
	entity_rust::run(60); // run 60 ticks per second
}
