use barely::prelude::*;

#[derive(Debug, PartialEq)]
struct State {
    material: MaterialHandle<CoolMaterial>,
    vertex_buffer: VecBuffer<Vertex>,
    moving_vertices: Vec<MovingVertex>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct MovingVertex {
    position: Vec2,
    velocity: Vec2,
    color: Color,
    index: usize,
}

fn main() {
    let mut app = App::<State>::new(init_state);

    app.title("Vertex buffer");
    app.update(update);
    app.fixed_update(fixed_update);

    app.run();
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CoolMaterial;

impl Material for CoolMaterial {
    fn shader() -> Shader {
        Shader::wgsl(include_str!("shaders/vertex_buffer.wgsl"))
    }
}

fn init_state(ctx: &mut Context) -> State {
    let material = ctx.create_material_handle(CoolMaterial);
    let vertex_buffer = ctx.create_vec_buffer(vec![Vertex::default(); 3], BufferUsages::VERTEX);
    let moving_vertices = vec![
        MovingVertex {
            position: Vec2::new(0.0, 0.5),
            velocity: Vec2::new(1.0, 1.0),
            color: Color::rgb(255, 0, 0),
            index: 0,
        },
        MovingVertex {
            position: Vec2::new(-0.5, -0.5),
            velocity: Vec2::new(-1.0, -1.0),
            color: Color::rgb(0, 255, 0),
            index: 1,
        },
        MovingVertex {
            position: Vec2::new(0.5, -0.5),
            velocity: Vec2::new(1.0, -1.0),
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
        vertex.position += vertex.velocity * ctx.delta_secs();

        if vertex.position.x > 1.0 {
            vertex.velocity.x *= -1.0;
            vertex.position.x = 0.99;
        }

        if vertex.position.x < -1.0 {
            vertex.velocity.x *= -1.0;
            vertex.position.x = -0.99;
        }

        if vertex.position.y > 1.0 {
            vertex.velocity.y *= -1.0;
            vertex.position.y = 0.99;
        }

        if vertex.position.y < -1.0 {
            vertex.velocity.y *= -1.0;
            vertex.position.y = -0.99;
        }
    }

    for moving_vertex in state.moving_vertices.iter() {
        let vertex = &mut state.vertex_buffer.items[moving_vertex.index];
        vertex.position = moving_vertex.position.extend(0.0);
        vertex.color = moving_vertex.color;
    }
}

fn update(state: &mut State, ctx: &mut Context) {
    ctx.clear_screen(Color::rgb(14, 26, 37));
    ctx.draw_vertices_unindexed(&state.vertex_buffer, &state.material);
}
