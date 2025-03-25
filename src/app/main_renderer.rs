use egui_wgpu::wgpu::{self, util::DeviceExt, CommandEncoder, TextureView};
use texture::Texture;
use vertex::Vertex;

mod vertex;
mod texture;
mod renderer_utils;

pub struct MainRenderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub render_pipeline: wgpu::RenderPipeline,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub amount_of_vertices: u32,
    pub diffuse_bind_group: wgpu::BindGroup,
}

impl MainRenderer {

    pub async fn new(
        instance: &wgpu::Instance,
        surface: wgpu::Surface<'static>,
        width: u32,
        height: u32,
    ) -> Self {

        let (adapter, device, queue) = renderer_utils::get_device(instance, &surface).await;
        let surface_config = renderer_utils::configure_surface(&surface, width, height, &device, &adapter);

        let mut diffuse_texture = Texture::new("Checker.png", "Diffuse", &device, &queue);
        let diffuse_texture_bind_group = diffuse_texture.create_bind_group(&device);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });


        const VERTICES: &[Vertex] = &[
            Vertex { position: [0.0, 0.5, 0.0], uv: [0.5, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0], uv: [0.0, 1.0] },
            Vertex { position: [0.5, -0.5, 0.0], uv: [1.0, 1.0] },
        ];

        const INDICES: &[u16] = &[
            0, 1, 2
        ];

        let amount_of_vertices = VERTICES.len() as u32;

        let vertex_buffer_description = &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        };

        let vertex_buffer = device.create_buffer_init(vertex_buffer_description);

        let index_buffer_description = &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        };

        let index_buffer = device.create_buffer_init(index_buffer_description);


        let render_pipeline_layout = 
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&diffuse_texture.bind_group_layout.unwrap()],
                push_constant_ranges: &[]
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex",
                buffers: &[Vertex::get_buffer_layout()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
            cache: None
        });

        Self {
            device,
            queue,
            surface,
            surface_config,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            amount_of_vertices,
            diffuse_bind_group: diffuse_texture_bind_group,
        }
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self, encoder: &mut CommandEncoder, surface_view: &TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_view,
                resolve_target: None,
                ops: wgpu::Operations { 
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.5,
                        g: 0.5,
                        b: 0.5,
                        a: 1.0 
                    }),
                    store: wgpu::StoreOp::Store
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.amount_of_vertices, 0, 0..1);
           
    }
}