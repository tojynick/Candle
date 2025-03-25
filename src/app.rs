use egui_wgpu::wgpu::SurfaceError;
use egui_wgpu::{wgpu, ScreenDescriptor};
use gui_renderer::GUIRenderer;
use std::sync::Arc;
use std::time::Duration;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub mod main_renderer;
pub mod gui_renderer;

use main_renderer::MainRenderer;

use crate::utilities::FPSCounter;


pub struct App {
    wgpu_instance: wgpu::Instance,
    main_renderer: Option<MainRenderer>,
    gui_renderer: Option<GUIRenderer>,
    fps_counter: FPSCounter,
    window: Option<Arc<Window>>,
}

impl App {

    pub fn new() -> Self {

        let wgpu_instance = egui_wgpu::wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        Self {
            wgpu_instance,
            main_renderer: None,
            gui_renderer: None,
            fps_counter: FPSCounter::new(),
            window: None,
        }
    }

    async fn set_window(&mut self, window: Window) {
        
        let window = Arc::new(window);
        let initial_width = 1280;
        let initial_height = 720;

        let _ = window.request_inner_size(LogicalSize::new(initial_width, initial_height));

        let surface = self
            .wgpu_instance
            .create_surface(window.clone())
            .expect("Failed to create surface!");

        let main_renderer = MainRenderer::new(
            &self.wgpu_instance,
            surface,
            initial_width,
            initial_height,
        )
        .await;

        let gui_renderer = GUIRenderer::new(
            &main_renderer.device,
            main_renderer.surface_config.format,
            None,
            1,
            &window
        );

        self.window.get_or_insert(window);
        self.main_renderer.get_or_insert(main_renderer);
        self.gui_renderer.get_or_insert(gui_renderer);
    }


    fn resize(&mut self, width: u32, height: u32) {

        if width > 0 && height > 0 {
            self.main_renderer.as_mut().unwrap().resize_surface(width, height);
        }

    }

    fn redraw(&mut self) {
        
        // Attempt to handle minimizing window
        if let Some(window) = self.window.as_ref() {
            if let Some(min) = window.is_minimized() {
                if min {
                    return;
                }
            }
        }

        let main_renderer = self.main_renderer.as_mut().unwrap();
        let gui_renderer = self.gui_renderer.as_mut().unwrap();

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [main_renderer.surface_config.width, main_renderer.surface_config.height],
            pixels_per_point: self.window.as_ref().unwrap().scale_factor() as f32,
        };

        let surface_texture = main_renderer.surface.get_current_texture();

        match surface_texture {
            Err(SurfaceError::Outdated) => {
                // Ignoring outdated to allow resizing and minimization
                println!("wgpu surface is outdated!");
                return;
            }
            Err(_) => {
                surface_texture.expect("Failed to acquire next swap chain texture!");
                return;
            }
            Ok(_) => {}
        };

        let surface_texture = surface_texture.unwrap();

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = main_renderer
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Encoder") });

        let window = self.window.as_ref().unwrap();

        // Main pass
        {
            main_renderer.render(&mut encoder, &surface_view);
        }

        // GUI pass
        {
            gui_renderer.begin_gui(window);

            gui_renderer.render(self.fps_counter.fps);

            gui_renderer.end_gui(
                &main_renderer.device,
                &main_renderer.queue,
                &mut encoder,
                window,
                &surface_view,
                screen_descriptor,
            );
        }

        main_renderer.queue.submit(Some(encoder.finish()));
        surface_texture.present();

        self.fps_counter.update();
    }
}


impl ApplicationHandler for App {
    
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let window = event_loop
            .create_window(
                Window::default_attributes()
                .with_title("Candle")
            )
            .unwrap();

        pollster::block_on(self.set_window(window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {

        self.gui_renderer.as_mut().unwrap().handle_input(self.window.as_ref().unwrap(), &event);

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                self.redraw();
                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::Resized(new_size) => {
                self.resize(new_size.width, new_size.height);
            }
            _ => (),
        }
    }
}
