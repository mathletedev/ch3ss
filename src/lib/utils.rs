use crate::config::{
	ANIMATION_SCALE, ANIMATION_SPEED, HIDDEN_OPACITY, SPRITESHEET_SIZE, TILE_SIZE, WINDOW_SIZE,
};
use ggez::{
	graphics::{self, Color, DrawParam, Image, Rect},
	mint::Point2,
	Context, GameResult,
};
use std::f32::consts::PI;

pub fn draw_sprite(
	ctx: &mut Context,
	sprite: i8,
	x: usize,
	y: usize,
	z: usize,
	elapsed: f32,
	sprite_sheet: &Image,
	focused_layer: i8,
) -> GameResult {
	graphics::draw(
		ctx,
		sprite_sheet,
		DrawParam::default()
			.src(Rect {
				x: sprite as f32 / SPRITESHEET_SIZE,
				y: 0.0,
				w: 1.0 / SPRITESHEET_SIZE,
				h: 1.0,
			})
			.dest(Point2 {
				x: (x as f32 - y as f32 - 1.0) * TILE_SIZE.0 / 2.0 + WINDOW_SIZE.0 / 2.0,
				y: (x as f32 * 0.5 + y as f32 * 0.5) * TILE_SIZE.1 / 2.0
					+ 100.0 + z as f32 * TILE_SIZE.1 * 2.0
					+ if elapsed > PI / 4.0 || (focused_layer != -1 && focused_layer != z as i8) {
						0.0
					} else {
						((elapsed * ANIMATION_SPEED + ((x + y) as f32) * PI / 14.0).sin()
							* ANIMATION_SCALE)
							.min(0.0)
					} - if (sprite as f32) < SPRITESHEET_SIZE - 2.0 {
					TILE_SIZE.1 * 0.5
				} else {
					0.0
				},
			})
			.color(if focused_layer == -1 || focused_layer == z as i8 {
				Color::WHITE
			} else {
				Color {
					r: 1.0,
					g: 1.0,
					b: 1.0,
					a: HIDDEN_OPACITY,
				}
			}),
	)
}
