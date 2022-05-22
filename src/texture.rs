use nalgebra::Vector2;
use wgpu::*;

#[derive(Debug)]
pub struct TextureWithView(pub Texture, pub TextureView);

impl TextureWithView {
    pub fn new(texture: Texture) -> Self {
        let view = texture.create_view(&TextureViewDescriptor::default());
        Self(texture, view)
    }

    pub fn create_with_usage(
        device: &Device,
        size: Vector2<u32>,
        format: TextureFormat,
        usage: TextureUsages,
    ) -> Self {
        Self::new(device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage,
        }))
    }

    pub fn create_render_attachment(
        device: &Device,
        size: Vector2<u32>,
        format: TextureFormat,
    ) -> Self {
        Self::create_with_usage(
            device,
            size,
            format,
            TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        )
    }

    pub fn attachment(&self) -> RenderPassColorAttachment {
        RenderPassColorAttachment {
            view: &self.1,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                store: true,
            },
        }
    }
}
