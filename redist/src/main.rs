extern crate game;
extern crate engine;
extern crate interface;

use interface::Runner;

fn main()
{
	let mut engine = engine::GameEngine::create::<String>().unwrap();
	let mut game = game::Game::new();
	loop
	{
		let exit = engine.run_events(|_, _| {}) || engine.run_game(&mut game);
		if exit
		{
			break;
		}
	}
}
