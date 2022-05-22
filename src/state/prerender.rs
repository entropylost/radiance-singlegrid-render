use nalgebra::Vector2;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;

use super::{IntermediateState, State};
use crate::texture::TextureWithView;
use crate::vertex::{Vertex, VertexList};

#[derive(Debug)]
pub struct PrerenderTextures {
    pub albedo_lin: TextureWithView,
    pub radiance_lin: TextureWithView, // Shares alpha with albedo.
    pub normal: TextureWithView,
}

impl PrerenderTextures {
    pub fn new(device: &Device, size: Vector2<u32>) -> Self {
        Self {
            albedo_lin: TextureWithView::create_render_attachment(
                device,
                size,
                TextureFormat::Rgba32Float,
            ),
            radiance_lin: TextureWithView::create_with_usage(
                device,
                size,
                TextureFormat::Rgba32Float, // Alpha unused
                TextureUsages::TEXTURE_BINDING
                    | TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::COPY_SRC,
            ),
            normal: TextureWithView::create_render_attachment(
                device,
                size,
                TextureFormat::Rg32Float,
            ),
        }
    }
    pub fn attachments(&self) -> [RenderPassColorAttachment; 3] {
        [
            self.albedo_lin.attachment(),
            self.radiance_lin.attachment(),
            self.normal.attachment(),
        ]
    }
}

#[derive(Debug)]
pub struct PrerenderState {
    vertices: VertexList,
    vertex_buffer: Buffer,
    pub prerender_textures: PrerenderTextures,
    prerender_pipeline: RenderPipeline,
    pub prerender_output_bind_group_layout: BindGroupLayout,
    pub prerender_output_bind_group: BindGroup,
}

impl PrerenderState {
    pub fn new(st: IntermediateState, vertices: VertexList) -> Self {
        let vertex_buffer = st.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertices.to_bytes(),
            usage: BufferUsages::VERTEX,
        });

        let prerender_textures = PrerenderTextures::new(st.device, st.size);

        let prerender_pipeline_layout =
            st.device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&st.global_uniforms.bind_group_layout],
                push_constant_ranges: &[],
            });

        let prerender_pipeline = st.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&prerender_pipeline_layout),
            vertex: VertexState {
                module: &st
                    .device
                    .create_shader_module(&include_wgsl!("../shaders/prerender.vert.wgsl")),
                entry_point: "main",
                buffers: &[Vertex::layout()],
            },
            fragment: Some(FragmentState {
                module: &st
                    .device
                    .create_shader_module(&include_wgsl!("../shaders/prerender.frag.wgsl")),
                entry_point: "main",
                targets: &[
                    ColorTargetState {
                        format: TextureFormat::Rgba32Float,
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    },
                    ColorTargetState {
                        format: TextureFormat::Rgba32Float,
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    },
                    ColorTargetState {
                        format: TextureFormat::Rg32Float,
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    },
                ],
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

        let prerender_output_bind_group_layout =
            st.device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                            ty: BindingType::Texture {
                                sample_type: TextureSampleType::Float { filterable: false },
                                view_dimension: TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                            ty: BindingType::Texture {
                                sample_type: TextureSampleType::Float { filterable: false },
                                view_dimension: TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 2,
                            visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                            ty: BindingType::Texture {
                                sample_type: TextureSampleType::Float { filterable: false },
                                view_dimension: TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                    ],
                });

        let prerender_output_bind_group = st.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &prerender_output_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&prerender_textures.albedo_lin.1),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&prerender_textures.radiance_lin.1),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(&prerender_textures.normal.1),
                },
            ],
        });

        Self {
            vertices,
            vertex_buffer,
            prerender_textures,
            prerender_pipeline,
            prerender_output_bind_group_layout,
            prerender_output_bind_group,
        }
    }

    pub fn render(st: &mut State, encoder: &mut CommandEncoder) {
        let mut prerender_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &st.prerender_state.prerender_textures.attachments(),
            depth_stencil_attachment: None,
        });

        prerender_pass.set_pipeline(&st.prerender_state.prerender_pipeline);
        prerender_pass.set_vertex_buffer(0, st.prerender_state.vertex_buffer.slice(..));
        prerender_pass.set_bind_group(0, &st.global_uniforms.bind_group, &[]);
        prerender_pass.draw(0..st.prerender_state.vertices.len(), 0..1);
    }
}
