#![feature(box_syntax)]

#[macro_use]
extern crate serde_derive;

extern crate sdl2;
extern crate serde;

extern crate interface;

use sdl2::EventPump;
use sdl2::keyboard::{KeyboardState, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use interface::{Runnable, RunnableGame};

struct World;

impl World
{
	// fn has_collision(pos_x: i32, pos_y: i32) -> bool
	// {
	// 	World::has_collision_x(pos_x) || World::has_collision_y(pos_y)
	// }
	fn has_collision_x(pos_x: i32) -> bool
	{
		pos_x <= 0 || pos_x >= 800
	}
	fn has_collision_y(pos_y: i32) -> bool
	{
		pos_y <= 0 || pos_y >= 600
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct Character
{
	pos_x: i32,
	pos_y: i32,
	vel_x: f32,
	vel_y: f32,
	jump_counter: u8,
	jumping: bool,
	jump_start: bool,
}

const PLAYER_WIDTH: i32 = 32;
const PLAYER_HEIGHT: i32 = 32;
const SPEED_GRAVITY: f32 = 0.3;

impl Character
{
	fn update(&mut self, keyboard_state: &KeyboardState)
	{
		const SPEED_MAX: f32 = 8.;
		const SPEED_ACCELERATION: f32 = 0.1;
		const SPEED_DECCELERATION: f32 = 0.2;

		let pressed_left = keyboard_state.is_scancode_pressed(Scancode::Left) as i32;
		let pressed_right = keyboard_state.is_scancode_pressed(Scancode::Right) as i32;
		let direction = pressed_left - pressed_right;
		let pressed = direction != 0;

		self.vel_x -= SPEED_ACCELERATION * direction as f32;
		if self.vel_x.abs() > SPEED_MAX
		{
			self.vel_x = -SPEED_MAX * direction as f32
		}
		else if !pressed && self.vel_x.abs() >= std::f32::EPSILON
		{
			self.vel_x -= self.vel_x.signum() * SPEED_DECCELERATION;
			if self.vel_x.abs() <= SPEED_DECCELERATION + std::f32::EPSILON
			{
				self.vel_x = 0.0;
			}
		}

		let mut sign_x = self.vel_x.signum() as i32;
		let check_x = sign_x * 16;

		let collide_side = pressed && World::has_collision_x(self.pos_x + check_x);

		if keyboard_state.is_scancode_pressed(Scancode::Z)
		{
			let ground = World::has_collision_y(self.pos_y + 1);
			if collide_side && !self.jump_start && !ground
			{
				sign_x = -sign_x;
				self.vel_x = sign_x as f32 * 4.0;
				self.vel_y = -4.0;
			}
			else if ground && !self.jump_start
			{
				self.jump_counter = 0;
			}
			if !self.jumping && ground && self.jump_counter == 0
			{
				self.jumping = true;
			}
			if self.jumping && self.jump_counter < 8
			{
				self.vel_y -= 3.0 / (1.0 + self.jump_counter as f32);
				self.jump_counter += 1;
			}
			if !self.jump_start
			{
				self.jump_start = true;
			}
		}
		else
		{
			self.jumping = false;
			self.jump_start = false;
		}
		if collide_side && self.vel_y > 0.0
		{
			self.vel_y += 0.01;
		}
		else
		{
			self.vel_y += SPEED_GRAVITY;
		}

		let sign_y = self.vel_y.signum() as i32;
		let check_y = sign_y * 16 - 16;
		for _ in 0..(self.vel_y.abs().round() as i32)
		{
			if !World::has_collision_y(self.pos_y + check_y)
			{
				self.pos_y += sign_y;
			}
			else
			{
				self.vel_y = 0.0;
				break;
			}
		}

		let check_x = sign_x * 16;
		for _ in 0..(self.vel_x.abs().ceil() as i32)
		{
			if !World::has_collision_x(self.pos_x + check_x)
			{
				self.pos_x += sign_x;
			}
			else
			{
				self.vel_x = 0.0;
				break;
			}
		}
	}
	fn draw(&self, renderer: &mut Canvas<Window>)
	{
		let rect_x = self.pos_x - (PLAYER_WIDTH / 2);
		let rect_y = self.pos_y - PLAYER_HEIGHT;
		let rect = Rect::new(rect_x, rect_y, PLAYER_WIDTH as u32, PLAYER_HEIGHT as u32);
		renderer.set_draw_color(Color::RGB(255, 255, 255));
		renderer.fill_rect(rect).unwrap();
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game
{
	character: Character,
}

impl Game
{
	pub fn new() -> Self
	{
		Game
		{
			character: Character
			{
				pos_x: 400,
				pos_y: 300,
				vel_x: 0.0,
				vel_y: 0.0,
				jump_counter: 0,
				jumping: false,
				jump_start: false,
			}
		}
	}
}

impl Runnable<EventPump, Canvas<Window>> for Game
{
	fn update(&mut self, context: &mut EventPump)
	{
		// println!("Updatuing");
		let keyboard_state = context.keyboard_state();
		self.character.update(&keyboard_state);
	}
	fn render(&self, context: &mut Canvas<Window>)
	{
		// println!("RENDERING");
		self.character.draw(context);
	}
}

impl RunnableGame<EventPump, Canvas<Window>> for Game {}

#[no_mangle]
pub extern fn game() -> Option<Box<RunnableGame<EventPump, Canvas<Window>>>>
{
	Some(Box::new(Game::new()))
}
