use raytracer::{Camera, Vec3, World};

use std::error;
use std::sync::{Arc, Mutex};
use std::thread;

use rand::prelude::*;

use image;

#[macro_use]
extern crate glium;

fn main() -> Result<(), Box<dyn error::Error>> {
    let nx = 400;
    let ny = 200;
    let ns = 64;
    let image_buffer = Arc::new(Mutex::new(vec![0u8; nx * ny * 4])); // RGBA

    // Run raytracing computation on a separated thread.
    let handle = {
        let image_buffer = Arc::clone(&image_buffer);
        thread::spawn(move || {
            let world = World::random();

            let look_from = Vec3::new(13.0, 2.0, 3.0);
            let look_at = Vec3::new(0.0, 0.0, 0.0);
            let vertical_fov = 20.0;
            let aspect_ratio = nx as f32 / ny as f32;
            let aperture = 0.1;
            let distance_to_focus = 10.0;
            let camera = Camera::new(
                look_from,
                look_at,
                Vec3::new(0.0, 1.0, 0.0),
                vertical_fov,
                aspect_ratio,
                aperture,
                distance_to_focus,
            );

            for y in (0..ny).rev() {
                for x in 0..nx {
                    let mut col = Vec3::default();
                    for _ in 0..ns {
                        let u = (x as f32 + random::<f32>()) / nx as f32;
                        let v = (y as f32 + random::<f32>()) / ny as f32;
                        let ray = camera.ray_at(u, v);
                        col += world.color(&ray, 0);
                    }
                    col /= ns as f32;

                    // Gamma correction (gamma 2).
                    col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

                    let ir = (255.9 * col[0]) as u8;
                    let ig = (255.9 * col[1]) as u8;
                    let ib = (255.9 * col[2]) as u8;
                    let ia = 255;

                    let position = ((ny - y - 1) * nx + x) << 2;

                    {
                        // Write pixel color.
                        let mut buffer = image_buffer.lock().unwrap();
                        buffer[position + 0] = ir;
                        buffer[position + 1] = ig;
                        buffer[position + 2] = ib;
                        buffer[position + 3] = ia;
                    }
                }
            }
        })
    };

    // On the main thread, show raytracing progress within a glium window.
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_title("Raytracing with Rust!")
        .with_dimensions(glutin::dpi::LogicalSize::new(nx as f64, ny as f64));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop)?;

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    glium::implement_vertex!(Vertex, position);
    let vertex_buffer = glium::VertexBuffer::new(
        &display,
        &[
            Vertex {
                position: [1.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0],
            },
            Vertex {
                position: [-1.0, -1.0],
            },
        ],
    )?;
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        out vec2 v_tex_coords;
        void main() {
            vec2 madd = vec2(0.5, 0.5);
            v_tex_coords = position * madd + madd;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)?;

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let image = {
            let buffer = image_buffer.lock().unwrap();
            glium::texture::RawImage2d::from_raw_rgba_reversed(&buffer, (nx as u32, ny as u32))
        };
        let texture = glium::texture::Texture2d::new(&display, image)?;
        let uniforms = glium::uniform! {
            tex: &texture,
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                _ => (),
            },
            _ => (),
        });
    }

    handle.join().unwrap();

    // When finished save image to disk.
    let buffer = image_buffer.lock().unwrap();
    image::save_buffer("out.png", &buffer, nx as u32, ny as u32, image::RGBA(8))?;

    Ok(())
}
