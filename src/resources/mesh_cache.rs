use crate::topology::{IndexBuffer, Mesh, VertexBuffer};
use std::collections::HashMap;

pub type MeshId = u64;

pub struct MeshResource {
    label: String,
    id: MeshId,
    mesh: Mesh,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
}

impl MeshResource {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn id(&self) -> MeshId {
        self.id
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn vertex_buffer(&self) -> &VertexBuffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
}

#[derive(Copy, Clone)]
pub struct MeshHandle<'a> {
    id: MeshId,
    cache: &'a MeshCache,
}

impl<'a> MeshHandle<'a> {
    pub fn mesh_id(&self) -> MeshId {
        self.id
    }

    pub fn mesh_cache(&self) -> &MeshCache {
        self.cache
    }

    pub fn get_resource(&self) -> &MeshResource {
        let resource = self.cache.get(self.mesh_id());
        resource.expect("Mesh in mesh handle was not in the referenced cache")
    }
}

pub struct MeshCache {
    meshes: HashMap<MeshId, MeshResource>,
    next_id: MeshId,
}

impl MeshCache {
    pub fn new() -> Self {
        MeshCache {
            meshes: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh, device: &wgpu::Device, label: &str) -> MeshId {
        let id = self.next_id;
        self.next_id += 1;

        let vertex_buffer = mesh.create_vertex_buffer(device);
        let index_buffer = mesh.create_index_buffer(device);

        let mesh_resource = MeshResource {
            label: label.to_string(),
            id,
            mesh,
            vertex_buffer,
            index_buffer,
        };
        self.meshes.insert(id, mesh_resource);

        id
    }

    pub fn get_handle(&self, mesh_id: MeshId) -> MeshHandle {
        MeshHandle {
            id: mesh_id,
            cache: &self,
        }
    }

    pub fn get(&self, id: MeshId) -> Option<&MeshResource> {
        self.meshes.get(&id)
    }

    pub fn get_mut(&mut self, id: MeshId) -> Option<&mut MeshResource> {
        self.meshes.get_mut(&id)
    }
}
