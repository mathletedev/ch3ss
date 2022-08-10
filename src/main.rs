use ggez::{
	conf::WindowMode,
	event::{self, EventHandler},
	graphics::{self, Color, DrawParam, Image, Rect},
	mint::Point2,
	timer, Context, ContextBuilder, GameError, GameResult,
};

const WINDOW_SIZE: (f32, f32) = (1680.0, 1050.0);
const BOARD_SIZE: (usize, usize, usize) = (24, 24, 4);
const TILE_SIZE: (f32, f32) = (64.0, 64.0);

struct State {
	sprite_sheet: Image,
}

impl State {
	fn new(ctx: &mut Context) -> GameResult<State> {
		Ok(State {
			sprite_sheet: graphics::Image::new(ctx, "/spritesheet.png")?,
		})
	}
}

impl EventHandler<GameError> for State {
	fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
		graphics::clear(ctx, Color::WHITE);

		for x in 0..BOARD_SIZE.0 {
			for y in 0..BOARD_SIZE.1 {
				graphics::draw(
					ctx,
					&self.sprite_sheet,
					DrawParam::default()
						.src(Rect {
							x: if (x + y) % 2 == 0 { 0.5 } else { 0.0 },
							y: 0.0,
							w: 0.5,
							h: 1.0,
						})
						.dest(Point2 {
							x: (x as f32 - y as f32 - 1.0) * TILE_SIZE.0 / 2.0
								+ WINDOW_SIZE.0 / 2.0,
							y: (x as f32 * 0.5 + y as f32 * 0.5) * TILE_SIZE.1 / 2.0
								+ WINDOW_SIZE.1 / 8.0 + (((timer::time_since_start(ctx)
								.as_secs_f32() * 2.0 + x as f32 * 0.25)
								.sin() + (timer::time_since_start(ctx)
								.as_secs_f32() * 2.0 + y
								as f32
								* 0.25)
								.cos()) * 32.0)
								.round(),
						}),
				)?;
			}
		}

		graphics::present(ctx)?;

		Ok(())
	}
}

fn main() -> GameResult {
	let (mut ctx, event_loop) = ContextBuilder::new("CH3SS", "mathletedev")
		.window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
		.build()?;

	let state = State::new(&mut ctx).unwrap();

	event::run(ctx, event_loop, state);
}
