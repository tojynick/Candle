use std::env;

use egui_wgpu::wgpu::{self, BindGroup, BindGroupLayout, Device, Sampler, TextureView};
use image::GenericImageView;

pub struct Texture {
    pub name: &'static str,
    pub view: TextureView,
    pub sampler: Sampler,
    pub bind_group_layout: Option<BindGroupLayout>,
}

impl Texture {

    pub fn new(path: &'static str, texture_name: &'static str, device: &Device, queue: &wgpu::Queue) -> Self {

        let path = env!("CARGO_MANIFEST_DIR").to_owned() + "/src/resources/" + path;

        let bytes = std::fs::read(path).unwrap();
        let image = image::load_from_memory(bytemuck::cast_slice(&bytes)).unwrap();
        let rgba = image.to_rgba8();

        let dimensions = image.dimensions();

        let texture_size = wgpu::Extent3d{
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1
        };

        let texture_descriptor = &wgpu::TextureDescriptor {
            label: Some(texture_name),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        let texture = device.create_texture(texture_descriptor);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler_descriptor = &wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        };
        let sampler = device.create_sampler(sampler_descriptor);


        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        Self {
            name: texture_name,
            view: texture_view,
            sampler,
            bind_group_layout: None,
        }
       
    }

    pub fn create_bind_group(&mut self, device: &Device) -> BindGroup {

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
        

        

        let bind_group_name = format!("{} bind group", self.name);

        let diffuse_bind_group_descriptor = &wgpu::BindGroupDescriptor {

            label: Some(bind_group_name.as_str()),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        };

        let bind_group = device.create_bind_group(diffuse_bind_group_descriptor);
        self.bind_group_layout = Some(texture_bind_group_layout);

        return bind_group;
    }
}