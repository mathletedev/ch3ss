use ggez::{conf::WindowMode, event, ContextBuilder, GameResult};

pub mod lib;

use crate::lib::{constants::WINDOW_SIZE, state::State};

fn main() -> GameResult {
	let (mut ctx, event_loop) = ContextBuilder::new("CH3SS", "mathletedev")
		.window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
		.build()?;

	let mut state = State::new(&mut ctx).unwrap();

	state.board[0][0][0] = 0;
	state.board[3][4][2] = 5;

	event::run(ctx, event_loop, state);
}
