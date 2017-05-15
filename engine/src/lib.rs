extern crate sdl2;

extern crate interface;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use interface::{Runner, Runnable};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

type UpdateContext = EventPump;
type RenderContext = Canvas<Window>;

pub struct GameEngine
{
	update: UpdateContext,
	render: Canvas<Window>,
}

impl GameEngine
{
	fn get_char(keycode: Keycode) -> u8
	{
		if keycode == Keycode::LCtrl || keycode == Keycode::RCtrl
		{
			1
		}
		else
		{
			let name = keycode.name();
			if name.len() == 1
			{
				name.as_bytes()[0]
			}
			else
			{
				0
			}
		}
	}
}

impl Runner<UpdateContext, RenderContext> for GameEngine
{
	fn create<E>() -> Result<Self, E>
	{
		let sdl_context = sdl2::init().expect("Unable to initialize SDL2.");
		let video_subsystem = sdl_context.video().expect("Unable to initialize SDL2 video.");
		let window = video_subsystem.window("Platformer", WINDOW_WIDTH, WINDOW_HEIGHT)
			.build()
			.expect("Unable to initialize SDL2 window.");

		let canvas = window.into_canvas().build().expect("Unable to initialize SDL2 renderer.");
		let event_pump = sdl_context.event_pump().expect("Unable to initialize SDL2 event pump.");

		Ok(Self
		{
			update: event_pump,
			render: canvas,
		})
	}

	fn run_events<F>(&mut self, mut callback: F) -> bool where F: FnMut(u8, bool)
	{
		for event in self.update.poll_iter()
		{
			match event
			{
				Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
				{
					return true;
				},
				Event::KeyDown { keycode: key, .. } =>
				{
					if let Some(keycode) = key
					{
						callback(Self::get_char(keycode), true)
					}
				}
				Event::KeyUp { keycode: key, .. } =>
				{
					if let Some(keycode) = key
					{
						callback(Self::get_char(keycode), false)
					}
				}
				_ => {}
			}
		}
		false
	}
	fn run_game<G>(&mut self, game: &mut G) -> bool where G: Runnable<UpdateContext, RenderContext>
	{
		self.render.set_draw_color(Color::RGB(0, 0, 0));
		self.render.clear();

		game.update(&mut self.update);
		game.render(&mut self.render);

		self.render.present();

		std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 60));
		false
	}
}
