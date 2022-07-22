use druid::{
	theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
	FontDescriptor, WindowDesc,
};
use druid::widget::{CrossAxisAlignment, Flex, Label, Painter};

#[derive(Debug, Clone, Data, PartialEq)]
enum LetterState {
	Empty,
	Input,
	NotFound,
	WrongSpot,
	Correct,
}

#[derive(Debug, Clone, Data, Lens)]
struct WobbleState {
	line1: (
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
	),
	line2: (
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
	),
	line3: (
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
	),
	line4: (
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
	),
	line5: (
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
	),
	line6: (
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
		(char, LetterState),
	),
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

fn letter_key(letter: char) -> impl Widget<WobbleState> {
	let rect = Painter::new(|ctx, _, _| {
		let bounds = ctx.size().to_rect();

		ctx.fill(bounds, &Color::rgba8(0xFF, 0xFF, 0xFF, 0xFF));

		if ctx.is_hot() {
			ctx.stroke(bounds.inset(-0.5), &Color::rgba8(0x00, 0x00, 0x00, 0x44), 1.0);
		}

		if ctx.is_active() {
			ctx.fill(bounds, &Color::rgba8(0x00, 0x71, 0x00, 0x88));
		}
	});

	Label::new(format!("{}", letter))
		.with_text_size(24.0)
		.center()
		.background(rect)
		.expand()
		.on_click(move |_ctx, data: &mut WobbleState, _env| {})
}

fn button(label: String) -> impl Widget<WobbleState> {
	let rect = Painter::new(|ctx, _, _| {
		let bounds = ctx.size().to_rect();

		ctx.fill(bounds, &Color::rgba8(0xFF, 0xFF, 0xFF, 0xFF));

		if ctx.is_hot() {
			ctx.stroke(bounds.inset(-0.5), &Color::rgba8(0x00, 0x00, 0x00, 0x44), 1.0);
		}

		if ctx.is_active() {
			ctx.fill(bounds, &Color::rgb8(0x00, 0x71, 0x00));
		}
	});

	Label::new(format!("{}", label))
		.with_text_size(24.0)
		.center()
		.background(rect)
		.expand()
		.on_click(move |_ctx, data: &mut WobbleState, _env| {})
}

fn letter_box(label: String) -> impl Widget<WobbleState> {
	let rect = Painter::new(|ctx, _, _| {
		let bounds = ctx.size().to_rect();
		ctx.fill(bounds, &Color::rgba8(0xFF, 0xFF, 0xFF, 0xFF));
		ctx.stroke(bounds, &Color::rgba8(0x00, 0x00, 0x00, 0x22), 1.0);
	});

	Label::new(format!("{}", label))
		.with_text_size(30.0)
		.center()
		.background(rect)
		.expand()
}

fn build_layout() -> impl Widget<WobbleState> {
	let heading = Label::new("wobble")
		.with_text_size(32.0)
		.with_text_color(Color::rgb8(0xFF, 0xFF, 0xFF))
		.center()
		.padding(5.0);

	let letter_space = 1.0;
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
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space),
				letter_space
		)
		.with_spacer(letter_grid_space)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space),
				letter_space
		)
		.with_spacer(letter_grid_space)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space),
				letter_space
		)
		.with_spacer(letter_grid_space)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space),
				letter_space
		)
		.with_spacer(letter_grid_space)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space),
				letter_space
		)
		.with_spacer(letter_grid_space)
		.with_flex_child(
			Flex::row()
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space)
				.with_flex_child(letter_box(" ".to_string()), letter_space)
				.with_spacer(letter_grid_space),
				letter_space
		)
		.with_flex_spacer(0.2)
		.with_flex_child(
			Flex::row()
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
				.with_flex_child(letter_key('p'), key_space),
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
				.with_flex_child(button("Enter".to_string()), 1.0)
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
				.with_flex_child(button("del".to_string()), 1.0),
			0.3,
		)
}

pub fn main() {
	let window = WindowDesc::new(build_layout)
		.window_size((500., 800.))
		.resizable(true)
		.title(
			LocalizedString::new("app-title").with_placeholder("wobble"),
		);

	let calc_state = WobbleState {
		line1: (
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
		),
		line2: (
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
		),
		line3: (
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
		),
		line4: (
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
		),
		line5: (
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
		),
		line6: (
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
			(' ', LetterState::Empty),
		),
	};

	AppLauncher::with_window(window)
		.use_simple_logger()
		// .configure_env(|env, _| {
		// 	env.set(theme::UI_FONT, FontDescriptor::default().with_size(12.0));
		// 	env.set(theme::LABEL_COLOR, Color::rgba8(0x00, 0x00, 0x00, 0xFF));
		// 	env.set(druid::theme::BACKGROUND_LIGHT, Color::WHITE);
		// 	env.set(druid::theme::BACKGROUND_DARK, Color::WHITE);
		// })
		.launch(calc_state)
		.expect("launch failed");
}
