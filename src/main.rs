pub mod config;
pub mod lib;
use crate::{config::WINDOW_SIZE, lib::state::State};
use ggez::{conf::WindowMode, event, ContextBuilder, GameResult};

fn main() -> GameResult {
	let (mut ctx, event_loop) = ContextBuilder::new("CH3SS", "mathletedev")
		.window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
		.build()?;

	let state = State::new(&mut ctx).unwrap();

	event::run(ctx, event_loop, state);
}
