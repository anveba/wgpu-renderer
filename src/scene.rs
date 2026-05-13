use crate::{
    graphics::Material,
    resources::{MeshHandle, MeshResource},
};

mod transform;

pub use transform::*;

pub struct Instance<'a> {
    mesh: MeshHandle<'a>,
    material: Material<'a>,
    transform: Transform,
}

impl<'a> Instance<'a> {
    pub fn mesh_resource(&self) -> &MeshResource {
        self.mesh.get_resource()
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }
}

pub struct Scene<'a> {
    instances: Vec<Instance<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
        }
    }

    pub fn add_instance(
        &mut self,
        mesh: MeshHandle<'a>,
        material: Material<'a>,
        transform: Transform,
    ) {
        self.instances.push(Instance {
            mesh,
            material,
            transform,
        });
    }

    pub fn instances(&self) -> &[Instance] {
        self.instances.as_slice()
    }
}
