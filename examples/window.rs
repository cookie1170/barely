use barely::prelude::*;

fn main() {
    let mut app = App::<()>::default();

    app.title("Hello there");
    app.update(update);

    app.run()
}

fn update(_: &mut (), ctx: &mut Context) {
    ctx.clear_screen(Color::rgb(14, 26, 37));
}
