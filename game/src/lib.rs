#![feature(box_syntax)]

#[macro_use]
extern crate serde_derive;

extern crate sdl2;
extern crate serde;

extern crate interface;

use sdl2::keyboard::KeyboardState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

use interface::{Runnable};

struct World;

impl World
{
	fn has_collision(pos_y: i32) -> bool
	{
		pos_y >= 600
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct Character
{
	position: (f32, f32),
	velocity: (f32, f32),
	variable: f32,
}

impl Character
{
	fn update(&mut self)
	{
		let (_, ref mut vel_y) = self.velocity;
		let (_, ref mut pos_y) = self.position;
		*vel_y += 0.3;

		for _ in 0..(vel_y.floor() as i32)
		{
			if !World::has_collision((*pos_y + 1.) as i32)
			{
				*pos_y += 1.;
			}
			else
			{
				*vel_y = 0.;
				break;
			}
		}
	}
	fn draw(&self, renderer: &mut Renderer)
	{
		const WIDTH: u32 = 32;
		const HEIGHT: u32 = 32;
		let (pos_x, pos_y) = self.position;
		let rect_x = pos_x.round() as i32 - (WIDTH / 2) as i32;
		let rect_y = pos_y.round() as i32 - HEIGHT as i32;
		let rect = Rect::new(rect_x, rect_y, WIDTH, HEIGHT);
		renderer.set_draw_color(Color::RGB(255, 255, 0));
		renderer.fill_rect(rect).unwrap();
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct Game
{
	character: Character,
}

impl Game
{
	fn new() -> Self
	{
		Game
		{
			character: Character
			{
				position: (400., 300.),
				velocity: (0.0, 0.0),
				variable: 0.0,
			}
		}
	}
}

impl Runnable<(Renderer<'static>, KeyboardState<'static>)> for Game
{
	fn run(&mut self, context: &mut (Renderer, KeyboardState))
	{
		let (ref mut renderer, _) = *context;
		self.character.update();
		self.character.draw(renderer);
	}
}

#[no_mangle]
pub extern fn main() -> Option<Box<Runnable<(Renderer<'static>, KeyboardState<'static>)>>>
{
	Some(Box::new(Game::new()))
}
