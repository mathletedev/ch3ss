use std::f32::consts::PI;

use ggez::{
	conf::WindowMode,
	event::{self, EventHandler, KeyCode},
	graphics::{self, Color, DrawParam, Image, Rect},
	mint::Point2,
	timer, Context, ContextBuilder, GameError, GameResult,
};

const WINDOW_SIZE: (f32, f32) = (800.0, 800.0);
const BOARD_SIZE: (usize, usize, usize) = (8, 8, 4);
const TILE_SIZE: (f32, f32) = (64.0, 64.0);
const ANIMATION_SPEED: f32 = 8.0;
const ANIMATION_SCALE: f32 = 32.0;
const HIDDEN_OPACITY: f32 = 0.1;

struct State {
	sprite_sheet: Image,
	focused_layer: i8,
	animation_init: f32,
}

impl State {
	fn new(ctx: &mut Context) -> GameResult<State> {
		Ok(State {
			sprite_sheet: Image::new(ctx, "/spritesheet.png")?,
			focused_layer: -1,
			animation_init: f32::MAX,
		})
	}
}

impl EventHandler<GameError> for State {
	fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
		graphics::clear(ctx, Color::BLACK);

		for x in 0..BOARD_SIZE.0 {
			for y in 0..BOARD_SIZE.1 {
				for z in 0..BOARD_SIZE.2 {
					let elapsed = timer::time_since_start(ctx).as_secs_f32() - self.animation_init;

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
									+ 100.0 + z as f32 * TILE_SIZE.1 * 2.0
									+ if elapsed > PI / 4.0
										|| (self.focused_layer != -1
											&& self.focused_layer != z as i8)
									{
										0.0
									} else {
										((elapsed * ANIMATION_SPEED + ((x + y) as f32) * PI / 14.0)
											.sin() * ANIMATION_SCALE)
											.min(0.0)
									},
							})
							.color(
								if self.focused_layer == -1 || self.focused_layer == z as i8 {
									Color::WHITE
								} else {
									Color {
										r: 1.0,
										g: 1.0,
										b: 1.0,
										a: HIDDEN_OPACITY,
									}
								},
							),
					)?;
				}
			}
		}

		graphics::present(ctx)?;

		Ok(())
	}

	fn key_down_event(
		&mut self,
		ctx: &mut Context,
		keycode: event::KeyCode,
		_keymods: event::KeyMods,
		_repeat: bool,
	) {
		match keycode {
			KeyCode::Space => self.animation_init = timer::time_since_start(ctx).as_secs_f32(),
			KeyCode::Key0 => {
				self.animation_init = timer::time_since_start(ctx).as_secs_f32();
				self.focused_layer = -1;
			}
			KeyCode::Key1 => {
				self.animation_init = timer::time_since_start(ctx).as_secs_f32();
				self.focused_layer = 0;
			}
			KeyCode::Key2 => {
				self.animation_init = timer::time_since_start(ctx).as_secs_f32();
				self.focused_layer = 1;
			}
			KeyCode::Key3 => {
				self.animation_init = timer::time_since_start(ctx).as_secs_f32();
				self.focused_layer = 2;
			}
			KeyCode::Key4 => {
				self.animation_init = timer::time_since_start(ctx).as_secs_f32();
				self.focused_layer = 3;
			}
			_ => {}
		}
	}
}

fn main() -> GameResult {
	let (mut ctx, event_loop) = ContextBuilder::new("CH3SS", "mathletedev")
		.window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
		.build()?;

	let state = State::new(&mut ctx).unwrap();

	event::run(ctx, event_loop, state);
}
