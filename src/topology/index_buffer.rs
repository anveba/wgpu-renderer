use wgpu::util::DeviceExt;

pub struct IndexBuffer {
    buffer: wgpu::Buffer,
}

impl<'a> IndexBuffer {
    pub fn new(buffer: &[u32], label: Option<&str>, device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: label,
            contents: unsafe {
                std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u8,
                    buffer.len() * std::mem::size_of::<u32>(),
                )
            },
            usage: wgpu::BufferUsages::INDEX,
        });
        IndexBuffer { buffer }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn format(&self) -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint32
    }
}
