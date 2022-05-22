use nalgebra::{vector, Vector2};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;
use winit::window::Window;

use crate::uniform::{GlobalUniforms, UniformData};
use crate::vertex::VertexList;

pub mod prerender;
use prerender::PrerenderState;
pub mod radiance;
use radiance::{RadianceSettings, RadianceState};
pub mod render;
use render::RenderState;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct IntermediateState<'a> {
    instance: &'a Instance,
    surface: &'a Surface,
    adapter: &'a Adapter,
    device: &'a Device,
    queue: &'a Queue,
    config: &'a SurfaceConfiguration,
    size: Vector2<u32>,
    global_uniforms: &'a UniformData<GlobalUniforms>,
    fullscreen_buffer: &'a Buffer,
    fullscreen_vert: &'a ShaderModule,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct State {
    instance: Instance,
    surface: Surface,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: Vector2<u32>,
    global_uniforms: UniformData<GlobalUniforms>,
    fullscreen_buffer: Buffer,
    prerender_state: PrerenderState,
    radiance_state: RadianceState,
    render_state: RenderState,
}

impl State {
    pub async fn init(
        window: &Window,
        vertices: VertexList,
        radiance_settings: RadianceSettings,
    ) -> Self {
        let instance = Instance::new(Backends::all());
        let window_size = window.inner_size();
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("No adapter");

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::empty(),
                    limits: Limits {
                        max_bind_groups: 5,
                        ..Default::default()
                    },
                },
                None,
            )
            .await
            .expect("No device");

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window_size.width,
            height: window_size.height,
            present_mode: PresentMode::Fifo,
        };
        surface.configure(&device, &config);
        let size = vector![config.width, config.height];

        let global_uniforms =
            UniformData::new(&device, false, ShaderStages::all(), GlobalUniforms {
                window_size: vector![window_size.width as f32, window_size.height as f32],
            });

        let fullscreen_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[vector![4.0_f32, -2.0], vector![-2.0, 4.0], vector![
                -2.0, -2.0
            ]]),
            usage: BufferUsages::VERTEX,
        });

        let fullscreen_vert =
            device.create_shader_module(&include_wgsl!("shaders/fullscreen.vert.wgsl"));

        let intermediate_state = IntermediateState {
            instance: &instance,
            surface: &surface,
            adapter: &adapter,
            device: &device,
            queue: &queue,
            config: &config,
            size,
            global_uniforms: &global_uniforms,
            fullscreen_buffer: &fullscreen_buffer,
            fullscreen_vert: &fullscreen_vert,
        };

        let prerender_state = PrerenderState::new(intermediate_state, vertices);
        let radiance_state =
            RadianceState::new(intermediate_state, &prerender_state, radiance_settings);
        let render_state = RenderState::new(intermediate_state, &prerender_state, &radiance_state);

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            global_uniforms,
            fullscreen_buffer,
            prerender_state,
            radiance_state,
            render_state,
        }
    }
    pub fn render(&mut self) {
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        PrerenderState::render(self, &mut encoder);
        RadianceState::render(self, &mut encoder);

        let output = self.surface.get_current_texture().unwrap();

        RenderState::render(self, &mut encoder, &output);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
