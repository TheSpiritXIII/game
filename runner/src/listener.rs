use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::mpsc::{channel, Receiver};

use libloading::{Library, Symbol};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

use interface::{Runnable, RunnableGame};

use util;

macro_rules! try_str
{
	($e: expr) =>
	{
		try!($e.map_err(|err| format!("{}", err)));
	}
}

pub struct GameInstance<U, R>
{
	#[allow(dead_code)]
	lib: Library,
	game: Option<Box<RunnableGame<U, R>>>,
}

impl<U, R> GameInstance<U, R>
{
	fn load<P: AsRef<OsStr>>(path: P) -> Result<Self, String>
	{
		let lib = try_str!(Library::new(path));
		let game = unsafe
		{
			let entry: Symbol<extern fn() -> Option<Box<RunnableGame<U, R>>>> = try_str!(lib.get(b"game"));
			entry()
		};
		Ok(Self
		{
			lib: lib,
			game: Some(game.unwrap()),
		})
	}
	fn is_compatible<P: AsRef<OsStr>>(path: P, data: &str) -> bool
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
	fn from_data<P: AsRef<OsStr>>(path: P, data: &str) -> Result<Self, String>
	{
		let mut instance = Self::load(path)?;
		instance.game.as_mut().unwrap().deserialize(data);
		Ok(instance)
	}
	fn as_data(&self) -> String
	{
		self.game.as_ref().unwrap().serialize()
	}
	fn update(&mut self, context: &mut U)
	{
		// println!("Updating...");
		self.game.as_mut().unwrap().update(context);
	}
	fn render(&self, context: &mut R)
	{
		self.game.as_ref().unwrap().render(context);
	}
}

impl<U, R> Drop for GameInstance<U, R>
{
	fn drop(&mut self)
	{
		// Game must be dropped before lib.
		self.game = None;
	}
}

pub struct GameListener<U, R>
{
	instance: Option<GameInstance<U, R>>,
	watcher: RecommendedWatcher,
	receiver: Receiver<DebouncedEvent>,
	path: PathBuf,
	watching: bool,
	paused: bool,
	compatibility: bool,
}

impl<U, R> GameListener<U, R>
{
	pub fn new() -> Result<Self, String>
	{
		let mut exe_path = env::current_exe().map_err(|err| format!("{}", err))?;
		exe_path.pop();

		let path = exe_path.join("libgame.so");
		println!("PAT {:?}", path);
		let instance = GameInstance::load(&path)?;

		let (tx, rx) = channel();
		let watcher: RecommendedWatcher = try_str!(Watcher::new(tx, Duration::from_secs(2)));

		let instance = Self
		{
			instance: Some(instance),
			watcher: watcher,
			receiver: rx,
			path: path,
			watching: false,
			paused: false,
			compatibility: false,
		};
		instance.print_status();
		Ok(instance)
	}
	pub fn print_status(&self)
	{
		util::print_time();
		let paused = util::as_answer(self.paused);
		let watching = util::as_status(self.watching);
		let compatibility = util::as_status(self.compatibility);
		println!("Game Status: Paused: {}; Watching: {}; Compatibility Check: {};", paused, watching, compatibility);
	}
	pub fn pause_toggle(&mut self)
	{
		self.paused = !self.paused;
		let state = if !self.paused
		{
			"resumed"
		}
		else
		{
			"paused"
		};
		util::print_time();
		println!("Game was {}.", state);
	}
	pub fn update_t(&mut self, context2: &mut U)
	{
		self.instance.as_mut().unwrap().update(context2);
	}
	pub fn render_t(&self, context: &mut R)
	{
		self.instance.as_ref().unwrap().render(context);
	}
	pub fn poll(&mut self) -> Result<(), String>
	{
		if self.receiver.recv_timeout(Duration::from_millis(0)).is_ok()
		{
			util::print_time();
			println!("Detected change. compatibilitying new game compatibility...");
			let data = self.instance.as_mut().unwrap().as_data();
			let path_managed = self.path.with_extension("running");
			try_str!(fs::copy(&self.path, &path_managed));
			if self.compatibility || GameInstance::<U, R>::is_compatible(&path_managed, data.as_str())
			{
				self.reload(false)?;
			}
			else
			{
				util::print_time();
				println!("Automatic reload failed. Incompatible. Please reload manually.");
			}
		}
		Ok(())
	}
	pub fn watch_toggle(&mut self) -> Result<(), String>
	{
		self.watching = !self.watching;
		let state = if !self.watching
		{
			try_str!(self.watcher.unwatch(&self.path));
			"disabled"
		}
		else
		{
			try_str!(self.watcher.watch(&self.path, RecursiveMode::NonRecursive));
			"enabled"
		};
		util::print_time();
		let compatibility = util::as_status(self.compatibility);
		println!("Automatic reloading was {}; Compatibility Check: {}", state, compatibility);
		Ok(())
	}
	pub fn compatibility_toggle(&mut self) -> Result<(), String>
	{
		self.compatibility = !self.compatibility;
		util::print_time();
		println!("Automatic game compatibility check was {}.", util::as_status(self.compatibility));
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
		util::print_time();
		println!("{}ing game...", verb);
		if self.watching
		{
			try_str!(self.watcher.unwatch(&self.path));
		}
		if reset
		{
			self.instance = None;
			self.instance = Some(GameInstance::<U, R>::load(&self.path)?);
		}
		else
		{
			let data = self.instance.as_mut().unwrap().as_data();
			self.instance = None;
			self.instance = Some(GameInstance::<U, R>::from_data(&self.path, data.as_str())?);
		}
		if self.watching
		{
			try_str!(self.watcher.watch(&self.path, RecursiveMode::NonRecursive));
		}
		util::print_time();
		println!("{} is complete.", verb);

		Ok(())
	}
}

impl<U, R> Runnable<U, R> for GameListener<U, R>
{
	fn update(&mut self, context: &mut U)
	{
		self.update_t(context);
	}

	fn render(&self, context: &mut R)
	{
		self.render_t(context);
	}
}
