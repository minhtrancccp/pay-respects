use std::io::{stderr, Write, Stderr, Result};

use crossterm::{
	queue,
	cursor,
	style::Print,
	event::{self, KeyCode, KeyEvent},
	terminal::{self, ClearType},
	Command,
};

pub struct Term(Stderr);
pub struct Events;
pub enum Action {
	Up,
	Down,
	Accept,
	Exit ,
}

impl Term {
	pub fn new() -> Self {
		terminal::enable_raw_mode().expect("Failed to enable raw mode");
		Term(stderr())
	}

	fn queue<T: Command>(&mut self, cmd: T) -> Result<()> {
		queue!(self.0, cmd)
	}

	fn flush(&mut self) -> Result<()> {
		self.0.flush()
	}

	fn move_up(&mut self, n: u16) -> Result<()> {
		self.queue(cursor::MoveUp(n))
	}

	fn move_down(&mut self, n: u16) -> Result<()> {
		self.queue(cursor::MoveDown(n))
	}

	fn move_column_start(&mut self) -> Result<()> {
		self.queue(cursor::MoveToColumn(0))
	}

	fn move_start(&mut self) -> Result<()> {
		self.queue(cursor::MoveTo(0, 0))
	}

	fn move_to(&mut self, x: u16, y: u16) -> Result<()> {
		self.queue(cursor::MoveTo(x, y))
	}

	fn move_to_row(&mut self, y: u16) -> Result<()> {
		self.queue(cursor::MoveTo(0, y))
	}

	fn move_to_column(&mut self, x: u16) -> Result<()> {
		self.queue(cursor::MoveToColumn(x))
	}

	fn clear_line(&mut self) -> Result<()> {
		self.queue(cursor::MoveToColumn(0))?;
		self.queue(terminal::Clear(ClearType::CurrentLine))
	}

	fn hide_cursor(&mut self) -> Result<()> {
		self.queue(cursor::Hide)
	}

	fn show_cursor(&mut self) -> Result<()> {
		self.queue(cursor::Show)
	}

	fn write(&mut self, text: &str) -> Result<()> {
		queue!(self.0, Print(text))?;
		self.0.flush()
	}
}

impl Drop for Term {
	fn drop(&mut self) {
		terminal::disable_raw_mode().expect("Failed to disable raw mode");
		self.show_cursor().expect("Failed to show cursor");
		self.move_start().expect("Failed to move cursor to start");
		self.flush().expect("Failed to flush terminal");
	}
}

impl Events {
	pub fn read() -> Result<KeyEvent> {
		// wait for a key event
		loop {
			if let event::Event::Key(key_event) = event::read()? {
				return Ok(key_event);
			}
		}
	}
}

impl Action {
	pub fn from_event(event: KeyEvent) -> Option<Self> {
		match event.code {
			// Up: Up arrow or k
			KeyCode::Up | KeyCode::Char('k') => Some(Action::Up),
			// Down: Down arrow or j
			KeyCode::Down | KeyCode::Char('j') => Some(Action::Down),
			KeyCode::Enter => Some(Action::Accept),
			// Exit: Escape key or q
			KeyCode::Esc | KeyCode::Char('q') => Some(Action::Exit),
			_ => None,
		}
	}
}
