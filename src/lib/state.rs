use super::{setup::setup_board, utils::draw_sprite};
use crate::config::BOARD_SIZE;
use ggez::{
	event::{self, EventHandler, KeyCode},
	graphics::{self, Color, Image},
	timer, Context, GameError, GameResult,
};

pub struct State {
	pub board: Vec<Vec<Vec<i8>>>,
	sprite_sheet: Image,
	focused_layer: i8,
	animation_init: f32,
}

impl State {
	pub fn new(ctx: &mut Context) -> GameResult<State> {
		Ok(State {
			board: setup_board(),
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

					draw_sprite(
						ctx,
						if (x + y + z) % 2 == 0 { 12 } else { 13 },
						x,
						y,
						z,
						elapsed,
						&self.sprite_sheet,
						self.focused_layer,
					)?;

					let piece = self.board[x][y][z];
					if piece == -1 {
						continue;
					}

					draw_sprite(
						ctx,
						piece,
						x,
						y,
						z,
						elapsed,
						&self.sprite_sheet,
						self.focused_layer,
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
			KeyCode::Space => self.focused_layer = BOARD_SIZE.2 as i8,
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

// fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
// 	let grid_x = (x / TILE_SIZE.0).floor();
// 	let grid_y = (y / TILE_SIZE.1).floor();

// 	let tile_x = grid_x - grid_y;
// 	let tile_y = 2.0 * (grid_x + grid_y);

// 	if tile_x < 0 || tile_x >= BOARD_SIZE.0 || tile_y < 0 || tile_y >= BOARD_SIZE.1 {
// 		self.selected_tile = (-1, -1)
// 	}

// 	self.selected_tile = ()
// }
