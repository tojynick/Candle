use winit::event_loop::{ControlFlow, EventLoop};

pub mod app;
pub mod utilities;


fn main() -> anyhow::Result<()>{
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run())?;
        Ok(())
    }
}


async fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = app::App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
