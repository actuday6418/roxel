mod render;
mod voxels;
use std::env;

use pollster::FutureExt;
use render::{State, Vector2f, Vertex};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const FAR_DIST: u32 = 700;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Enter the source files for color and height data!");
    }

    //Load images to memory
    let vd = voxels::VoxelData::new(&args[1], &args[2]);

    let mut theta: f32 = 0.0;
    let mut height: i32 = 100;
    let mut origin: (i32, i32) = (0, 0);
    let mut z_buff: Vec<u32> = Vec::new();
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut state = State::new(&window).block_on();
    let winit::dpi::PhysicalSize {
        width: mut win_w,
        height: mut win_h,
    } = window.inner_size();
    z_buff.resize_with(win_w as usize + 1, Default::default);
    let mut vertices: Vec<Vertex> = Vec::new();
    println!("dims: {} {}", win_w, win_h);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => match keycode {
                        VirtualKeyCode::A => theta += 0.1,
                        VirtualKeyCode::D => theta -= 0.1,
                        VirtualKeyCode::Right => origin.0 -= 10,
                        VirtualKeyCode::Left => origin.0 += 10,
                        VirtualKeyCode::Up => origin.1 += 10,
                        VirtualKeyCode::Down => origin.1 -= 10,
                        VirtualKeyCode::W => height += 10,
                        VirtualKeyCode::S => height -= 10,
                        _ => {}
                    },
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so w have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                z_buff.iter_mut().for_each(|x| *x = win_h);
                vertices.clear();
                state.num_vertices = 0;
                let sin = theta.sin();
                let cos = theta.cos();
                let mut dist = 1f32;
                let mut dz = 1f32;

                while dist < FAR_DIST as f32 {
                    let mut pl = Vector2f {
                        x: (-cos * dist as f32 - sin * dist as f32) + origin.0 as f32,
                        y: (sin * dist as f32 - cos * dist as f32) + origin.1 as f32,
                    };
                    let pr = Vector2f {
                        x: (cos * dist as f32 - sin * dist as f32) + origin.0 as f32,
                        y: (-sin * dist as f32 - cos * dist as f32) + origin.1 as f32,
                    };

                    let dx: f32 = (pr.x - pl.x) / (win_w as f32);
                    let dy: f32 = (pr.y - pl.y) / (win_w as f32);

                    for i in 1..(win_w + 1) {
                        let mut projected_height = (height as f32
                            - vd.get_height(&mut ((pl.x) as i32), &mut ((pl.y) as i32)))
                            / dist
                            * 240f32;
                        if projected_height > win_h as f32 {
                            projected_height = (win_h) as f32;
                        } else if projected_height <= 0f32 {
                            projected_height = 1f32;
                        }
                        draw_vertical_line(
                            i as f32,
                            projected_height as f32 - 1f32,
                            z_buff[i as usize] as f32,
                            &mut vertices,
                            vd.get_color(&mut ((pl.x) as i32), &mut ((pl.y) as i32)),
                        );
                        state.num_vertices += 2;
                        if projected_height < z_buff[i as usize] as f32 {
                            z_buff[i as usize] = projected_height as u32;
                        }
                        pl.x += dx;
                        pl.y += dy;
                    }
                    dist += dz;
                    dz += 0.2;
                }
                state
                    .queue
                    .write_buffer(&state.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

fn draw_vertical_line(
    x: f32,
    y_top: f32,
    y_bottom: f32,
    vertices: &mut Vec<Vertex>,
    color: [f32; 3],
) {
    vertices.push(Vertex {
        position: [x, y_top, 0.0],
        color,
    });
    vertices.push(Vertex {
        position: [x, y_bottom, 0.0],
        color,
    });
}
