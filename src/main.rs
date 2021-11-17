use orbtk::prelude::*;

fn main() {
	Application::new()
		.window(|ctx| {
			Window::new()
				.title("Dom Simon Awesome Sauce")
				.position((100.0, 100.0))
				.size(420.0, 730.0)
                .resizeable(true)
				.child(TextBlock::new().text("OrbTk").build(ctx))
                .child(Button::new().text("goodbye").attach(Grid::column(6)).min_size(48.0, 48).build(ctx))
				.build(ctx)
		})
		.run();
}
