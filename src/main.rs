use ggez::{
	conf::WindowMode,
	event::{self, EventHandler},
	graphics::{self, Color},
	ContextBuilder, GameError, GameResult,
};

struct State {}

impl State {
	pub fn new() -> Self {
		State {}
	}
}

impl EventHandler<GameError> for State {
	fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
		graphics::clear(ctx, Color::WHITE);

		graphics::present(ctx)?;

		Ok(())
	}
}

fn main() -> GameResult {
	let state = State::new();

	let (ctx, event_loop) = ContextBuilder::new("CH3SS", "mathletedev")
		.window_mode(WindowMode::default().dimensions(400.0, 400.0))
		.build()?;

	event::run(ctx, event_loop, state);
}
