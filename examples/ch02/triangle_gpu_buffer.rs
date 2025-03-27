use bytemuck::{cast_slice, Pod, Zeroable};
use std::{iter, mem};
use wgpu::{ util::DeviceExt, VertexBufferLayout
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use wgpu_gp::helpers as ws;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5],
        color: [0.0, 0.0, 1.0],
    },
];

struct State <'a> {
    init: ws::IWgpuInit<'a>,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    window: &'a Window,
}

impl<'a> State<'a> {
    async  fn new(window: &'a Window) -> Self {
        let init = ws::IWgpuInit::new(&window, 1, None)
            .await;
        let shader = init.device.create_shader_module(
            wgpu::include_wgsl!("triangle_gpu_buffer.wgsl")
        );
        let pipeline_layout = init.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }
        );
        
        let vertex_buffer_layout = VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![
                0 => Float32x2,
                1 => Float32x3,
            ],
        };

        let mut ppl = ws::IRenderPipeline {
            shader: Some(&shader),
            pipeline_layout: Some(&pipeline_layout),
            is_depth_stencil: false,
            vertex_buffer_layout: &[vertex_buffer_layout],
            ..Default::default()
        };

        let pipeline = ppl.new(&init);

        let vertex_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            init,
            pipeline,
            vertex_buffer,
            window
        }

    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.init.resize(new_size);
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError>{
        let output = self.init.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.init.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder")
            }
        );
        {
            let color_attachment = ws::create_color_attachment(&view);
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..VERTICES.len() as u32, 0..1);
        }
        self.init.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window  = Window::new(&event_loop).unwrap();
    window.set_title("Chapter 02: Vertex buffer ");
    env_logger::init();
    
    event_loop.set_control_flow(ControlFlow::Poll);
    
    let mut state = pollster::block_on(State::new(&window));

    let _ = event_loop.run(
        move |event, elwt| {
            match event {
                Event::WindowEvent { ref event, window_id  } if window_id == state.window.id() => {
                    if !state.input(event) {
                        match event {
                            WindowEvent::Resized(new_size) => {
                                state.resize(*new_size);
                            }
                            WindowEvent::CloseRequested => {
                                elwt.exit();
                            }
                            WindowEvent::RedrawRequested => {
                                state.update();
                                match state.render() {
                                    Ok(_) => {}
                                    // Reconfigure the surface if it's lost or outdated
                                    Err(
                                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                    ) => state.resize(state.init.size),
                                    // The system is out of memory, we should probably quit
                                    Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                                        log::error!("OutOfMemory");
                                        elwt.exit();
                                    }
                    
                                    // This happens when the a frame takes too long to present
                                    Err(wgpu::SurfaceError::Timeout) => {
                                        log::warn!("Surface timeout")
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                },
                _ => (),
            }
        }
    );
}

