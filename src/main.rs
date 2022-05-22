#![feature(int_roundings)]

use nalgebra::vector;
use palette::{Srgb, Srgba};
use state::radiance::RadianceSettings;
use state::State;
use std::time::{Duration, Instant};
use vertex::{Vertex, VertexList};
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event_loop::EventLoop;

mod state;
mod texture;
mod uniform;
mod vertex;

fn vertices() -> VertexList {
    let black_alpha = Srgba::new(0.0, 0.0, 0.0, 0.01).into();
    let black = Srgb::new(0.0, 0.0, 0.0);
    let dim = Srgb::new(0.1, 0.1, 0.1);
    let bright = Srgb::new(1.0, 1.0, 1.0);

    let mut vertices = VertexList::new();
    vertices
        .triangle([
            Vertex {
                position: vector![0.0, 0.0],
                normal: vector![0.0, 0.0],
                albedo: Srgba::new(1.0, 1.0, 1.0, 0.01).into(),
                radiance: black_alpha,
            },
            Vertex {
                position: vector![0.0, 4000.0],
                normal: vector![0.0, 0.0],
                albedo: Srgba::new(1.0, 1.0, 1.0, 0.01).into(),
                radiance: black_alpha,
            },
            Vertex {
                position: vector![4000.0, 0.0],
                normal: vector![0.0, 0.0],
                albedo: Srgba::new(1.0, 1.0, 1.0, 0.01).into(),
                radiance: black_alpha,
            },
        ])
        .rectangle(
            vector![300.0, 400.0],
            vector![20.0, 140.0],
            Srgb::new(0.5, 0.5, 0.5),
            black,
        )
        .rectangle(
            vector![300.0, 200.0],
            vector![200.0, 20.0],
            Srgb::new(0.9, 0.1, 0.1),
            black,
        )
        .rectangle(
            vector![0.0, 0.0],
            vector![2000.0, 10.0],
            Srgb::new(0.0, 0.0, 0.0),
            dim,
        )
        .rectangle(
            vector![0.0, 0.0],
            vector![10.0, 2000.0],
            Srgb::new(0.0, 0.0, 0.0),
            bright,
        )
        .rectangle(
            vector![800.0, 600.0],
            vector![2000.0, 10.0],
            Srgb::new(0.0, 0.0, 0.0),
            dim,
        )
        .rectangle(
            vector![800.0, 600.0],
            vector![10.0, 2000.0],
            Srgb::new(0.0, 0.0, 0.0),
            dim,
        );
    vertices
}

async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_min_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut state = State::init(&window, vertices(), RadianceSettings {
        light_directions: 32,
        light_bounces: 3,
        temporal_accumulate: false,
    })
    .await;

    let mut frame_time = Duration::from_millis(100);

    event_loop.run(move |event, _, _| match event {
        Event::RedrawRequested(..) => {
            let before = Instant::now();
            state.render();
            let after = Instant::now();
            let delta = after - before;
            frame_time = frame_time.mul_f32(0.9) + delta.mul_f32(0.1);
            let fps = 1.0 / frame_time.as_secs_f32();
            println!("Fps: {:?}", fps);
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}

fn main() {
    pollster::block_on(run());
}
