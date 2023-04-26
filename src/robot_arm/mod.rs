use glium::uniform;
#[allow(unused_imports)]
use glium::{glutin, Surface};

mod parser_obstacle;
mod sync_data_arm;
const SCALE: f32 = 0.01;

// Just some notes for myself.
// - Since we are using the perspective matrix, I changed the third to last value of the last row of every shader matrix to 2.0 instead of 0.0
fn convert_sampling_cord_to_glium_cords(cord: f32) -> f32 {
    let glium_cord = -100.0 + cord * 25.0f32;
    glium_cord
}

pub fn graph_robot_arm(path: Vec<[f32; 3]>, anchor: [f32; 2], arm_length: f32, display_trace: f32) {
    //set up drawing interface
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let light_souce = [-1.0, 0.4, 0.9f32];
    let arm_width = 0.0;

    // for robot arm vertices, indices, and normals
    let robot_arm = glium::VertexBuffer::new(&display, &sync_data_arm::ROBOT).unwrap();
    let normals = glium::VertexBuffer::new(&display, &sync_data_arm::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &sync_data_arm::ROBOT_INDICES,
    )
    .unwrap();

    // for the stick robot arm, aka a line
    // let robot_arm_stick = glium::VertexBuffer::new(&display, &sync_data_arm::ROBOT_STICK).unwrap();
    // // let normals = glium::VertexBuffer::new(&display, &sync_data_arm::NORMALS).unwrap();
    // let robot_arm_stick_indices = glium::IndexBuffer::new(
    //     &display,
    //     glium::index::PrimitiveType::LineStrip,
    //     &sync_data_arm::ROBOT_STICK_INDICES,
    // )
    // .unwrap();

    // for start/goal end point verices, indices, and normals
    let circular_position = glium::VertexBuffer::new(&display, &sync_data_arm::CIRCLE).unwrap();
    let circular_normals =
        glium::VertexBuffer::new(&display, &sync_data_arm::CIRCLE_NORMALS).unwrap();
    let circular_indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TriangleFan,
        &sync_data_arm::CIRCLE_INDICES,
    )
    .unwrap();

    //for the obstacles vertices, indices, and normals
    let filepath = "src/bin/obstacles.txt";
    let vertices = parser_obstacle::obstacle_parser_glium(&filepath)
        .expect("can't parse in obstacle vertices");
    let connecting_indices =
        parser_obstacle::obstacle_parser_find_connecting_indices(vertices.len() / 4)
            .expect("can't parse in obstacle connecting vertices");

    let obstacle_vertices = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let obstacle_normals = glium::VertexBuffer::new(&display, &sync_data_arm::NORMALS).unwrap();
    let obstacle_indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &connecting_indices,
    )
    .unwrap();

    let vertex_shader_robot_arm = r#"
        #version 150
        in vec3 position;
        in vec3 normal;
        uniform mat4 matrix;
        uniform mat4 matrix2;
        uniform mat4 matrix3;
        uniform mat4 perspective;
        void main() {
            gl_Position = perspective * matrix* matrix2* matrix3* vec4(position, 1.0);
        }
    "#;

    let fragment_shader_robot_arm = r#"
        #version 150
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;
        void main() {
            vec3 regular_color = vec3(0.1, 0.1, 0.1);
            color = vec4(regular_color, 0.9);
        }
    "#;

    let vertex_shader_obstacle = r#"
        #version 150
        in vec3 position;
        in vec3 normal;
        uniform mat4 matrix;
        uniform mat4 perspective;
        void main() {
            gl_Position = perspective * matrix * vec4(position, 1.0);
        }
        "#;

    let fragment_shader_obstacle = r#"
        #version 150
        out vec4 color;
        uniform vec3 u_light;
        void main() {
            vec3 regular_color = vec3(0.2, 0.4, 0.1);
            color = vec4(regular_color, 0.9);
        }
    "#;

    let vertex_shader_goal_circle = r#"
        #version 150
        in vec3 position;
        in vec3 normal;
        out vec3 v_normal;
        uniform mat4 matrix;
        uniform mat4 perspective;
        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = perspective * matrix * vec4(position, 0.3);
        }
    "#;

    let fragment_shader_goal_circle = r#"
        #version 150
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;
        void main() {
            vec3 regular_color = vec3(0.2, 0.4, 0.6);
            color = vec4(regular_color, 0.9);
        }
    "#;

    let program_robot_arm = glium::Program::from_source(
        &display,
        vertex_shader_robot_arm,
        fragment_shader_robot_arm,
        None,
    )
    .unwrap();

    let program_circle = glium::Program::from_source(
        &display,
        vertex_shader_goal_circle,
        fragment_shader_goal_circle,
        None,
    )
    .unwrap();

    let program_obstacle = glium::Program::from_source(
        &display,
        vertex_shader_obstacle,
        fragment_shader_obstacle,
        None,
    )
    .unwrap();

    // let path = vec![[0.0f32,0.0f32,0.9f32], [0.3,0.3,1.2],[0.0,0.0,0.0]];
    let mut path_index = path.len() - 1;
    let anchor_pos = [
        convert_sampling_cord_to_glium_cords(anchor[0]),
        convert_sampling_cord_to_glium_cords(anchor[1]),
    ];

    let obstacle_matrix = [
        [SCALE, 0.0, 0.0, 0.0],
        [0.0, SCALE, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 2.0, 1.0f32],
    ];

    let matrix_start_circle = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [anchor_pos[0] * SCALE, anchor_pos[1] * SCALE, 2.0, 1.0f32],
    ];

    let mut initial_clear = true;
    let mut no_cleaning = true;
    let mut refresh_rate = 166_669_566;
    if display_trace == 2.0 {
        no_cleaning = false;
        refresh_rate = 166_669;
    }

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(refresh_rate);
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

        if initial_clear {
            target.clear_color_and_depth((0.8, 0.8, 0.8, 0.8), 1.0);
            initial_clear = false;
        }
        if initial_clear || no_cleaning  {
            target.clear_color_and_depth((0.8, 0.8, 0.8, 0.8), 1.0);
        }
        // target.clear_color_and_depth((0.8, 0.8, 0.8, 0.8), 1.0); //test please uncomment it after debugging


        let [t1, t2, t3] = path[path_index]; // this is the right code

        let matrix1 = [
        [ t1.cos() * SCALE, t1.sin() * SCALE, 0.0, 0.0],
        [-t1.sin() * SCALE, t1.cos() * SCALE, 0.0, 0.0],
        [0.0, 0.0, SCALE, 0.0],
        [anchor_pos[0] * SCALE, anchor_pos[1] * SCALE, 2.0, 1.0f32],
        ];

        //we don't need to scale again here because the first matrix already took care of it
        let matrix2 = [
        [ t2.cos() , t2.sin() , 0.0, 0.0],
        [-t2.sin() , t2.cos() , 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [arm_length , arm_width, 0.0, 1.0f32],
        ];

        let matrix3 = [
        [ t3.cos() , t3.sin() , 0.0, 0.0],
        [-t3.sin() , t3.cos() , 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [arm_length, arm_width, 0.0, 1.0f32],
        ];

        if path_index < 1 {
            path_index = path.len() - 1;
        } else {
            path_index -= 1;
        }

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;
            let f = 1.0 / (fov / 2.0).tan();
            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };


        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let identity_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

        // for the rectangular arms
        target
            .draw(
                (&robot_arm, &normals),
                &indices,
                &program_robot_arm,
                &uniform! {matrix: identity_matrix, matrix2: matrix1, matrix3: identity_matrix,  u_light: light_souce, perspective: perspective},
                &params,
            )
            .unwrap();
        target
            .draw(
                (&robot_arm, &normals),
                &indices,
                &program_robot_arm,
                &uniform! { matrix: matrix1, matrix2: matrix2,matrix3: identity_matrix, u_light: light_souce,perspective: perspective},
                &params,
            )
            .unwrap();

        target
        .draw(
            (&robot_arm, &normals),
            &indices,
            &program_robot_arm,
            &uniform! { matrix: matrix1, matrix2: matrix2, matrix3: matrix3, u_light: light_souce,perspective: perspective},
            &params,
        )
        .unwrap();
        //testing out drawing w a stick figure first
        // target
        // .draw(
        //     (&robot_arm_stick, &normals),
        //     &robot_arm_stick_indices,
        //     &program_robot_arm,
        //     &uniform! { matrix: matrix1, matrix2: matrix2, matrix3: matrix3, u_light: light_souce},
        //     &params_draw_line,
        // )
        // .unwrap();
        // for the start circular disk
        target
        .draw(
            (&circular_position, &circular_normals),
            &circular_indices,
            &program_circle,
            &uniform! { matrix: matrix_start_circle, u_light: light_souce, perspective: perspective},
            &params,
        )
        .unwrap();

        // for displaying obstacles
        target
        .draw(
            (&obstacle_vertices, &obstacle_normals),
            &obstacle_indices,
            &program_obstacle,
            &uniform! { matrix: obstacle_matrix, u_light: light_souce, perspective: perspective},
            &params,
        )
        .unwrap();
        target.finish().unwrap();
    });
}
