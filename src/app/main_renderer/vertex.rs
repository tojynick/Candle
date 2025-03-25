use egui_wgpu::wgpu::{VertexAttribute, VertexBufferLayout, VertexStepMode};


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
   pub fn get_buffer_layout() -> VertexBufferLayout<'static> {

        VertexBufferLayout {

            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {

                    format: egui_wgpu::wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0
                },
                VertexAttribute {
                    format: egui_wgpu::wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as u64,
                    shader_location: 1
                }
            ]
        }
   }
}