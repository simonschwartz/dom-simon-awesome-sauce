use druid::widget::{CrossAxisAlignment, Either, Flex, Label, Painter};
use druid::{
	theme, AppLauncher, Color, Data, FontDescriptor, FontFamily, Lens, LocalizedString,
	RenderContext, Widget, WidgetExt, WindowDesc,
};
use rand::seq::SliceRandom;

mod db;
mod layout;

#[derive(Debug, Copy, Clone, Data, PartialEq)]
enum LetterState {
	Empty,
	Set,
	Input,
	NotFound,
	WrongSpot,
	Correct,
}

#[derive(Debug, PartialEq, Data, Clone)]
pub enum GameState {
	InProgress,
	Lost,
	Won,
}

type Guess = [(char, LetterState); 5];

#[derive(Debug, Clone, Data, Lens)]
pub struct WobbleState {
	guesses: [Guess; 6],
	word: String,
	history: String,
	finished: GameState,
	msg: String,
}

impl Default for WobbleState {
	fn default() -> Self {
		let mut guesses = [[(' ', LetterState::Empty); 5]; 6];
		guesses[0][0].1 = LetterState::Input;

		WobbleState {
			guesses,
			word: String::from(*db::ANSWERS.choose(&mut rand::thread_rng()).unwrap()),
			history: String::from(""),
			finished: GameState::InProgress,
			msg: String::new(),
		}
	}
}

impl WobbleState {
	fn get_active_coordinates(&self) -> (usize, usize, usize, usize) {
		// defaulting to the end of both row and column here feels safer than going to the top and possibly overriding existing values?
		let mut guess_word = 5;
		let mut guess_letter = 4;

		'outer: for (guess_word_i, word) in self.guesses.iter().enumerate() {
			for (guess_letter_i, (_, letter_state)) in word.iter().enumerate() {
				if *letter_state == LetterState::Input {
					guess_word = guess_word_i;
					guess_letter = guess_letter_i;
					break 'outer;
				}
			}
		}

		// calculate step forward
		let next_guess_letter = if guess_letter == 4 {
			4
		} else {
			guess_letter + 1
		};

		// calculate step backwards
		let prev_guess_letter = if guess_letter == 0 {
			0
		} else if guess_letter == 4 && self.guesses[guess_word][guess_letter].0 != ' ' {
			4
		} else {
			guess_letter - 1
		};

		(
			guess_word,
			guess_letter,
			prev_guess_letter,
			next_guess_letter,
		)
	}

	fn has_full_guess(&self) -> bool {
		let (guess_word, _, _, _) = self.get_active_coordinates();
		if self.guesses[guess_word][0].0 != ' '
			&& self.guesses[guess_word][1].0 != ' '
			&& self.guesses[guess_word][2].0 != ' '
			&& self.guesses[guess_word][3].0 != ' '
			&& self.guesses[guess_word][4].0 != ' '
		{
			true
		} else {
			false
		}
	}

	fn has_at_least_one_guess(&self) -> bool {
		let (guess_word, _, _, _) = self.get_active_coordinates();
		if self.guesses[guess_word][0].0 != ' ' {
			true
		} else {
			false
		}
	}

	fn has_char_in_history(&self, letter: &char) -> bool {
		self.history.contains(*letter)
	}

	fn process_guess(&mut self) {
		if self.has_full_guess() {
			let (guess_word, _, _, _) = self.get_active_coordinates();
			let guess = format!(
				"{}{}{}{}{}",
				self.guesses[guess_word][0].0,
				self.guesses[guess_word][1].0,
				self.guesses[guess_word][2].0,
				self.guesses[guess_word][3].0,
				self.guesses[guess_word][4].0
			);
			if !db::ALLOWED_ANSWERS.contains(&guess.as_ref())
				&& !db::ANSWERS.contains(&guess.as_ref())
			{
				self.msg = String::from("This word was not found in our word list");
			} else {
				self.msg = String::from("");
				self.history.push(self.guesses[guess_word][0].0);
				self.history.push(self.guesses[guess_word][1].0);
				self.history.push(self.guesses[guess_word][2].0);
				self.history.push(self.guesses[guess_word][3].0);
				self.history.push(self.guesses[guess_word][4].0);

				for i in 0..=4 {
					let letter = self.guesses[guess_word][i].0;
					let word_vec = self.word.chars().collect::<Vec<char>>();

					self.guesses[guess_word][i].1 = if letter == word_vec[i] {
						LetterState::Correct
					} else if self.word.contains(letter) {
						LetterState::WrongSpot
					} else {
						LetterState::NotFound
					};
				}

				if guess == self.word {
					self.finished = GameState::Won;
				} else if guess_word < 5 {
					self.guesses[guess_word + 1][0].1 = LetterState::Input;
				} else {
					self.finished = GameState::Lost;
				}
			}
		}
	}

	fn restart(&mut self) {
		*self = WobbleState::default();
	}
}

#[test]
fn get_active_coordinates_test() {
	let mut state = WobbleState {
		guesses: [[(' ', LetterState::Empty); 5]; 6],
		word: String::from(*db::ANSWERS.choose(&mut rand::thread_rng()).unwrap()),
		history: String::from(""),
		finished: GameState::InProgress,
		msg: String::new(),
	};
	state.guesses[0][3] = (' ', LetterState::Input);
	assert_eq!(state.get_active_coordinates(), (0, 3, 2, 4));

	state = WobbleState {
		guesses: [[(' ', LetterState::Empty); 5]; 6],
		word: String::from(*db::ANSWERS.choose(&mut rand::thread_rng()).unwrap()),
		history: String::from(""),
		finished: GameState::InProgress,
		msg: String::new(),
	};
	state.guesses[1][2] = (' ', LetterState::Input);
	assert_eq!(state.get_active_coordinates(), (1, 2, 1, 3));

	// edge cases
	state = WobbleState {
		guesses: [[(' ', LetterState::Empty); 5]; 6],
		word: String::from(*db::ANSWERS.choose(&mut rand::thread_rng()).unwrap()),
		history: String::from(""),
		finished: GameState::InProgress,
		msg: String::new(),
	};
	state.guesses[0][0] = (' ', LetterState::Input);
	assert_eq!(state.get_active_coordinates(), (0, 0, 0, 1));

	state = WobbleState {
		guesses: [[(' ', LetterState::Empty); 5]; 6],
		word: String::from(*db::ANSWERS.choose(&mut rand::thread_rng()).unwrap()),
		history: String::from(""),
		finished: GameState::InProgress,
		msg: String::new(),
	};
	state.guesses[5][4] = (' ', LetterState::Input);
	assert_eq!(state.get_active_coordinates(), (5, 4, 3, 4));

	// what if we get a bad state passed in?
	state = WobbleState {
		guesses: [[(' ', LetterState::Empty); 5]; 6],
		word: String::from(*db::ANSWERS.choose(&mut rand::thread_rng()).unwrap()),
		history: String::from(""),
		finished: GameState::InProgress,
		msg: String::new(),
	};
	assert_eq!(state.get_active_coordinates(), (5, 4, 3, 4));
}

pub fn keyboard_btn(letter: char) -> impl Widget<WobbleState> {
	let rect = Painter::new(move |ctx, data: &WobbleState, _| {
		let is_active = !data.has_char_in_history(&letter);
		// TODO: mark LetterState in keyboard button with colors

		let bounds = ctx.size().to_rect().to_rounded_rect(4.0);

		if is_active {
			ctx.fill(bounds, &Color::rgba8(0xFF, 0xFF, 0xFF, 0xFF));
		} else {
			ctx.fill(bounds, &Color::rgba8(0xDD, 0xDD, 0xDD, 0x99));
		}
		ctx.stroke(bounds, &Color::rgba8(0x00, 0x00, 0x00, 0x55), 1.0);

		if ctx.is_hot() && is_active {
			ctx.fill(bounds, &Color::rgba8(0xEE, 0xEE, 0xEE, 0xFF));
		}

		if ctx.is_active() && is_active {
			ctx.fill(bounds, &Color::rgba8(0x00, 0xFF, 0x00, 0xFF));
		}
	});

	Label::new(format!("{}", letter.to_uppercase()))
		.with_text_size(24.0)
		.center()
		.background(rect)
		.expand()
		.on_click(move |_ctx, data: &mut WobbleState, _| {
			let (guess_word, guess_letter, _, next_guess_letter) = data.get_active_coordinates();

			data.guesses[guess_word][guess_letter].0 = letter;
			data.guesses[guess_word][guess_letter].1 = LetterState::Set;
			data.guesses[guess_word][next_guess_letter].1 = LetterState::Input;
		})
}

pub fn enter_btn(label: String) -> impl Widget<WobbleState> {
	let rect = Painter::new(|ctx, data: &WobbleState, _| {
		let is_active = data.has_full_guess();

		let bounds = ctx.size().to_rect().to_rounded_rect(2.0);

		if is_active {
			ctx.fill(bounds, &Color::rgba8(0x33, 0xFF, 0x33, 0x99));
		} else {
			ctx.fill(bounds, &Color::rgba8(0xDD, 0xDD, 0xDD, 0x99));
		}

		if ctx.is_hot() && is_active {
			ctx.stroke(bounds, &Color::rgba8(0x00, 0x00, 0x00, 0x44), 1.0);
		}

		if ctx.is_active() && is_active {
			ctx.fill(bounds, &Color::rgba8(0x00, 0xFF, 0x00, 0xFF));
		}
	});

	Label::new(label)
		.with_text_size(24.0)
		.center()
		.background(rect)
		.expand()
		.on_click(move |_ctx, data: &mut WobbleState, _env| {
			data.process_guess();
		})
}

pub fn del_btn(label: String) -> impl Widget<WobbleState> {
	let rect = Painter::new(|ctx, data: &WobbleState, _| {
		let is_active = data.has_at_least_one_guess();

		let bounds = ctx.size().to_rect().to_rounded_rect(2.0);

		if is_active {
			ctx.fill(bounds, &Color::rgba8(0x33, 0xFF, 0x33, 0x99));
		} else {
			ctx.fill(bounds, &Color::rgba8(0xDD, 0xDD, 0xDD, 0x99));
		}

		if ctx.is_hot() && is_active {
			ctx.stroke(bounds, &Color::rgba8(0x00, 0x00, 0x00, 0x44), 1.0);
		}

		if ctx.is_active() && is_active {
			ctx.fill(bounds, &Color::rgba8(0x00, 0xFF, 0x00, 0xFF));
		}
	});

	Label::new(label)
		.with_text_size(24.0)
		.center()
		.background(rect)
		.expand()
		.on_click(move |_ctx, data: &mut WobbleState, _env| {
			let (guess_word, guess_letter, _, _) = data.get_active_coordinates();
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

fn restart_btn(label: String) -> impl Widget<WobbleState> {
	let rect = Painter::new(|ctx, _, _| {
		let bounds = ctx.size().to_rect().to_rounded_rect(2.0);

		ctx.fill(bounds, &Color::rgba8(0x33, 0xFF, 0x33, 0x99));

		if ctx.is_hot() {
			ctx.stroke(bounds, &Color::rgba8(0x00, 0x00, 0x00, 0x44), 1.0);
		}

		if ctx.is_active() {
			ctx.fill(bounds, &Color::rgba8(0x00, 0xFF, 0x00, 0xFF));
		}
	});

	Label::new(label)
		.with_text_size(24.0)
		.center()
		.background(rect)
		.expand()
		.on_click(|_ctx, data: &mut WobbleState, _env| {
			data.restart();
		})
}

pub fn letter_box(guess_word: usize, guess_letter: usize) -> impl Widget<WobbleState> {
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
	let letter_space = 0.5;
	let letter_grid_space = 5.0;
	let key_space = 0.3;

	let popover = Label::new(move |data: &WobbleState, _env: &_| data.msg.clone())
		.with_text_color(Color::RED)
		.center()
		.padding(10.0);

	let view_switcher = Either::new(
		|data, _env| data.finished != GameState::InProgress,
		layout::end_screen(letter_grid_space),
		layout::keyboard(letter_grid_space, key_space),
	);

	Flex::column()
		.with_child(popover)
		// .with_flex_spacer(0.05)
		.with_child(layout::heading())
		.with_flex_spacer(0.2)
		.cross_axis_alignment(CrossAxisAlignment::End)
		.with_flex_child(
			layout::guess_tiles(letter_grid_space, letter_space),
			letter_space * 6.0,
		)
		.with_flex_spacer(0.2)
		.with_flex_child(view_switcher, 0.9)
}

pub fn main() {
	let window = WindowDesc::new(build_layout)
		.window_size((500., 800.))
		.resizable(true)
		.title(LocalizedString::new("app-title").with_placeholder("wobble"));

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
		.launch(WobbleState::default())
		.expect("launch failed");
}
