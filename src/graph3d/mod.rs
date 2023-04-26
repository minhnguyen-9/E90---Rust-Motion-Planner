use std::iter::Scan;

use glium::{glutin, implement_vertex, uniform, Surface};

use crate::graph3d::sync_data::CIRCLE;
mod parser_glium;
mod sync_data;
// mod sync_data;
const SCALE: f32 = 0.01;

pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Debug)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

pub trait As3d {
    fn as_3d(&self) -> (f32, f32, f32);
}

impl As3d for Point3 {
    fn as_3d(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

impl As3d for Point2 {
    fn as_3d(&self) -> (f32, f32, f32) {
        (self.x, self.y, 0.0)
    }
}

pub fn graph3d<T: As3d + 'static>(path: Vec<T>) {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let filepath = "src/bin/obstacles.txt";
    let vertices =
        parser_glium::obstacle_parser_glium(&filepath).expect("can't parse in obstacle vertices");
    let connecting_indices =
        parser_glium::obstacle_parser_find_connecting_indices(vertices.len() / 4)
            .expect("can't parse in obstacle connecting vertices");

    let positions = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let normals = glium::VertexBuffer::new(&display, &sync_data::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &connecting_indices,
    )
    .unwrap();

    let robot = glium::VertexBuffer::new(&display, &sync_data::ROBOT).expect("no robot found");
    //let path = glium::VertexBuffer::new(&display, &sync_data::PATHS).expect("no paths found"); // this is for reading in from sync_data
    let indices2 = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &sync_data::INDICES2,
    )
    .unwrap();
    let normals2 = glium::VertexBuffer::new(&display, &sync_data::NORMALS2).unwrap();

    let circular_position = glium::VertexBuffer::new(&display, &CIRCLE).unwrap();
    let circular_normals = glium::VertexBuffer::new(&display, &sync_data::CIRCLE_NORMALS).unwrap();
    let circular_indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TriangleFan,
        &sync_data::CIRCLE_INDICES,
    )
    .unwrap();

    let vertex_shader_obstacle = r#"
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

    let fragment_shader_obstacle = r#"
        #version 150
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 0.3);
        }
    "#;

    let vertex_shader_robot = r#"

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

    let fragment_shader_robot = r#"
        #version 150
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.2, 0.3);
            vec3 regular_color = vec3(1.0, 0.2, 0.3);
            color = vec4(mix(dark_color, regular_color, brightness), 0.5);
        }
    "#;

    let vertex_shader_goal_circle = r#"
        #version 150
        in vec3 position;
        in vec3 normal;
        out vec3 v_normal;
        uniform mat4 matrix;
        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 0.3);
        }
    "#;

    let fragment_shader_goal_circle = r#"
        #version 150
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.0, 0.7, 0.0);
            vec3 regular_color = vec3(0.0, 0.6, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program = glium::Program::from_source(
        &display,
        vertex_shader_obstacle,
        fragment_shader_obstacle,
        None,
    )
    .unwrap();

    let program2 =
        glium::Program::from_source(&display, vertex_shader_robot, fragment_shader_robot, None)
            .unwrap();

    let program_circle = glium::Program::from_source(
        &display,
        vertex_shader_goal_circle,
        fragment_shader_goal_circle,
        None,
    )
    .unwrap();

    // let mut path_index = 0;
    let mut path_index = path.len() - 1;

    let (x, y, z) = path[0].as_3d();
    let matrix_goal_circle = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [
            (-100.0 + x * 25.0) * SCALE,
            (-100.0 + y * 25.0) * SCALE,
            z * SCALE,
            1.0f32,
        ],
    ];

    let start_idx = path.len() - 1;
    let (x, y, z) = path[start_idx].as_3d();
    let matrix_start_circle = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [
            (-100.0 + x * 25.0) * SCALE,
            (-100.0 + y * 25.0) * SCALE,
            z * SCALE,
            1.0f32,
        ],
    ];

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
        target.clear_color_and_depth((0.8, 0.8, 0.8, 0.8), 1.0);

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        if path_index <= 1 {
            path_index = path.len() - 1;
        } else {
            path_index -= 1;
        }

        let (x, y, z) = path[path_index].as_3d();

        let matrix2 = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [
                (-100.0 + x * 25.0) * SCALE,
                (-100.0 + y * 25.0) * SCALE,
                z * SCALE,
                1.0f32,
            ],
        ];

        let light = [-1.0, 0.4, 0.9f32];
        let circle_light = [-1.0, 0.4, 0.9f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // for the rectangular obstacles
        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! { matrix: matrix, u_light: light},
                &params,
            )
            .unwrap();

        // for the goal circular disk
        target
            .draw(
                (&circular_position, &circular_normals),
                &circular_indices,
                &program_circle,
                &uniform! { matrix: matrix_goal_circle, u_light: circle_light},
                &params,
            )
            .unwrap();

        // for the start circular disk
        target
            .draw(
                (&circular_position, &circular_normals),
                &circular_indices,
                &program_circle,
                &uniform! { matrix: matrix_start_circle, u_light: circle_light},
                &params,
            )
            .unwrap();

        // to move the path the path to the goal
        target
            .draw(
                (&circular_position, &circular_normals),
                &circular_indices,
                &program2,
                &uniform! { matrix: matrix2, u_light: light},
                &params,
            )
            .unwrap();

        target.finish().unwrap();
    });
}
