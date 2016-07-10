system!( fps_tracker {
	use entity_rust::tick as game_tick;
	use graphics::graphics::draw;

	use std::time::{ Instant };
	use sdl2::pixels::{ Color };
	use sdl2::render::{ TextureQuery };
	use std::path::Path;
	use sdl2::rect::Rect;

	const NANOS_PER_SEC: u32 = 1_000_000_000;

	state {
		last_frame: Instant,
		last_tick: Instant,
		ticks_per_second: i64,
		frames_per_second: i64,
		ticks_since_fps_update: i64
	} {
		last_frame = Instant::now();
		last_tick = Instant::now();
		ticks_per_second = 0;
		frames_per_second = 0;
		ticks_since_fps_update = 0;
	}

	on game_tick , {}, {}, (self, data) => {
		self.ticks_since_fps_update += 1;
		let last_tick = self.last_tick;
		let now = Instant::now();
		let tick_duration = (now - last_tick).subsec_nanos();
		if tick_duration > 0 && self.ticks_since_fps_update > 5 {
			self.ticks_per_second = (NANOS_PER_SEC / tick_duration) as i64;
		}
		self.last_tick = Instant::now();
	}

	on_sync draw, (self, context) => {
		let last_frame: Instant;
		let ticks_per_second: i64;
		last_frame = self.last_frame;
		self.last_frame = Instant::now();
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