use wgpu::core::id::DeviceId;
use winit::{
    application::ApplicationHandler, dpi::{LogicalSize, PhysicalPosition}, event::WindowEvent, event_loop::ActiveEventLoop, window::Window
};
use crate::renderer::Renderer;


pub struct App<'a> {
    renderer: Option<Renderer<'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            renderer: None
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {

    // Basically runs on creation
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let window = event_loop.create_window(
            Window::default_attributes()
            .with_title("Candle")
            .with_inner_size(LogicalSize::new(1280, 720))
        ).unwrap();

        self.renderer = Some(Renderer::new(window));
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match event {

            WindowEvent::Resized(new_size) => {
                self.renderer.as_mut().unwrap().resize(new_size);
            }

            WindowEvent::CloseRequested => {
                println!("Bye!");
                event_loop.exit();
            },

            WindowEvent::RedrawRequested => {
                let renderer = self.renderer.as_mut().unwrap();

                match renderer.render() {

                    Ok(()) => {},

                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        renderer.resize(renderer.window_size);
                    },

                    Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                        eprint!("Out of memory!");
                    },

                    Err(wgpu::SurfaceError::Timeout) => {
                        eprint!("Surface timeout!")
                    }
                }
            },

            _ => (),
        }
    }
}