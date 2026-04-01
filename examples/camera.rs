use barely::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct State {
    vertex_buffer: SliceBuffer<Vertex>,
    material: MaterialHandle<DefaultMaterial>,
}

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: Vec3::new(0.5, -0.5, 0.0),
        color: Color::rgb(255, 0, 0),
    },
    Vertex {
        position: Vec3::new(-0.5, -0.5, 0.0),
        color: Color::rgb(0, 255, 0),
    },
    Vertex {
        position: Vec3::new(0.0, 0.5, 0.0),
        color: Color::rgb(0, 0, 255),
    },
];

fn main() {
    let mut app = App::<State>::new(init_state);

    app.title("Camera");
    app.update(update);

    app.run();
}

fn update(state: &mut State, ctx: &mut Context) {
    ctx.clear_screen(Color::rgb(14, 26, 37));
    ctx.set_camera(PerspectiveCamera::new(
        ctx.screen_size(),
        Vec3::Z,
        Vec3::NEG_Z,
    ));
    ctx.draw_vertices_unindexed(&state.vertex_buffer, &state.material);
}

fn init_state(ctx: &mut Context) -> State {
    State {
        vertex_buffer: ctx.create_slice_buffer(VERTICES, BufferUsages::VERTEX),
        material: ctx.create_material_handle(DefaultMaterial::default()),
    }
}
