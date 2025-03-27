use wgpu_gp::helpers as ws;

use winit::{
    event::{Event, WindowEvent,},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};


async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window  = Window::new(&event_loop).unwrap();
    ws::get_wgpu_info(&window).await;
    window.set_title("WGPU Test");
    env_logger::init();

    event_loop.set_control_flow(ControlFlow::Poll);

    let _ = event_loop.run(
        move |event, elwt| {
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    log::info!("Window closed");
                    elwt.exit();
                },
                // Event::AboutToWait => {
                //     log::info!("About to wait");
                //     window.request_redraw();
                // },
                _ => (),
            }
        }
    );
    
}

fn main() {
    pollster::block_on(run());
}