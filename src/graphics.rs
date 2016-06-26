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
					// draw::trigger(&mut context);
				}
				renderer.present();
			}

		});
	}
});
