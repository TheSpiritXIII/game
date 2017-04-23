extern crate sdl2;

extern crate interface;

use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::render::Renderer;

struct Context<'a, 'b: 'a, 'c, 'd: 'c>
{
	renderer: &'a mut Renderer<'b>,
	keyboard_state: &'c KeyboardState<'d>,
}

impl<'a, 'b, 'c, 'd> Context<'a, 'b, 'c, 'd>
{
	fn new(renderer: &'a mut Renderer<'b>, keyboard_state: &'c KeyboardState<'d>) -> Self
	{
		Self
		{
			renderer: renderer,
			keyboard_state: keyboard_state,
		}
	}
}

fn main_run() -> Result<(), String>
{
	let mut listener = interface::GameListener::new()?;

	let sdl_context = sdl2::init().expect("Unable to initialize SDL2.");
	let video_subsystem = sdl_context.video().expect("Unable to initialize SDL2 video.");
	let window = video_subsystem.window("Platformer", 800, 600)
		.opengl()
		.build()
		.expect("Unable to initialize SDL2 window.");

	let mut renderer = window.renderer().build().expect("Unable to initialize SDL2 renderer.");
	let mut event_pump = sdl_context.event_pump().expect("Unable to initialize SDL2 event pump.");

	'running: loop
	{
		let ctrl_pressed =
		{
			let key_state = event_pump.keyboard_state();
			let ctrl_pressed_left = key_state.is_scancode_pressed(Scancode::LCtrl);
			let ctrl_pressed_right = key_state.is_scancode_pressed(Scancode::RCtrl);
			ctrl_pressed_left || ctrl_pressed_right
		};
		// for event in event_pump.poll_iter()
		// {
		// 	match event
		// 	{
		// 		Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
		// 		{
		// 			break 'running;
		// 		},
		// 		Event::KeyDown { keycode: Some(Keycode::R), .. } =>
		// 		{
		// 			if ctrl_pressed
		// 			{
		// 				println!("Requested restart");
		// 				listener.reload(true)?;
		// 			}
		// 		},
		// 		Event::KeyDown { keycode: Some(Keycode::W), .. } =>
		// 		{
		// 			if ctrl_pressed
		// 			{
		// 				listener.watch_toggle()?;
		// 			}
		// 		}
		// 		_ => {}
		// 	}
		// }

		listener.poll()?;

		renderer.set_draw_color(Color::RGB(0, 0, 0));
		renderer.clear();

		let key_state = &event_pump.keyboard_state();
		{
				let mut context = Context
				{
					renderer: &mut renderer,
					keyboard_state: key_state,
				};
				listener.run(&mut context);
		}

		renderer.present();

		std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 60));
	}
	Ok(())
}

fn main()
{
	while let Err(err) = main_run()
	{
		println!("Error: {}", err);
		println!("Restarting game.");
	}
}
