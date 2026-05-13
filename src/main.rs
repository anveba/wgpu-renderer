use algebra::*;
use graphics::{Material, Rasteriser, Shader, Texture};
use resources::MeshCache;
use scene::{Scene, Transform};
use topology::{Mesh, Vertex};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod algebra;
mod graphics;
mod interface;
mod resources;
mod scene;
mod topology;

pub fn read_bytes_from_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(path)
}

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter =
        futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

    let (device, queue) = futures::executor::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None, // Trace path
    ))
    .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);

    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .filter(|f| f.is_srgb())
        .next()
        .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    let mut mesh_cache = MeshCache::new();

    let mesh = Mesh::new(
        vec![
            Vertex::new(
                Vec3::new(0.5, 0.5, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec2::new(1.0, 0.0),
            ),
            Vertex::new(
                Vec3::new(-0.5, 0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec2::new(0.0, 0.0),
            ),
            Vertex::new(
                Vec3::new(-0.5, -0.5, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec2::new(0.0, 1.0),
            ),
            Vertex::new(
                Vec3::new(0.5, -0.5, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec2::new(1.0, 1.0),
            ),
        ],
        vec![0, 1, 2, 2, 3, 0],
    );
    let mesh_id = mesh_cache.add_mesh(mesh, &device, "quad");
    let tex_data = Texture::from_bytes(
        read_bytes_from_file("texture.png")
            .expect("Could not read file")
            .as_slice(),
        &device,
        &queue,
        None,
    );

    let material = Material::new(&tex_data);

    let mut scene = Scene::new();
    scene.add_instance(mesh_cache.get_handle(mesh_id), material, Transform::none());

    let shader = Shader::from_src(include_str!("shaders/test_shader.wgsl"), &device);

    let rasteriser = Rasteriser::new(&shader, &config.format, &device);

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            Event::AboutToWait => {
                let output = surface.get_current_texture().unwrap();
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                rasteriser.draw(&scene, &view, &mut encoder, &device);

                queue.submit(std::iter::once(encoder.finish()));
                output.present();

                //window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {}
            _ => (),
        })
        .expect("event loop error");
}

fn main() {
    run();
}
