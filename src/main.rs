extern crate tuix;
use tuix::*;

fn main() {
	let app = Application::new(
		WindowDescription::new()
			.with_title("Counter")
			.with_inner_size(400, 100),
		|state, window| {
			println!("hello world");
		},
	);

	app.run();
}
