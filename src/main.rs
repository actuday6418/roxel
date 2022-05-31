use glium::glutin::event::{KeyboardInput, VirtualKeyCode};
use glium::{glutin, implement_vertex, program, uniform, Surface};
mod voxels;

const FAR_DIST: u32 = 700;
const WHITE: [f32; 3] = [1f32; 3];
const TARGET_FPS: u64 = 60;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

impl Vertex {
    pub fn new(x: f32, y: f32, color: [f32; 3]) -> Self {
        Vertex {
            position: [x, y],
            color,
        }
    }
}

struct State {
    pub theta: f32,
    pub height: i32,
    pub origin: (i32, i32),
    pub z_buff: [u32; 3000],
    pub vertices: Vec<Vertex>,
    pub dist: f32,
    pub dz: f32,
    pub w_height: u32,
    pub w_width: u32,
    vd: voxels::VoxelData,
    program: glium::Program,
    display: glium::Display,
}

impl State {
    fn new(program: glium::Program, display: glium::Display, vd: voxels::VoxelData) -> Self {
        State {
            vd,
            w_width: 0,
            w_height: 0,
            theta: 0f32,
            height: 100,
            origin: (0, 0),
            z_buff: [0; 3000],
            vertices: Vec::new(),
            dz: 0f32,
            dist: 0f32,
            program,
            display,
        }
    }
    fn set_dimensions(&mut self, h: u32, w: u32) {
        self.w_width = w;
        self.w_height = h;
        self.z_buff.iter_mut().for_each(|x| *x = h);
    }
}

fn draw(state: &mut State) {
    state.vertices.clear();
    let h = state.w_height;
    state.z_buff.iter_mut().for_each(|x| *x = h);
    state.dist = 1f32;
    state.dz = 1f32;
    let sin = state.theta.sin();
    let cos = state.theta.cos();

    while state.dist < FAR_DIST as f32 {
        let mut pl = Vertex::new(
            (-cos * state.dist as f32 - sin * state.dist as f32) + state.origin.0 as f32,
            (sin * state.dist as f32 - cos * state.dist as f32) + state.origin.1 as f32,
            WHITE,
        );
        let pr = Vertex::new(
            (cos * state.dist as f32 - sin * state.dist as f32) + state.origin.0 as f32,
            (-sin * state.dist as f32 - cos * state.dist as f32) + state.origin.1 as f32,
            WHITE,
        );

        let dx: f32 = (pr.position[0] - pl.position[0]) / (state.w_width as f32);
        let dy: f32 = (pr.position[1] - pl.position[1]) / (state.w_width as f32);

        for i in 1..(state.w_width + 1) {
            let mut projected_height = (state.height as f32
                - state.vd.get_height(
                    &mut ((pl.position[0]) as i32),
                    &mut ((pl.position[1]) as i32),
                ))
                / state.dist
                * 1000f32;
            if projected_height > state.w_height as f32 {
                projected_height = (state.w_height) as f32;
            } else if projected_height <= 0f32 {
                projected_height = 1f32;
            }
            state.vertices.push(Vertex::new(
                2f32 * i as f32 / state.w_width as f32 - 1f32,
                2f32 * projected_height as f32 / state.w_height as f32 - 1f32,
                state
                    .vd
                    .get_color(&mut ((pl.position[0]) as i32), &mut (pl.position[1] as i32)),
            ));
            state.vertices.push(Vertex::new(
                2f32 * i as f32 / state.w_width as f32 - 1f32,
                2f32 * state.z_buff[i as usize] as f32 / state.w_height as f32 - 1f32,
                state.vd.get_color(
                    &mut ((pl.position[0]) as i32),
                    &mut ((pl.position[1]) as i32),
                ),
            ));
            if projected_height < state.z_buff[i as usize] as f32 {
                state.z_buff[i as usize] = projected_height as u32;
            }
            pl.position[0] += dx;
            pl.position[1] += dy;
        }
        state.dist += state.dz;
        state.dz += 0.1;
    }
    // building the uniforms
    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1f32]
        ]
    };

    // building the index buffer
    let indices =
        glium::uniforms::UniformBuffer::<glium::index::DrawCommandNoIndices>::empty_dynamic(
            &state.display,
        )
        .unwrap();
    indices.write(&glium::index::DrawCommandNoIndices {
        count: state.vertices.len() as u32,
        instance_count: 1u32,
        first_index: 0u32,
        base_instance: 0u32,
    });
    let indices = glium::index::IndicesSource::MultidrawArray {
        buffer: indices.as_slice_any(),
        primitives: glium::index::PrimitiveType::LineLoop,
    };
    // drawing a frame
    let mut target = state.display.draw();
    target.clear_color(0.04, 0.2, 0.8, 0.0);
    target
        .draw(
            &glium::VertexBuffer::new(&state.display, state.vertices.as_slice()).unwrap(),
            indices,
            &state.program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
    target.finish().unwrap();
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        panic!("Enter the source files for color and height data!");
    }

    //Load images to memory
    let vd = voxels::VoxelData::new(&args[1], &args[2]);

    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec3 color;
                out vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;
                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },

        110 => {
            vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec2 position;
                attribute vec3 color;
                varying vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 110
                varying vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },

        100 => {
            vertex: "
                #version 100
                uniform lowp mat4 matrix;
                attribute lowp vec2 position;
                attribute lowp vec3 color;
                varying lowp vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 100
                varying lowp vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },
    )
    .unwrap();

    let mut state = State::new(program, display, vd);
    state.set_dimensions(
        state.display.get_framebuffer_dimensions().0,
        state.display.get_framebuffer_dimensions().1,
    );
    // Draw the triangle to the screen.
    draw(&mut state);

    // the main loop
    event_loop.run(move |event, _, control_flow| {
        let start_time = std::time::Instant::now();
        *control_flow = match event {
            glutin::event::Event::RedrawRequested(_) => {
                draw(&mut state);
                glutin::event_loop::ControlFlow::Wait
            }
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    draw(&mut state);
                    glutin::event_loop::ControlFlow::Poll
                }
                glutin::event::WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: glutin::event::ElementState::Pressed,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => {
                    match keycode {
                        VirtualKeyCode::A => state.theta += 0.1,
                        VirtualKeyCode::D => state.theta -= 0.1,
                        VirtualKeyCode::Right => state.origin.0 -= 10,
                        VirtualKeyCode::Left => state.origin.0 += 10,
                        VirtualKeyCode::Up => state.origin.1 += 10,
                        VirtualKeyCode::Down => state.origin.1 -= 10,
                        VirtualKeyCode::W => state.height += 10,
                        VirtualKeyCode::S => state.height -= 10,
                        _ => {}
                    };
                    glutin::event_loop::ControlFlow::Wait
                }
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
        match *control_flow {
            glutin::event_loop::ControlFlow::Exit => {}
            _ => {
                state.display.gl_window().window().request_redraw();
                /*
                 * Below logic to attempt hitting TARGET_FPS.
                 * Basically, sleep for the rest of our milliseconds
                 */
                let elapsed_time = std::time::Instant::now()
                    .duration_since(start_time)
                    .as_millis() as u64;

                let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                    true => 1000 / TARGET_FPS - elapsed_time,
                    false => 0,
                };
                let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
                *control_flow = glutin::event_loop::ControlFlow::WaitUntil(new_inst);
            }
        }
    });
}
