use glium::{glutin, implement_vertex, uniform, Surface};
mod sync_data;
// mod sync_data;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &sync_data::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &sync_data::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &sync_data::INDICES,
    )
    .unwrap();

    let robot = glium::VertexBuffer::new(&display, &sync_data::ROBOT).expect("no robot found");
    //let path = glium::VertexBuffer::new(&display, &sync_data::PATHS).expect("no paths found");
    let indices2 = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &sync_data::INDICES2,
    )
    .unwrap();
    let normals2 = glium::VertexBuffer::new(&display, &sync_data::NORMALS2).unwrap();

    let path = [
        [0.5, 0.2, 0.0],
        [0.2, 0.01, 0.0],
        [0.4, 0.15, 0.01],
        [0.3, 0.2, 0.0f32],
    ];

    let vertex_shader_src = r#"
        #version 150
        in vec3 position;
        in vec3 normal;
        out vec3 v_normal;
        uniform mat4 matrix;
        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let vertex_shader_src2 = r#"

        #version 150
        in vec3 position;
        in vec3 normal;
        out vec3 v_normal;
        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix  * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src2 = r#"
        #version 150
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let program2 =
        glium::Program::from_source(&display, vertex_shader_src2, fragment_shader_src2, None)
            .unwrap();

    let mut path_index = 0;

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(166_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];
        if path_index >= path.len()-1 {
            path_index = 0;
        }else{
            path_index += 1;
        }
        
        let t = path[path_index];

        let matrix2 = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [t[0], t[1], t[2], 1.0f32],
        ];

        let light = [-1.0, 0.4, 0.9f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! { matrix: matrix, u_light: light},
                &params,
            )
            .unwrap();

        target
            .draw(
                (&robot, &normals2),
                &indices2,
                &program2,
                &uniform! { matrix: matrix2, u_light: light},
                &params,
            )
            .unwrap();

        target.finish().unwrap();
    });
}
