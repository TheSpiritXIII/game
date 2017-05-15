extern crate chrono;
extern crate libloading;
extern crate notify;

extern crate engine;
extern crate interface;

mod listener;
mod util;

use interface::Runner;

use listener::GameListener;

fn main_run() -> Result<(), String>
{
	let mut engine = engine::GameEngine::create::<String>()?;
	let mut listener = GameListener::new()?;
	let mut key = [false; 256];
	loop
	{
		listener.poll()?;
		{
			let callback = |c, enabled|
			{
				key[c as usize] = enabled;
				if key[1]
				{
					if key['R' as usize]
					{
						util::print_time();
						println!("Requested restart.");
						listener.reload(true).unwrap();
					}
					if key['W' as usize]
					{
						listener.watch_toggle().unwrap();
					}
					if key['P' as usize]
					{
						listener.pause_toggle();
					}
					if key['C' as usize]
					{
						listener.compatibility_toggle().unwrap();
					}
					if key['S' as usize]
					{
						listener.print_status();
					}
				}
			};
			if engine.run_events(callback)
			{
				break
			}
		}
		if engine.run_game(&mut listener)
		{
			break
		}
	}
	Ok(())
}

fn main()
{
	while let Err(err) = main_run()
	{
		util::print_time();
		println!("Error: {}", err);
		util::print_time();
		println!("Restarting game due to error.");
	}
	println!("Hello");
}
