use druid::widget::{CrossAxisAlignment, Flex, Label, Painter};
use druid::{
	theme, AppLauncher, Color, Data, FontDescriptor, FontFamily, Lens, LocalizedString,
	RenderContext, Widget, WidgetExt, WindowDesc,
};

#[derive(Debug, Clone, Data, PartialEq)]
enum LetterState {
	Empty,
	Set,
	Input,
	NotFound,
	WrongSpot,
	Correct,
}

type Guess = [(char, LetterState); 5];

#[derive(Debug, Clone, Data, Lens)]
struct WobbleState {
	guesses: [Guess; 6],
}

// #[derive(Clone, Data, Lens)]
// struct CalcState {
// 	/// The number displayed. Generally a valid float.
// 	value: String,
// 	operand: f64,
// 	operator: char,
// 	in_num: bool,
// }

// impl CalcState {
// 	fn digit(&mut self, digit: u8) {
// 		if !self.in_num {
// 			self.value.clear();
// 			self.in_num = true;
// 		}
// 		let ch = (b'0' + digit) as char;
// 		self.value.push(ch);
// 	}

// 	fn display(&mut self) {
// 		// TODO: change hyphen-minus to actual minus
// 		self.value = self.operand.to_string();
// 	}

// 	fn compute(&mut self) {
// 		if self.in_num {
// 			let operand2 = self.value.parse().unwrap_or(0.0);
// 			let result = match self.operator {
// 				'+' => Some(self.operand + operand2),
// 				'−' => Some(self.operand - operand2),
// 				'×' => Some(self.operand * operand2),
// 				'÷' => Some(self.operand / operand2),
// 				_ => None,
// 			};
// 			if let Some(result) = result {
// 				self.operand = result;
// 				self.display();
// 				self.in_num = false;
// 			}
// 		}
// 	}

// 	fn op(&mut self, op: char) {
// 		match op {
// 			'+' | '−' | '×' | '÷' | '=' => {
// 				self.compute();
// 				self.operand = self.value.parse().unwrap_or(0.0);
// 				self.operator = op;
// 				self.in_num = false;
// 			}
// 			'±' => {
// 				if self.in_num {
// 					if self.value.starts_with('−') {
// 						self.value = self.value[3..].to_string();
// 					} else {
// 						self.value = ["−", &self.value].concat();
// 					}
// 				} else {
// 					self.operand = -self.operand;
// 					self.display();
// 				}
// 			}
// 			'.' => {
// 				if !self.in_num {
// 					self.value = "0".to_string();
// 					self.in_num = true;
// 				}
// 				if self.value.find('.').is_none() {
// 					self.value.push('.');
// 				}
// 			}
// 			'q' => {
// 				self.value = "0".to_string();
// 				self.in_num = false;
// 			}
// 			'C' => {
// 				self.value = "0".to_string();
// 				self.operator = 'C';
// 				self.in_num = false;
// 			}
// 			'⌫' => {
// 				if self.in_num {
// 					self.value.pop();
// 					if self.value.is_empty() || self.value == "−" {
// 						self.value = "0".to_string();
// 						self.in_num = false;
// 					}
// 				}
// 			}
// 			_ => unreachable!(),
// 		}
// 	}
// }

fn get_current_position(guesses: &[Guess; 6]) -> (usize, usize) {
	// defaulting to the end of both row and column here feels safer than going to the top and possibly overriding existing values?
	let mut guess_word = 5;
	let mut guess_letter = 4;

	'outer: for (guess_worda, word) in guesses.iter().enumerate() {
		for (guess_lettera, (_, letter_state)) in word.iter().enumerate() {
			if *letter_state == LetterState::Input {
				guess_word = guess_worda;
				guess_letter = guess_lettera;
				break 'outer;
			}
		}
	}

	(guess_word, guess_letter)
}

#[test]
fn get_current_position_test() {
	// created an empty default state for our data
	let fresh_state = [[(' ', LetterState::Empty); 5]; 6];

	// now for each test we clone the fresh state from above and tweak it to our liking
	let mut state = fresh_state.clone();
	state[0][3] = (' ', LetterState::Input);
	assert_eq!(get_current_position(&state), (0, 3));

	state = fresh_state.clone();
	state[1][2] = (' ', LetterState::Input);
	assert_eq!(get_current_position(&state), (1, 2));

	// edge cases
	state = fresh_state.clone();
	state[0][0] = (' ', LetterState::Input);
	assert_eq!(get_current_position(&state), (0, 0));

	state = fresh_state.clone();
	state[5][4] = (' ', LetterState::Input);
	assert_eq!(get_current_position(&state), (5, 4));

	// what if we get a bad state passed in?
	state = fresh_state.clone();
	assert_eq!(get_current_position(&state), (5, 4));
}

fn letter_key(letter: char) -> impl Widget<WobbleState> {
	let rect = Painter::new(|ctx, _, _| {
		let bounds = ctx.size().to_rect().to_rounded_rect(4.0);

		ctx.fill(bounds, &Color::rgba8(0xFF, 0xFF, 0xFF, 0xFF));
		ctx.stroke(bounds, &Color::rgba8(0x00, 0x00, 0x00, 0x55), 1.0);

		if ctx.is_hot() {
			ctx.fill(bounds, &Color::rgba8(0xEE, 0xEE, 0xEE, 0xFF));
		}

		if ctx.is_active() {
			ctx.fill(bounds, &Color::rgba8(0x00, 0xFF, 0x00, 0xFF));
		}
	});

	Label::new(format!("{}", letter.to_uppercase()))
		.with_text_size(24.0)
		.center()
		.background(rect)
		.expand()
		.on_click(move |_ctx, data: &mut WobbleState, _| {
			// call function to analyze data.guesses and return (guess_word,guess_letter) coordinates
			let (guess_word, guess_letter) = get_current_position(&data.guesses);
			let next_guess_letter = if guess_letter == 4 {
				4
			} else {
				guess_letter + 1
			};
			data.guesses[guess_word][guess_letter].0 = letter;
			data.guesses[guess_word][guess_letter].1 = LetterState::Set;
			data.guesses[guess_word][next_guess_letter].1 = LetterState::Input;
		})
}

fn enter_key(label: String) -> impl Widget<WobbleState> {
	let rect = Painter::new(|ctx, _, _| {
		let bounds = ctx.size().to_rect().to_rounded_rect(2.0);

		ctx.fill(bounds, &Color::rgba8(0xAA, 0xAA, 0xAA, 0x99));

		if ctx.is_hot() {
			ctx.stroke(bounds, &Color::rgba8(0x00, 0x00, 0x00, 0x44), 1.0);
		}

		if ctx.is_active() {
			ctx.fill(bounds, &Color::rgba8(0x00, 0xFF, 0x00, 0xFF));
		}
	});

	Label::new(format!("{}", label))
		.with_text_size(24.0)
		.center()
		.background(rect)
		.expand()
		.on_click(move |_ctx, _data: &mut WobbleState, _env| {})
}

fn del_key(label: String) -> impl Widget<WobbleState> {
	let rect = Painter::new(|ctx, _, _| {
		let bounds = ctx.size().to_rect().to_rounded_rect(2.0);

		ctx.fill(bounds, &Color::rgba8(0xAA, 0xAA, 0xAA, 0x99));

		if ctx.is_hot() {
			ctx.stroke(bounds, &Color::rgba8(0x00, 0x00, 0x00, 0x44), 1.0);
		}

		if ctx.is_active() {
			ctx.fill(bounds, &Color::rgba8(0x00, 0xFF, 0x00, 0xFF));
		}
	});

	Label::new(format!("{}", label))
		.with_text_size(24.0)
		.center()
		.background(rect)
		.expand()
		.on_click(move |_ctx, data: &mut WobbleState, _env| {
			let (guess_word, guess_letter) = get_current_position(&data.guesses);
			let next_guess_letter = if guess_letter == 0 {
				0
			} else if guess_letter == 4 && data.guesses[guess_word][guess_letter].0 != ' ' {
				4
			} else {
				guess_letter - 1
			};
			data.guesses[guess_word][guess_letter].0 = ' ';
			data.guesses[guess_word][guess_letter].1 = LetterState::Empty;
			data.guesses[guess_word][next_guess_letter].0 = ' ';
			data.guesses[guess_word][next_guess_letter].1 = LetterState::Input;
		})
}

fn letter_box(guess_word: usize, guess_letter: usize) -> impl Widget<WobbleState> {
	let rect = Painter::new(move |ctx, guesses: &[Guess; 6], _| {
		let bounds = ctx.size().to_rect();
		let (bg, color) = match &guesses[guess_word][guess_letter].1 {
			LetterState::Empty => (
				Color::rgba8(0xFF, 0xFF, 0xFF, 0xFF),
				Color::rgba8(0x00, 0x00, 0x00, 0x33),
			),
			LetterState::Set => (
				Color::rgba8(0xFF, 0xFF, 0xFF, 0xFF),
				Color::rgba8(0x00, 0x00, 0x00, 0x33),
			),
			LetterState::Input => (Color::rgba8(0xFF, 0xFF, 0xFF, 0xFF), Color::BLACK),
			LetterState::NotFound => (
				Color::rgba8(0xAA, 0xAA, 0xAA, 0x55),
				Color::rgba8(0x00, 0x00, 0x00, 0x33),
			),
			LetterState::WrongSpot => (
				Color::rgba8(0xFF, 0xFF, 0x00, 0xFF),
				Color::rgba8(0x00, 0x00, 0x00, 0x33),
			),
			LetterState::Correct => (
				Color::rgba8(0x00, 0xFF, 0x00, 0xFF),
				Color::rgba8(0x00, 0x00, 0x00, 0x33),
			),
		};
		ctx.fill(bounds, &bg);
		ctx.stroke(bounds, &color, 2.5);
	});

	Label::new(move |guesses: &[Guess; 6], _env: &_| {
		format!(
			"{}",
			guesses[guess_word][guess_letter].0.clone().to_uppercase()
		)
	})
	.with_text_size(30.0)
	.with_text_color(Color::BLACK)
	.center()
	.background(rect)
	.expand()
	.lens(WobbleState::guesses)
}

fn build_layout() -> impl Widget<WobbleState> {
	let heading = Label::new("wobble")
		.with_font(FontDescriptor::new(FontFamily::SERIF))
		.with_text_size(60.0)
		.with_text_color(Color::BLACK)
		.center()
		.padding(10.0);

	let letter_space = 0.5;
	let letter_grid_space = 5.0;
	let key_space = 0.3;

	Flex::column()
		.with_flex_spacer(0.2)
		.with_child(heading)
		.with_flex_spacer(0.2)
		.cross_axis_alignment(CrossAxisAlignment::End)
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
		.with_flex_spacer(0.2)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_key('q'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('w'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('e'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('r'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('t'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('y'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('u'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('i'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('o'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('p'), key_space)
				.with_spacer(letter_grid_space),
			0.3,
		)
		.with_spacer(1.0)
		.with_flex_child(
			Flex::row()
				.with_flex_spacer(0.2)
				.with_flex_child(letter_key('a'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('s'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('d'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('f'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('g'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('h'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('j'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('k'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('l'), key_space)
				.with_flex_spacer(0.2),
			0.3,
		)
		.with_spacer(1.0)
		.with_flex_child(
			Flex::row()
				.with_flex_child(enter_key("Enter".to_string()), 1.0)
				.with_spacer(1.0)
				.with_flex_child(letter_key('z'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('x'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('c'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('v'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('b'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('n'), key_space)
				.with_spacer(key_space)
				.with_flex_child(letter_key('m'), key_space)
				.with_spacer(1.0)
				.with_flex_child(del_key("del".to_string()), 1.0),
			0.3,
		)
}

pub fn main() {
	let window = WindowDesc::new(build_layout)
		.window_size((500., 800.))
		.resizable(true)
		.title(LocalizedString::new("app-title").with_placeholder("wobble"));

	let calc_state = WobbleState {
		guesses: [
			[
				('w', LetterState::NotFound),
				('o', LetterState::Correct),
				('b', LetterState::WrongSpot),
				('b', LetterState::NotFound),
				('l', LetterState::Correct),
			],
			[
				('D', LetterState::Set),
				(' ', LetterState::Input),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
			],
			[
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
			],
			[
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
			],
			[
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
			],
			[
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
				(' ', LetterState::Empty),
			],
		],
	};

	AppLauncher::with_window(window)
		.use_simple_logger()
		.configure_env(|env, _| {
			env.set(
				theme::UI_FONT,
				FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(12.0),
			);
			env.set(theme::LABEL_COLOR, Color::BLACK);
			env.set(druid::theme::WINDOW_BACKGROUND_COLOR, Color::WHITE);
		})
		.launch(calc_state)
		.expect("launch failed");
}
