use crate::algebra::{Vec2, Vec3};

use super::Vertex;
use wgpu::{util::DeviceExt, VertexBufferLayout};

pub struct VertexBuffer {
    buffer: wgpu::Buffer,
}

impl VertexBuffer {
    pub fn new(buffer: &[Vertex], label: Option<&str>, device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: unsafe {
                std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u8,
                    buffer.len() * std::mem::size_of::<Vertex>(),
                )
            },
            usage: wgpu::BufferUsages::VERTEX,
        });
        VertexBuffer { buffer }
    }

    pub fn layout() -> VertexBufferLayout<'static> {
        use std::mem::size_of;
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<Vec3>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (size_of::<Vec2>() + size_of::<Vec3>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}
