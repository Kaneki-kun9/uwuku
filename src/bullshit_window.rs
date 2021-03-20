

use piston::window::{WindowSettings, BuildFromWindowSettings};

use std::error::Error;

pub struct TetrisWindow {
	settings: WindowSettings,
}

impl TetrisWindow {
	pub fn render(&self) {
		println!("___________");
		println!("| | | | | |");
		println!("| | | | | |");
		println!("| | | | | |");
		println!("| | | | | |");
		println!("| | | | | |");
		println!("___________");
	}
}

impl BuildFromWindowSettings for TetrisWindow{
	fn build_from_window_settings(settings: &WindowSettings) -> Result<Self, Box<dyn Error>> {
		println!("Using Tetris Backend implementation");
		Ok(TetrisWindow {
			settings: settings.clone(),
		})
	}
}
