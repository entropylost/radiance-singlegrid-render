use bitflags::bitflags;
use bytemuck::{bytes_of, cast_slice, Pod, Zeroable};
use nalgebra::Vector2;
use palette::LinSrgb;
use std::f32::consts::TAU;
use std::mem::size_of;
use std::num::NonZeroU32;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;

use super::prerender::PrerenderState;
use super::{IntermediateState, State};
use crate::texture::TextureWithView;
use crate::uniform::UniformData;

const RADIANCE_WORKGROUP_SIZE: u32 = 16;
const MAX_LIGHT_DIRECTIONS: usize = 64;

#[derive(Debug)]
pub struct RadianceTextures {
    pub directional_radiance: TextureWithView,
    pub total_radiance: TextureWithView,
}

impl RadianceTextures {
    pub fn new(device: &Device, light_directions: u32, size: Vector2<u32>) -> Self {
        let directional_radiance_texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: light_directions,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R32Uint,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
        });
        let directional_radiance_texture_view =
            directional_radiance_texture.create_view(&TextureViewDescriptor {
                dimension: Some(TextureViewDimension::D2Array),
                array_layer_count: Some(NonZeroU32::new(light_directions).unwrap()),
                ..Default::default()
            });
        Self {
            directional_radiance: TextureWithView(
                directional_radiance_texture,
                directional_radiance_texture_view,
            ),
            total_radiance: TextureWithView::create_with_usage(
                device,
                size,
                TextureFormat::Rgba32Float,
                TextureUsages::TEXTURE_BINDING
                    | TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::COPY_DST,
            ),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct RadianceCollectUniforms {
    light_directions: u32,
}

bitflags! {
    #[repr(C)]
    #[derive(Pod, Zeroable, Default)]
    struct RadianceDirectionFlags: u32 {
        const REVERSE_DIRECTION = 0b01;
        const VERTICAL_TRACING = 0b10;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct RadianceDirectionalUniforms {
    slope: f32,
    flags: RadianceDirectionFlags,
    _padding: [u32; 2],
    starting_radiance: LinSrgb,
    _padding_2: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct RadianceUniforms {
    inv_light_directions: f32,
    _padding: [u32; 3],
    directional_uniforms: [RadianceDirectionalUniforms; MAX_LIGHT_DIRECTIONS],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct Workgroup {
    offset: i32,
    light_direction_index: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct RadianceSettings {
    pub light_directions: u32,
    pub light_bounces: u32,
    pub temporal_accumulate: bool,
}

#[derive(Debug)]
pub struct RadianceState {
    light_bounces: u32,
    temporal_accumulate: bool,
    radiance_textures: RadianceTextures,
    radiance_collect_uniforms: UniformData<RadianceCollectUniforms>,
    radiance_collect_bind_group: BindGroup,
    radiance_collect_pipeline: RenderPipeline,
    radiance_uniforms_bind_group: BindGroup,
    num_workgroups: u32,
    pub radiance_bind_group_layout: BindGroupLayout,
    pub radiance_bind_group: BindGroup,
    radiance_pipeline: ComputePipeline,
}

impl RadianceState {
    pub fn new(
        st: IntermediateState,
        prerender_state: &PrerenderState,
        settings: RadianceSettings,
    ) -> Self {
        assert!(settings.light_bounces >= 1);

        let radiance_textures =
            RadianceTextures::new(st.device, settings.light_directions, st.size);

        let radiance_collect_uniforms = UniformData::new(
            st.device,
            false,
            ShaderStages::COMPUTE | ShaderStages::FRAGMENT,
            RadianceCollectUniforms {
                light_directions: settings.light_directions,
            },
        );

        let radiance_collect_bind_group_layout =
            st.device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Uint,
                            view_dimension: TextureViewDimension::D2Array,
                            multisampled: false,
                        },
                        count: None,
                    }],
                });
        let radiance_collect_bind_group = st.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &radiance_collect_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&radiance_textures.directional_radiance.1),
            }],
        });

        let radiance_collect_pipeline_layout =
            st.device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &st.global_uniforms.bind_group_layout,
                    &radiance_collect_uniforms.bind_group_layout,
                    &prerender_state.prerender_output_bind_group_layout,
                    &radiance_collect_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let radiance_collect_pipeline =
            st.device.create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&radiance_collect_pipeline_layout),
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
                    module: &st.device.create_shader_module(&include_wgsl!(
                        "../shaders/radiance_collect.frag.wgsl"
                    )),
                    entry_point: "main",
                    targets: &[ColorTargetState {
                        format: TextureFormat::Rgba32Float,
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

        let (radiance_uniforms, workgroups) =
            Self::compute_radiance_uniforms(st, settings.light_directions);

        let radiance_uniform_buffer = st.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytes_of(&radiance_uniforms),
            usage: BufferUsages::UNIFORM,
        });

        let workgroups_buffer = st.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(&workgroups),
            usage: BufferUsages::STORAGE,
        });

        let radiance_uniforms_bind_group_layout =
            st.device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        let radiance_uniforms_bind_group = st.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &radiance_uniforms_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: radiance_uniform_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: workgroups_buffer.as_entire_binding(),
                },
            ],
        });

        let radiance_bind_group_layout =
            st.device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::COMPUTE | ShaderStages::FRAGMENT,
                            ty: BindingType::Texture {
                                sample_type: TextureSampleType::Float { filterable: false },
                                view_dimension: TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::StorageTexture {
                                access: StorageTextureAccess::WriteOnly,
                                format: TextureFormat::R32Uint,
                                view_dimension: TextureViewDimension::D2Array,
                            },
                            count: None,
                        },
                    ],
                });

        let radiance_bind_group = st.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &radiance_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&radiance_textures.total_radiance.1),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(
                        &radiance_textures.directional_radiance.1,
                    ),
                },
            ],
        });

        let radiance_pipeline_layout =
            st.device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &st.global_uniforms.bind_group_layout,
                    &radiance_uniforms_bind_group_layout,
                    &prerender_state.prerender_output_bind_group_layout,
                    &radiance_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let radiance_pipeline = st
            .device
            .create_compute_pipeline(&ComputePipelineDescriptor {
                label: None,
                layout: Some(&radiance_pipeline_layout),
                module: &st
                    .device
                    .create_shader_module(&include_wgsl!("../shaders/radiance.comp.wgsl")),
                entry_point: "main",
            });

        Self {
            light_bounces: settings.light_bounces,
            temporal_accumulate: settings.temporal_accumulate,
            radiance_textures,
            radiance_collect_uniforms,
            radiance_collect_bind_group,
            radiance_collect_pipeline,
            radiance_uniforms_bind_group,
            num_workgroups: workgroups.len() as u32,
            radiance_bind_group_layout,
            radiance_bind_group,
            radiance_pipeline,
        }
    }

    fn compute_radiance_uniforms(
        st: IntermediateState,
        light_directions: u32,
    ) -> (RadianceUniforms, Vec<Workgroup>) {
        let mut workgroups = Vec::new();
        let mut radiance_uniforms = RadianceUniforms {
            inv_light_directions: 1.0 / (light_directions as f32),
            _padding: [0; 3],
            directional_uniforms: [Default::default(); MAX_LIGHT_DIRECTIONS],
        };
        for i in 0..light_directions {
            let direction = i as f32 / light_directions as f32 - 0.5;

            // +----x  -0.375   -0.25   -0.125
            // |            *.r+v | v-r.*
            // |          +r  *.  |  .*   -
            // y                *.|.*
            //        +-0.5 ------+------ 0
            // +----y           .*|*.
            // |          -r  .*  |  *.   +
            // | (v)        .* -v | v+ *.
            // x         0.375  0.25   0.125

            let is_vertical_tracing = match direction.abs() {
                x if x <= 0.125 => false,
                x if x <= 0.375 => true,
                _ => false,
            };
            let is_direction_reversed = !(-0.125..=0.375).contains(&direction);
            let slope = (match direction {
                x if (-0.125..=0.125).contains(&x) => x,
                x if (0.125..=0.375).contains(&x) => -(x - 0.25),
                x if (-0.375..=-0.125).contains(&x) => -(x + 0.25),
                x if (0.375..=0.501).contains(&x) => x - 0.5,
                x if (-0.501..=-0.375).contains(&x) => x + 0.5,
                _ => unreachable!(),
            } * TAU)
                .tan();
            let mut flags = RadianceDirectionFlags::empty();
            flags.set(
                RadianceDirectionFlags::VERTICAL_TRACING,
                is_vertical_tracing,
            );
            flags.set(
                RadianceDirectionFlags::REVERSE_DIRECTION,
                is_direction_reversed,
            );
            let axies = if flags.contains(RadianceDirectionFlags::VERTICAL_TRACING) {
                st.size.yx()
            } else {
                st.size
            };
            let offset = -slope * axies.x as f32;
            let offset = (offset + offset.signum() * 0.999) as i32;
            let total_size = offset.abs() as u32 + axies.y;
            let offset = offset.min(0);
            radiance_uniforms.directional_uniforms[i as usize] = RadianceDirectionalUniforms {
                slope,
                flags,
                _padding: [0; 2],
                starting_radiance: LinSrgb::new(0.0, 0.0, 0.0),
                _padding_2: 0,
            };
            let num_workgroups =
                (total_size + RADIANCE_WORKGROUP_SIZE - 1) / RADIANCE_WORKGROUP_SIZE;
            for wg in 0..num_workgroups {
                workgroups.push(Workgroup {
                    offset: offset + (RADIANCE_WORKGROUP_SIZE * wg) as i32,
                    light_direction_index: i,
                });
            }
        }
        (radiance_uniforms, workgroups)
    }

    pub fn render(st: &mut State, encoder: &mut CommandEncoder) {
        if !st.radiance_state.temporal_accumulate {
            encoder.copy_texture_to_texture(
                ImageCopyTexture {
                    texture: &st.prerender_state.prerender_textures.radiance_lin.0,
                    mip_level: 0,
                    origin: Origin3d::ZERO,
                    aspect: TextureAspect::All,
                },
                ImageCopyTexture {
                    texture: &st.radiance_state.radiance_textures.total_radiance.0,
                    mip_level: 0,
                    origin: Origin3d::ZERO,
                    aspect: TextureAspect::All,
                },
                Extent3d {
                    width: st.size.x,
                    height: st.size.y,
                    depth_or_array_layers: 1,
                },
            );
        }

        for _ in 0..st.radiance_state.light_bounces {
            let mut radiance_pass =
                encoder.begin_compute_pass(&ComputePassDescriptor { label: None });

            radiance_pass.set_pipeline(&st.radiance_state.radiance_pipeline);
            radiance_pass.set_bind_group(0, &st.global_uniforms.bind_group, &[]);
            radiance_pass.set_bind_group(1, &st.radiance_state.radiance_uniforms_bind_group, &[]);
            radiance_pass.set_bind_group(2, &st.prerender_state.prerender_output_bind_group, &[]);
            radiance_pass.set_bind_group(3, &st.radiance_state.radiance_bind_group, &[]);
            radiance_pass.dispatch(st.radiance_state.num_workgroups, 1, 1);

            drop(radiance_pass);

            let mut radiance_collect_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[st
                    .radiance_state
                    .radiance_textures
                    .total_radiance
                    .attachment()],
                depth_stencil_attachment: None,
            });

            radiance_collect_pass.set_pipeline(&st.radiance_state.radiance_collect_pipeline);
            radiance_collect_pass.set_vertex_buffer(0, st.fullscreen_buffer.slice(..));
            radiance_collect_pass.set_bind_group(0, &st.global_uniforms.bind_group, &[]);
            radiance_collect_pass.set_bind_group(
                1,
                &st.radiance_state.radiance_collect_uniforms.bind_group,
                &[],
            );
            radiance_collect_pass.set_bind_group(
                2,
                &st.prerender_state.prerender_output_bind_group,
                &[],
            );
            radiance_collect_pass.set_bind_group(
                3,
                &st.radiance_state.radiance_collect_bind_group,
                &[],
            );
            radiance_collect_pass.draw(0..3, 0..1);
        }
    }
}
