use crate::{scene::Scene, topology::VertexBuffer};

use super::Shader;

pub struct Rasteriser {
    pipeline: wgpu::RenderPipeline,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl Rasteriser {
    pub fn new(
        shader: &Shader,
        target_format: &wgpu::TextureFormat,
        device: &wgpu::Device,
    ) -> Self {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipeline_layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader.module(),
                entry_point: "vs_main",
                buffers: &[VertexBuffer::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader.module(),
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: *target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Rasteriser {
            pipeline: render_pipeline,
            texture_bind_group_layout,
        }
    }

    pub fn draw(
        &self,
        scene: &Scene,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
    ) {
        for instance in scene.instances() {
            let resource = instance.mesh_resource();
            let vertex_buffer = resource.vertex_buffer();
            let index_buffer = resource.index_buffer();
            let material = instance.material();

            let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&material.diffuse().view()),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&material.diffuse().sampler()),
                    },
                ],
                label: Some("diffuse_bind_group"),
            });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_vertex_buffer(0, vertex_buffer.buffer().slice(..));
            render_pass.set_index_buffer(index_buffer.buffer().slice(..), index_buffer.format());
            render_pass.set_bind_group(0, &texture_bind_group, &[]);
            render_pass.draw_indexed(0..resource.mesh().index_count() as u32, 0, 0..1);
        }
    }
}
