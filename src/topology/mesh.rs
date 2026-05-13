use super::{IndexBuffer, Vertex, VertexBuffer};

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    pub fn create_vertex_buffer(&self, device: &wgpu::Device) -> VertexBuffer {
        let buffer = self.vertices.as_slice();

        VertexBuffer::new(buffer, None, device)
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn create_index_buffer(&self, device: &wgpu::Device) -> IndexBuffer {
        IndexBuffer::new(self.indices.as_slice(), None, device)
    }

    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}
