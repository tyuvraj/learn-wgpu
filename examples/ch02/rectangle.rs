pub mod common;
use common::Inputs;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use std::borrow::Cow;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window  = Window::new(&event_loop).unwrap();
    window.set_title("Chapter 02");
    env_logger::init();
    
    event_loop.set_control_flow(ControlFlow::Poll);
    
    let mut inputs = Inputs {
        source: wgpu::ShaderSource::Wgsl(
            Cow::Borrowed(include_str!("rectangle_vertex_color.wgsl"))
        ),
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
    };

    pollster::block_on(inputs.new(event_loop, window, 6));
}