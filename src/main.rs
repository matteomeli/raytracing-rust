use raytracer::{Camera, Vec3, World};

use std::error;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc, Arc, Mutex,
};
use std::thread;
use std::time::Instant;

use rand::prelude::*;

use image;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

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
    num_threads: usize,
    pub shared: Arc<Mutex<RaytracerData>>,
    pool: ThreadPool,
    has_started: bool,
    num_started: usize,
    mum_completed: Arc<AtomicUsize>,
}

impl Raytracer {
    pub fn new(width: u32, height: u32, num_samples: u32, num_threads: usize) -> Self {
        let num_pixels = (width * height) as usize;

        let shared = Arc::new(Mutex::new(RaytracerData {
            width,
            height,
            samples: vec![0.0; num_pixels * 5], // RGBA + samples count
            pixels: vec![0u8; num_pixels * 4],  // RGBA
        }));

        let pool = ThreadPool::new(num_threads);

        let has_started = false;
        let num_started = 0;
        let mum_completed = Arc::new(AtomicUsize::new(0));

        Raytracer {
            width,
            height,
            num_samples,
            num_threads,
            shared,
            pool,
            has_started,
            num_started,
            mum_completed,
        }
    }

    pub fn render(&mut self) {
        let width = self.width;
        let height = self.height;
        let num_samples = self.num_samples;

        let world = Arc::new(World::random());

        let look_from = Vec3::new(13.0, 2.0, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let vertical_fov = 20.0;
        let aspect_ratio = width as f32 / height as f32;
        let aperture = 0.1;
        let distance_to_focus = 10.0;
        let camera = Arc::new(Camera::new(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            vertical_fov,
            aspect_ratio,
            aperture,
            distance_to_focus,
        ));

        // Simply divide horizontally for now, in num_cpu's equal chunks
        let chunks = self.num_threads as u32;
        let chunk_size = width / chunks;
        for chunk in 0..chunks {
            let shared = self.shared.clone();
            let world = world.clone();
            let camera = camera.clone();

            let start_x = chunk * chunk_size;
            let end_x = start_x + chunk_size;

            let num_completed = self.mum_completed.clone();

            self.pool.execute(move || {
                for y in (0..height).rev() {
                    for x in start_x..end_x {
                        for _ in 0..num_samples {
                            let u = (x as f32 + random::<f32>()) / width as f32;
                            let v = (y as f32 + random::<f32>()) / height as f32;
                            let ray = camera.ray_at(u, v);
                            let col = world.color(&ray, 0);

                            shared.lock().unwrap().add_sample(x, y, col);
                        }
                    }
                }

                num_completed.fetch_add(1, Ordering::SeqCst);
            });

            self.num_started += 1;
        }

        self.has_started = true;
    }

    pub fn is_done(&self) -> bool {
        self.has_started && self.mum_completed.fetch_add(0, Ordering::SeqCst) == self.num_started
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let width = 400;
    let height = 200;
    let num_samples = 100;
    let mut raytracer = Raytracer::new(width, height, num_samples, num_cpus::get());
    let mut has_finished_rendering = false;

    let start_time = Instant::now();
    raytracer.render();

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

        if !has_finished_rendering && raytracer.is_done() {
            let end_time = Instant::now();
            println!(
                "Rendering finished in {}s.",
                end_time.duration_since(start_time).as_secs_f64()
            );
            has_finished_rendering = true;
        }

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
