use barely::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct State {
    material: MaterialHandle<CoolMaterial>,
    vertex_buffer: VecBuffer<Vertex>,
    moving_vertices: Vec<MovingVertex>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct MovingVertex {
    position: [f32; 2],
    velocity: [f32; 2],
    color: Color,
    index: usize,
}

fn main() {
    let mut app = App::<State>::new(init_state);

    app.graphics_config(barely::graphics::handle::GraphicsConfig {
        vsync: false,
        ..Default::default()
    });
    info!("start");
    app.title("Vertex buffer");
    app.update(update);
    app.fixed_update(fixed_update);

    app.run();
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct CoolMaterial;

impl Material for CoolMaterial {
    fn shader() -> Shader {
        Shader::wgsl(include_str!("shaders/vertex_buffer.wgsl"))
    }
}

fn init_state(ctx: &mut Context) -> State {
    let material = ctx.create_material_handle(CoolMaterial);
    let vertex_buffer =
        ctx.create_vec_buffer(vec![Vertex::default(); 3], wgpu::BufferUsages::VERTEX);
    let moving_vertices = vec![
        MovingVertex {
            position: [0.0, 0.5],
            velocity: [1.0, 1.0],
            color: Color::rgb(255, 0, 0),
            index: 0,
        },
        MovingVertex {
            position: [-0.5, -0.5],
            velocity: [-1.0, -1.0],
            color: Color::rgb(0, 255, 0),
            index: 1,
        },
        MovingVertex {
            position: [0.5, -0.5],
            velocity: [1.0, -1.0],
            color: Color::rgb(0, 0, 255),
            index: 2,
        },
    ];

    State {
        material,
        vertex_buffer,
        moving_vertices,
    }
}

fn fixed_update(state: &mut State, ctx: &FixedContext) {
    for vertex in state.moving_vertices.iter_mut() {
        vertex.position[0] += vertex.velocity[0] * ctx.delta_secs();
        vertex.position[1] += vertex.velocity[1] * ctx.delta_secs();

        if vertex.position[0] > 1.0 {
            vertex.velocity[0] *= -1.0;
        }

        if vertex.position[0] < -1.0 {
            vertex.velocity[0] *= -1.0;
        }

        if vertex.position[1] > 1.0 {
            vertex.velocity[1] *= -1.0;
        }

        if vertex.position[1] < -1.0 {
            vertex.velocity[1] *= -1.0;
        }
    }

    for moving_vertex in state.moving_vertices.iter() {
        let vertex = &mut state.vertex_buffer.items[moving_vertex.index];
        vertex.position[0] = moving_vertex.position[0];
        vertex.position[1] = moving_vertex.position[1];
        vertex.color = moving_vertex.color;
    }
}

fn update(state: &mut State, ctx: &mut Context) {
    ctx.clear_screen(Color::rgb(14, 26, 37));
    ctx.draw_vertex_buffer_unindexed(&state.vertex_buffer, &state.material);
}
