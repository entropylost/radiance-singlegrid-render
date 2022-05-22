use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use generational_arena::{Arena, Index};
use nalgebra::{vector, Vector2};
use palette::{Alpha, IntoColor, LinSrgb, LinSrgba, Srgb, Srgba};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub albedo: Srgba,
    pub render_albedo: Srgba,
    pub radiance: Srgb,
    pub render_radiance: Srgb,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Pod, Zeroable)]
pub struct RawMaterial {
    pub albedo: LinSrgba,
    pub render_albedo: LinSrgba,
    pub radiance: LinSrgba,
    pub render_radiance: LinSrgba,
}

impl From<Material> for RawMaterial {
    fn from(material: Material) -> Self {
        let albedo = material.albedo.into();
        let render_albedo = material.render_albedo.into();
        Self {
            albedo,
            render_albedo,
            radiance: Alpha {
                color: material.radiance.into(),
                alpha: albedo.alpha,
            },
            render_radiance: Alpha {
                color: material.radiance.into(),
                alpha: render_albedo.alpha,
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FullMaterial {
    pub external: Material,
    pub internal: Material,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Pod, Zeroable)]
pub struct FullRawMaterial {
    pub external: RawMaterial,
    pub internal: RawMaterial,
}

impl From<FullMaterial> for FullRawMaterial {
    fn from(material: FullMaterial) -> Self {
        Self {
            external: material.external.into(),
            internal: material.internal.into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: Vector2<f32>,
    pub normal: Vector2<f32>,
    pub material_id: u32,
}

impl Vertex {
    const ATTRIBUTES: [VertexAttribute; 3] =
        vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32];

    pub(crate) fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VertexList {
    triangles: Arena<[Vertex; 3]>,
    materials: Arena<FullRawMaterial>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriangleHandle(Index);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialHandle(Index);

impl VertexList {
    pub fn new(capacity: usize) -> Self {
        Self {
            triangles: Arena::with_capacity(capacity),
            materials: Arena::with_capacity(capacity),
        }
    }
    pub fn triangle(&mut self, triangle: [Vertex; 3]) -> TriangleHandle {
        self.triangles.push(triangle);
        self
    }
    pub fn parallelogram(
        &mut self,
        c: Vector2<f32>,
        x: Vector2<f32>,
        y: Vector2<f32>,
        albedo: impl IntoColor<LinSrgba>,
        radiance: impl IntoColor<LinSrgb>,
    ) -> [TriangleHandle; 4] {
        let albedo = albedo.into_color();
        let radiance = Alpha {
            color: radiance.into_color(),
            alpha: albedo.alpha,
        };
        let a = x + y;
        let b = x - y;
        let ny = vector![y.y, -y.x].normalize();
        let nx = vector![-x.y, x.x].normalize();
        let tri = |a, b, normal| {
            [
                Vertex {
                    position: c,
                    normal,
                    albedo,
                    radiance,
                },
                Vertex {
                    position: c + a,
                    normal,
                    albedo,
                    radiance,
                },
                Vertex {
                    position: c + b,
                    normal,
                    albedo,
                    radiance,
                },
            ]
        };
        self.triangle(tri(b, a, ny))
            .triangle(tri(a, -b, nx))
            .triangle(tri(-b, -a, -ny))
            .triangle(tri(-a, b, -nx));
        self
    }
    pub fn rectangle(
        &mut self,
        c: Vector2<f32>,
        a: Vector2<f32>,
        albedo: impl IntoColor<LinSrgba>,
        radiance: impl IntoColor<LinSrgb>,
    ) -> &mut Self {
        self.parallelogram(c, vector![a.x, 0.0], vector![0.0, a.y], albedo, radiance)
    }
    pub fn to_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.triangles)
    }
    pub fn len(&self) -> u32 {
        self.triangles.len() as u32 * 3
    }
}
