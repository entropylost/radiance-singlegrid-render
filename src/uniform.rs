use bytemuck::{bytes_of, Pod, Zeroable};
use nalgebra::Vector2;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;

#[derive(Debug)]
pub struct UniformData<T: Clone + Copy + Pod + Zeroable> {
    pub data: T,
    pub buffer: Buffer,
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
}

impl<T: Clone + Copy + Pod + Zeroable> UniformData<T> {
    pub fn new(device: &Device, update: bool, visibility: ShaderStages, data: T) -> Self {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytes_of(&data),
            usage: BufferUsages::UNIFORM
                | (if update {
                    BufferUsages::COPY_DST
                } else {
                    BufferUsages::empty()
                }),
        });
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None, // TODO: What does this mean?
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        Self {
            data,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GlobalUniforms {
    pub window_size: Vector2<f32>,
}
