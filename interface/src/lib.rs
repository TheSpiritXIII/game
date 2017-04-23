extern crate libloading;
extern crate notify;
extern crate serde;
extern crate serde_json;

use std::time::Duration;
use std::sync::mpsc::{channel, Receiver};

use libloading::{Library, Symbol};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use serde::de::DeserializeOwned;

macro_rules! try_str
{
	($e: expr) =>
	{
		try!($e.map_err(|err| format!("{}", err)));
	}
}

pub trait Runnable<T>: Serializable
{
	/// Runs the game loop a single time.
	fn run(&mut self, context: &mut T);
}

pub trait Serializable
{
	fn serialize(&self) -> String;
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

pub struct GameInstance<T>
{
	#[allow(dead_code)]
	lib: Library,
	game: Option<Box<Runnable<T>>>,
}

impl<T> GameInstance<T>
{
	fn load<P: AsRef<std::ffi::OsStr>>(path: P) -> Result<Self, String>
	{
		let lib = try_str!(Library::new(path));
		let game = unsafe
		{
			let entry: Symbol<extern fn() -> Option<Box<Runnable<T>>>> = try_str!(lib.get(b"main"));
			entry()
		};
		Ok(Self
		{
			lib: lib,
			game: Some(game.unwrap()),
		})
	}
	fn is_compatible<P: AsRef<std::ffi::OsStr>>(path: P, data: &str) -> bool
	{
		if let Ok(mut instance) = Self::load(path)
		{
			instance.game.as_mut().unwrap().deserialize(data)
		}
		else
		{
			false
		}
	}
	fn from_data<P: AsRef<std::ffi::OsStr>>(path: P, data: &str) -> Result<Self, String>
	{
		let mut instance = Self::load(path)?;
		instance.game.as_mut().unwrap().deserialize(data);
		Ok(instance)
	}
	fn as_data(&self) -> String
	{
		self.game.as_ref().unwrap().serialize()
	}
	fn run(&mut self, context: &mut T)
	{
		self.game.as_mut().unwrap().run(context);
	}
}

impl<T> Drop for GameInstance<T>
{
	fn drop(&mut self)
	{
		// Game must be dropped before lib.
		self.game = None;
	}
}

pub struct GameListener<T>
{
	instance: Option<GameInstance<T>>,
	watcher: RecommendedWatcher,
	receiver: Receiver<notify::DebouncedEvent>,
	path: std::path::PathBuf,
	active: bool,
}

impl<T> GameListener<T>
{
	pub fn new() -> Result<Self, String>
	{
		let mut exe_path = std::env::current_exe().map_err(|err| format!("{}", err))?;
		exe_path.pop();

		let path = exe_path.join("libgame.so");

		let instance = GameInstance::load(&path)?;

		let (tx, rx) = channel();
		let watcher: RecommendedWatcher = try_str!(Watcher::new(tx, Duration::from_secs(2)));

		Ok(Self
		{
			instance: Some(instance),
			watcher: watcher,
			receiver: rx,
			path: path,
			active: false,
		})
	}
	pub fn run(&mut self, context: &mut T)
	{
		// self.instance.as_mut().unwrap().run(context);
		println!("I accept.")
	}
	pub fn print<F>(&mut self, t: &mut F) {
		println!("I accept.");
	}
	pub fn poll(&mut self) -> Result<(), String>
	{
		match self.receiver.recv_timeout(Duration::from_millis(0))
		{
			Ok(_) =>
			{
				println!("Detected change. Checking new game compatibility...");
				let data = self.instance.as_mut().unwrap().as_data();
				let path_managed = self.path.with_extension("running");
				try_str!(std::fs::copy(&self.path, &path_managed));
				if GameInstance::<T>::is_compatible(&path_managed, data.as_str())
				{
					self.reload(false)?;
				}
				else
				{
					println!("Automatic reload failed. Please reload manually.");
				}
			}
			Err(_) =>
			{
				// Ignore timeout.
			}
		}
		Ok(())
	}
	pub fn watch_toggle(&mut self) -> Result<(), String>
	{
		self.active = !self.active;
		let state = if self.active == false
		{
			try_str!(self.watcher.unwatch(&self.path));
			"disabled"
		}
		else
		{
			try_str!(self.watcher.watch(&self.path, RecursiveMode::NonRecursive));
			"enabled"
		};
		println!("Automatic reloading was {}.", state);
		Ok(())
	}
	pub fn reload(&mut self, reset: bool) -> Result<(), String>
	{
		let verb = if reset
		{
			"Restart"
		}
		else
		{
			"Reload"
		};
		print!("{}ing game...", verb);
		if self.active
		{
			try_str!(self.watcher.unwatch(&self.path));
		}
		if reset
		{
			self.instance = None;
			self.instance = Some(GameInstance::<T>::load(&self.path)?);
		}
		else
		{
			let data = self.instance.as_mut().unwrap().as_data();
			self.instance = None;
			self.instance = Some(GameInstance::<T>::from_data(&self.path, data.as_str())?);
		}
		if self.active
		{
			try_str!(self.watcher.watch(&self.path, RecursiveMode::NonRecursive));
		}
		println!(" {} is complete.", verb);

		Ok(())
	}
}
