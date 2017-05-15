extern crate serde;
extern crate serde_json;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait Runnable<U, R>
{
	/// Runs the game loop a single time and updates data.
	fn update(&mut self, context: &mut U);

	/// Draws the game at its current point in time.
	fn render(&self, context: &mut R);
}

pub trait Serializable
{
	/// Serialize itself as a string. Calling `deserialize` on this output must be transitive.
	fn serialize(&self) -> String;

	/// Deserializes the given data and overwrites itself, returning false if it is not possible.
	fn deserialize(&mut self, data: &str) -> bool;
}

impl<T> Serializable for T where T: Serialize + DeserializeOwned
{
	fn serialize(&self) -> String
	{
		serde_json::to_string(self).unwrap()
	}
	fn deserialize(&mut self, data: &str) -> bool
	{
		if let Ok(deserialize) = serde_json::from_str::<T>(data)
		{
			*self = deserialize;
			true
		}
		else
		{
			false
		}
	}
}

pub trait RunnableGame<U, R>: Runnable<U, R> + Serializable {}

pub trait Runner<U, R> : Sized
{
	/// Initializes the engine or returns the given error.
	fn create<E>() -> Result<Self, E>;

	/// Empties the event loop. Returns true to quit the game.
	///
	/// This function is given a callback function. It must be called after each key press and must
	/// pass the key being pressed. This allows for adding custom hooks to the game via shortcuts.
	///
	fn run_events<F>(&mut self, callback: F) -> bool where F: FnMut(u8, bool);

	/// Runs the given game a single frame. Returns true to quit the game.
	fn run_game<G>(&mut self, game: &mut G) -> bool where G: Runnable<U, R>;
}
