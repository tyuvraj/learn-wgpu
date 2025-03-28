use wgpu::{IndexFormat, PrimitiveTopology, ShaderSource};

use winit::{
    event::{Event, WindowEvent,},
    event_loop::EventLoop,
    window::Window,
};

use wgpu_gp::helpers as ws;

pub struct Inputs<'a> {
    pub source: ShaderSource<'a>,
    pub topology: PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
}

impl Inputs<'_> {
    pub async fn new(&mut self, event_loop: EventLoop<()>, window: Window, 
    num_vertices: u32) {
        let init = ws::IWgpuInit::new(&window, 1, None).await;
        let shader = init.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("simple shader"),
            source: self.source.clone(),
        });
        let pipeline_layout = init.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("simple pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let mut ppl = ws::IRenderPipeline {
            shader: Some(&shader),
            pipeline_layout: Some(&pipeline_layout),
            is_depth_stencil: false,
            topology: self.topology,
            strip_index_format: self.strip_index_format,
            ..Default::default()
        };

    let render_pipeline = ppl.new(&init);

    let _ = event_loop.run(
        move |event, elwt| {
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    log::info!("Window closed");
                    elwt.exit();
                },
                Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                    let frame = init.surface.get_current_texture().unwrap();
                    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = init.device.create_command_encoder(
                        &wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder")
                        }
                    );
                    {
                        let color_attachment = ws::create_color_attachment(&view);
                        let mut rpass = encoder.begin_render_pass(
                            &wgpu::RenderPassDescriptor {
                                label: None,
                                color_attachments: &[Some(color_attachment)],
                                depth_stencil_attachment: None,
                                occlusion_query_set: None,
                                timestamp_writes: None,
                            }
                        );
                        rpass.set_pipeline(&render_pipeline);
                        rpass.draw(0..num_vertices, 0..1);
                    }
                    init.queue.submit(std::iter::once(encoder.finish()));
                    frame.present();
                }
                _ => (),
            }
        }
    );
        
    }
}