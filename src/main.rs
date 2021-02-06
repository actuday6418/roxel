extern crate image;
extern crate sfml;

mod voxels;

use sfml::graphics::{Color, PrimitiveType, RenderTarget, RenderWindow, Vertex, VertexArray};
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, Event, Key, Style};

//use std::fs::File;

use std::env;

const WIN_H: u32 = 512;
const WIN_W: u32 = 700;
const FAR_DIST: u32 = 700;

fn draw_vline(vertices: &mut VertexArray, x: f32, y_top: f32, y_bottom: f32, color: Vec<u8>) {
    vertices.append(&Vertex::new(
        Vector2f::new(x, y_top),
        Color::rgb(color[0], color[1], color[2]),
        Vector2f::new(0., 0.),
    ));
    vertices.append(&Vertex::new(
        Vector2f::new(x, y_bottom),
        Color::rgb(color[0], color[1], color[2]),
        Vector2f::new(0., 0.),
    ));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Enter the source files for color and height data!");
    }

    //Load images to memory
    let vd = voxels::VoxelData::new(&args[1], &args[2]);

    let context_settings = ContextSettings::default();
    let mut window = RenderWindow::new((WIN_W, WIN_H), "Voxels", Style::CLOSE, &context_settings);
    window.set_vertical_sync_enabled(true);
    let mut vertices = VertexArray::default();
    vertices.set_primitive_type(PrimitiveType::LineStrip);
    let mut theta: f32 = 0.0;
    let mut height: i32 = 100;
    let mut origin: (i32, i32) = (0, 0);
    let mut z_buff = [WIN_H-1; (WIN_W + 1) as usize];
    let mut dist;
    let mut dz;

    loop {
        vertices.clear();
        z_buff.iter_mut().for_each(|x| *x = WIN_H);
        dist = 1f32;
        dz = 1f32;

        //Handle window events
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code: Key::A, .. } => {
                    theta += 0.1;
                }
                Event::KeyPressed { code: Key::D, .. } => {
                    theta -= 0.1;
                }
                Event::KeyPressed {
                    code: Key::Right, ..
                } => {
                    origin.0 -= 10;
                }
                Event::KeyPressed {
                    code: Key::Left, ..
                } => {
                    origin.0 += 10;
                }
                Event::KeyPressed { code: Key::Up, .. } => {
                    origin.1 += 10;
                }
                Event::KeyPressed {
                    code: Key::Down, ..
                } => {
                    origin.1 -= 10;
                }
                Event::KeyPressed { code: Key::W, .. } => {
                    height += 10;
                }
                Event::KeyPressed { code: Key::S, .. } => {
                    height -= 10;
                }
                _ => {}
            }
        }

        let sin = theta.sin();
        let cos = theta.cos();

        //window.clear(Color::BLUE);

        while dist < FAR_DIST as f32 {
            let mut pl = Vector2f::new(
                (-cos * dist as f32 - sin * dist as f32) + origin.0 as f32,
                (sin * dist as f32 - cos * dist as f32) + origin.1 as f32,
            );
            let pr = Vector2f::new(
                (cos * dist as f32 - sin * dist as f32) + origin.0 as f32,
                (-sin * dist as f32 - cos * dist as f32) + origin.1 as f32,
            );

            let dx: f32 = (pr.x - pl.x) / (WIN_W as f32);
            let dy: f32 = (pr.y - pl.y) / (WIN_W as f32);

            for i in 0..(WIN_W + 1) {
                let mut projected_height = (height as f32 - vd.get_height(
                        &mut ((pl.x + vd.width / 2 as f32) as i32),
                        &mut ((pl.y + vd.height / 2 as f32) as i32),
                    ))/dist*240f32;
                if projected_height > WIN_H as f32 {
                    projected_height = (WIN_H - 1) as f32;
                } else if projected_height < 0f32 {
                    projected_height = 0f32;
                }
                    draw_vline(
                        &mut vertices,
                        i as f32,
                        projected_height as f32-1f32,
                        z_buff[i as usize] as f32,
                        vd.get_color(
                            &mut ((pl.x + vd.width / 2 as f32) as i32),
                            &mut ((pl.y + vd.height / 2 as f32) as i32),
                        ),
                );
                if projected_height < z_buff[i as usize] as f32 {
                    z_buff[i as usize] = projected_height as u32;
                }
                pl.x += dx;
                pl.y += dy;
            }
            dist += dz;
            dz += 0.2;
        }
        window.clear(Color::CYAN);
        window.draw(&vertices);
        window.display();
    }
}


/*
CODE that will simply display a PNG
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Enter the source files for color and height data!");
    }
    let vd = voxels::VoxelData::new(&args[1], &args[2]);

    let context_settings = ContextSettings::default();
    let mut window = RenderWindow::new((WIN_W, WIN_H), "Voxels", Style::CLOSE, &context_settings);
    window.set_vertical_sync_enabled(true);
    let mut vertices = VertexArray::default();
    vertices.set_primitive_type(PrimitiveType::Quads);
    
    let THICC: u32 = WIN_H/vd.height;

    for i in 0..vd.height {
        for j in 0..vd.width {
            let c = vd.get_color(&mut (i as i32),&mut (j as i32));
            let h = vd.get_height(&mut (i as i32),&mut (j as i32));
            vertices.append(&Vertex::new(
                Vector2f::new((i*THICC) as f32,(j*THICC) as f32),
                Color::rgb(c[0], c[1], c[2]),
                Vector2f::new(0., 0.),
            ));
            vertices.append(&Vertex::new(
                Vector2f::new((i*THICC+THICC) as f32,(j*THICC) as f32),
                Color::rgb(c[0], c[1], c[2]),
                Vector2f::new(0., 0.),
            ));
            vertices.append(&Vertex::new(
                Vector2f::new((i*THICC+THICC) as f32,(j*THICC+THICC) as f32),
                Color::rgb(c[0], c[1], c[2]),
                Vector2f::new(0., 0.),
            ));
            vertices.append(&Vertex::new(
                Vector2f::new((i*THICC) as f32,(j*THICC+THICC) as f32),
                Color::rgb(c[0], c[1], c[2]),
                Vector2f::new(0., 0.),
            ));
        }
    }
    window.draw(&vertices);
    window.display();
    loop{}
}
*/