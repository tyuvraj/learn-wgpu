pub mod common;
use common::Inputs;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use std::borrow::Cow;

fn main() {
    let mut primitive_type="traingle-list";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        primitive_type = &args[1];
    }
    let mut topology = wgpu::PrimitiveTopology::TriangleList;
    let mut index_format = None;
    match &primitive_type[..] {
        "triangle-strip" => {
            topology = wgpu::PrimitiveTopology::TriangleStrip;
            index_format = Some(wgpu::IndexFormat::Uint32);
        },
        "line-list" => {
            topology = wgpu::PrimitiveTopology::LineList;
        },
        "line-strip" => {
            topology = wgpu::PrimitiveTopology::LineStrip;
        },
        _ => {},
    }
    let event_loop = EventLoop::new().unwrap();
    let window  = Window::new(&event_loop).unwrap();
    window.set_title(&*format!("{}{}", "Chapter 02: primitive type: ", primitive_type));
    env_logger::init();
    
    event_loop.set_control_flow(ControlFlow::Poll);
    
    let mut inputs = Inputs {
        source: wgpu::ShaderSource::Wgsl(
            Cow::Borrowed(include_str!("triangle_primitive.wgsl"))
        ),
        topology,
        strip_index_format: index_format,
    };

    pollster::block_on(inputs.new(event_loop, window, 9));
}