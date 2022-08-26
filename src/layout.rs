use druid::widget::{Either, Flex, Label};
use druid::{Color, FontDescriptor, FontFamily, Widget, WidgetExt};

use crate::{del_btn, enter_btn, keyboard_btn, letter_box, restart_btn, GameState, WobbleState};

pub fn heading() -> impl Widget<WobbleState> {
	Label::new("wobble")
		.with_font(FontDescriptor::new(FontFamily::SERIF))
		.with_text_size(60.0)
		.with_text_color(Color::BLACK)
		.center()
		.padding(10.0)
}

pub fn guess_tiles(letter_grid_space: f64, letter_space: f64) -> impl Widget<WobbleState> {
	Flex::column()
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(0, 0), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(0, 1), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(0, 2), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(0, 3), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(0, 4), letter_space)
				.with_spacer(letter_grid_space),
			letter_space,
		)
		.with_spacer(letter_grid_space)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(1, 0), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(1, 1), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(1, 2), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(1, 3), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(1, 4), letter_space)
				.with_spacer(letter_grid_space),
			letter_space,
		)
		.with_spacer(letter_grid_space)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(2, 0), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(2, 1), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(2, 2), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(2, 3), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(2, 4), letter_space)
				.with_spacer(letter_grid_space),
			letter_space,
		)
		.with_spacer(letter_grid_space)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(3, 0), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(3, 1), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(3, 2), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(3, 3), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(3, 4), letter_space)
				.with_spacer(letter_grid_space),
			letter_space,
		)
		.with_spacer(letter_grid_space)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(4, 0), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(4, 1), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(4, 2), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(4, 3), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(4, 4), letter_space)
				.with_spacer(letter_grid_space),
			letter_space,
		)
		.with_spacer(letter_grid_space)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(5, 0), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(5, 1), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(5, 2), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(5, 3), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(5, 4), letter_space)
				.with_spacer(letter_grid_space),
			letter_space,
		)
}

pub fn keyboard(letter_grid_space: f64, key_space: f64) -> impl Widget<WobbleState> {
	Flex::column()
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(keyboard_btn('q'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('w'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('e'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('r'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('t'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('y'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('u'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('i'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('o'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('p'), key_space)
				.with_spacer(letter_grid_space),
			0.3,
		)
		.with_spacer(1.0)
		.with_flex_child(
			Flex::row()
				.with_flex_spacer(0.2)
				.with_flex_child(keyboard_btn('a'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('s'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('d'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('f'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('g'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('h'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('j'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('k'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('l'), key_space)
				.with_flex_spacer(0.2),
			0.3,
		)
		.with_spacer(1.0)
		.with_flex_child(
			Flex::row()
				.with_flex_child(enter_btn("Enter".to_string()), 1.0)
				.with_spacer(1.0)
				.with_flex_child(keyboard_btn('z'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('x'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('c'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('v'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('b'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('n'), key_space)
				.with_spacer(key_space)
				.with_flex_child(keyboard_btn('m'), key_space)
				.with_spacer(1.0)
				.with_flex_child(del_btn("del".to_string()), 1.0),
			0.3,
		)
}

pub fn end_screen(letter_grid_space: f64) -> impl Widget<WobbleState> {
	Flex::column()
		.with_flex_child(
			Either::new(
				|data, _env| data.finished == GameState::Won,
				Flex::row()
					.with_spacer(letter_grid_space)
					.with_flex_child(
						Label::new("You won!!")
							.with_text_size(30.0)
							.with_text_color(Color::BLACK)
							.center(),
						letter_grid_space,
					)
					.with_spacer(letter_grid_space),
				Flex::row()
					.with_spacer(letter_grid_space)
					.with_flex_child(
						Label::new("You lost")
							.with_text_size(30.0)
							.with_text_color(Color::BLACK)
							.center(),
						letter_grid_space,
					)
					.with_spacer(letter_grid_space)
					.with_flex_child(
						Label::new(move |data: &WobbleState, _env: &_| {
							if data.finished == GameState::Lost {
								format!("The word was: \"{}\"", data.word.to_uppercase())
							} else {
								String::new()
							}
						})
						.with_text_size(20.0)
						.with_text_color(Color::BLACK)
						.center(),
						letter_grid_space,
					)
					.with_spacer(letter_grid_space),
			),
			0.45,
		)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(restart_btn(String::from("Restart")), letter_grid_space)
				.with_spacer(letter_grid_space),
			0.45,
		)
		.with_spacer(letter_grid_space)
}
