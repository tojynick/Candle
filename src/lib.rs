use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod renderer;

pub fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = app::App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}