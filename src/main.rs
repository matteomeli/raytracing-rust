use raytracer::{Camera, Vec3, World};

use std::error;
use std::sync::{Arc, Mutex};
use std::thread;

use rand::prelude::*;

use image;

struct RaytracerData {
    width: u32,
    height: u32,
    samples: Vec<f32>,
    pixels: Vec<u8>,
}

impl RaytracerData {
    pub fn add_sample(&mut self, x: u32, y: u32, sample: Vec3) {
        let index = ((y * self.width + x) * 5) as usize;
        self.samples[index + 0] += sample.r();
        self.samples[index + 1] += sample.g();
        self.samples[index + 2] += sample.b();
        self.samples[index + 3] += 1.0; // opaque
        self.samples[index + 4] += 1.0; // increase number of samples
    }

    pub fn pixels(&mut self) -> &[u8] {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let data_index = ((y * self.width + x) * 5) as usize;
                let pixel_index = (((self.height - y - 1) * self.width + x) << 2) as usize;
                let num_samples = self.samples[data_index + 4];

                self.pixels[pixel_index + 0] =
                    (255.99 * (self.samples[data_index + 0] / num_samples).sqrt()) as u8;
                self.pixels[pixel_index + 1] =
                    (255.99 * (self.samples[data_index + 1] / num_samples).sqrt()) as u8;
                self.pixels[pixel_index + 2] =
                    (255.99 * (self.samples[data_index + 2] / num_samples).sqrt()) as u8;
                self.pixels[pixel_index + 3] = 255u8;
            }
        }

        &self.pixels
    }
}

struct Raytracer {
    width: u32,
    height: u32,
    num_samples: u32,
    pub shared: Arc<Mutex<RaytracerData>>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Raytracer {
    pub fn new(width: u32, height: u32, num_samples: u32) -> Self {
        let num_pixels = (width * height) as usize;
        Raytracer {
            width,
            height,
            num_samples,
            shared: Arc::new(Mutex::new(RaytracerData {
                width,
                height,
                samples: vec![0.0; num_pixels * 5], // RGBA + samples count
                pixels: vec![0u8; num_pixels * 4],  // RGBA
            })),
            thread: None,
        }
    }

    pub fn start(&mut self) {
        let shared = self.shared.clone();
        let ns = self.num_samples;
        let nx = self.width;
        let ny = self.height;

        let handle = thread::spawn(move || {
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
                    for _ in 0..ns {
                        let u = (x as f32 + random::<f32>()) / nx as f32;
                        let v = (y as f32 + random::<f32>()) / ny as f32;
                        let ray = camera.ray_at(u, v);
                        let col = world.color(&ray, 0);

                        shared.lock().unwrap().add_sample(x, y, col);
                    }
                }
            }
        });

        self.thread = Some(handle);
    }
}

impl Drop for Raytracer {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let width = 400;
    let height = 200;
    let num_samples = 100;

    let mut raytracer = Raytracer::new(width, height, num_samples);
    raytracer.start();

    // On the main thread, show raytracing progress within a glium window.
    use glium::index::PrimitiveType;
    use glium::{glutin, implement_vertex, program, uniform, Surface, VertexBuffer};

    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_title("Raytracing with Rust!")
        .with_dimensions(glutin::dpi::LogicalSize::new(width as f64, height as f64));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &events_loop)?;

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }
        implement_vertex!(Vertex, position, tex_coords);
        VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [1.0, 1.0],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [-1.0, 1.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0],
                    tex_coords: [1.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0],
                    tex_coords: [0.0, 0.0],
                },
            ],
        )?
    };

    let indices = glium::index::NoIndices(PrimitiveType::TriangleStrip);

    let program = program!(&display,
    140 => {
        vertex: r#"
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            "#,
        fragment: r#"
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 color;
                void main() {
                    color = texture(tex, v_tex_coords);
                }
            "#
    })?;

    let mut closed = false;
    while !closed {
        let image = {
            let mut shared = raytracer.shared.lock().unwrap();
            glium::texture::RawImage2d::from_raw_rgba_reversed(
                shared.pixels(),
                (width as u32, height as u32),
            )
        };
        let texture = glium::texture::Texture2d::new(&display, image)?;
        let uniforms = glium::uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex: &texture,
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default(),
        )?;
        target.finish()?;

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                _ => (),
            },
            _ => (),
        });
    }

    // When finished save image to disk.
    let mut shared = raytracer.shared.lock().unwrap();
    image::save_buffer(
        "out.png",
        shared.pixels(),
        width as u32,
        height as u32,
        image::RGBA(8),
    )?;

    Ok(())
}
