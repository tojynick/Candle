use egui_wgpu::wgpu::{self, Adapter, Device, Queue, Surface, SurfaceConfiguration, TextureFormat};

pub async fn get_device(instance: &wgpu::Instance, surface: &wgpu::Surface<'static>) -> (Adapter, Device, Queue) {
    
    let power_pref = wgpu::PowerPreference::HighPerformance;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: power_pref,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let features = wgpu::Features::default();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: features,
                required_limits: Default::default(),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    return (adapter, device, queue);
}


pub fn configure_surface(surface: &Surface, width: u32, height: u32, device: &Device, adapter: &Adapter) -> SurfaceConfiguration {

    let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .expect("Failed to select proper surface texture format!");

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Immediate,
            desired_maximum_frame_latency: 0,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        return surface_config;
}