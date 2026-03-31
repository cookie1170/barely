use barely::prelude::*;

const VERTICES: [Vertex; 4] = [
    Vertex {
        position: Vec3::new(0.25, -0.5, 0.0),
        color: Color::rgb(60, 186, 75),
    },
    Vertex {
        position: Vec3::new(0.25, 0.5, 0.0),
        color: Color::rgb(147, 255, 65),
    },
    Vertex {
        position: Vec3::new(-0.25, 0.5, 0.0),
        color: Color::rgb(147, 255, 65),
    },
    Vertex {
        position: Vec3::new(-0.25, -0.5, 0.0),
        color: Color::rgb(60, 186, 75),
    },
];
const INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

#[derive(Debug, PartialEq)]
struct State {
    material: MaterialHandle<WaveMaterial>,
    vertices: SliceBuffer<Vertex>,
    indices: SliceBuffer<u32>,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct WaveMaterial {
    // WebGL2 requires uniforms to be 16-byte aligned
    #[cfg(target_arch = "wasm32")]
    _webgl_padding: Vec3,
    time: f32,
}

fn main() {
    Shader::register_wesl_source(
        "package::wave".parse().unwrap(),
        include_str!("shaders/wave.wgsl"),
    );

    let mut app = App::<State>::new(init_state);

    app.title("Material uniform");
    app.update(update);

    app.run();
}

fn update(state: &mut State, ctx: &mut Context) {
    ctx.clear_screen(Color::rgb(14, 26, 37));

    state.material.inner.time += ctx.delta_secs();
    ctx.update_material_handle(&mut state.material);
    ctx.draw_vertices(&state.vertices, &state.indices, &state.material);
}

fn init_state(ctx: &mut Context) -> State {
    State {
        material: ctx.create_material_handle(WaveMaterial::default()),
        vertices: ctx.create_slice_buffer(VERTICES, BufferUsages::VERTEX),
        indices: ctx.create_slice_buffer(INDICES, wgpu::BufferUsages::INDEX),
    }
}

impl Material for WaveMaterial {
    fn shader() -> Shader {
        Shader::wesl(&"package::wave".parse().unwrap())
    }
}
