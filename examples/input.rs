use barely::prelude::*;

fn main() {
    let mut app = App::<()>::default();

    app.fixed_update(fixed_update);

    app.run();
}

fn fixed_update(_: &mut (), ctx: &FixedContext) {
    if ctx.input().just_pressed(KeyCode::Space) {
        info!("Jump!");
    }

    if ctx.input().just_released(KeyCode::Space) {
        info!("Stop jump");
    }
}
