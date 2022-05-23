use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use nalgebra::{vector, Vector2};
use palette::{Alpha, IntoColor, LinSrgb, LinSrgba};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Pod, Zeroable)]
pub struct Vertex {
    pub position: Vector2<f32>,
    pub normal: Vector2<f32>,
    pub albedo: LinSrgba,
    pub radiance: LinSrgba, // TODO: Make RawVertex types.
}

impl Vertex {
    const ATTRIBUTES: [VertexAttribute; 4] =
        vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4, 3 => Float32x4];

    pub fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VertexList {
    triangles: Vec<[Vertex; 3]>,
}

impl VertexList {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }
    pub fn triangle(&mut self, triangle: [Vertex; 3]) -> &mut Self {
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
    ) -> &mut Self {
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
