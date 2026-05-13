pub struct Shader {
    shader: wgpu::ShaderModule,
}

impl Shader {
    pub fn from_src(src: &str, device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Test Shader"),
            source: wgpu::ShaderSource::Wgsl(src.into()),
        });
        Self { shader }
    }

    pub fn module(&self) -> &wgpu::ShaderModule {
        &self.shader
    }
}
