use nalgebra::Vector2;
use std::mem::size_of;
use wgpu::*;

use super::prerender::PrerenderState;
use super::radiance::RadianceState;
use super::{IntermediateState, State};

#[derive(Debug)]
pub struct RenderState {
    render_pipeline: RenderPipeline,
}

impl RenderState {
    pub fn new(
        st: IntermediateState,
        _prerender_state: &PrerenderState,
        radiance_state: &RadianceState,
    ) -> Self {
        let render_pipeline_layout = st.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &st.global_uniforms.bind_group_layout,
                &radiance_state.radiance_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = st.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: st.fullscreen_vert,
                entry_point: "main",
                buffers: &[VertexBufferLayout {
                    array_stride: size_of::<Vector2<f32>>() as BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &vertex_attr_array![0 => Float32x2],
                }],
            },
            fragment: Some(FragmentState {
                module: &st
                    .device
                    .create_shader_module(&include_wgsl!("../shaders/render.frag.wgsl")),
                entry_point: "main",
                targets: &[ColorTargetState {
                    format: st.config.format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                }],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self { render_pipeline }
    }

    pub fn render(st: &mut State, encoder: &mut CommandEncoder, output: &SurfaceTexture) {
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&st.render_state.render_pipeline);
        render_pass.set_vertex_buffer(0, st.fullscreen_buffer.slice(..));
        render_pass.set_bind_group(0, &st.global_uniforms.bind_group, &[]);
        render_pass.set_bind_group(1, &st.radiance_state.radiance_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
