use std::iter;
use winit::{
    dpi::PhysicalSize, event::{Event, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop}, window::Window
};
use wgpu_gp::helpers as ws;

struct State <'a> {
    init: ws::IWgpuInit<'a>,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    window: &'a Window,
}

impl<'a> State<'a> {
    async  fn new(window: &'a Window) -> Self {
        let mut init = ws::IWgpuInit::new(&window, 1, None)
            .await;

        let diffuse_bytes = include_bytes!("happy-tree.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let image_texture_view = ws::create_image_texture_view(&init, &diffuse_image);
        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();
        println!("Image dimensions: {:?}", dimensions);
        let _ = window.request_inner_size(PhysicalSize::new(dimensions.0, dimensions.1));
        let image_sampler = ws::create_default_sampler(&init);
        
        let shader = init.device.create_shader_module(
            wgpu::include_wgsl!("imshow.wgsl")
        );

        let bind_group_layout = init.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some(
                    "Bind Group Layout"
                ),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None
                    }
                ]
            }
        );

        let bind_group = init.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&image_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&image_sampler),
                    }
                ],
                label: Some("Bind Group")
            }
        );

        let pipeline_layout = init.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            }
        );

        let mut ppl = ws::IRenderPipeline {
            shader: Some(&shader),
            pipeline_layout: Some(&pipeline_layout),
            is_depth_stencil: false,
            vertex_buffer_layout: &[],
            ..Default::default()
        };

        let pipeline = ppl.new(&init);

        Self {
            init,
            pipeline,
            bind_group,
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
            render_pass.set_bind_group(0, &self.bind_group, &[]); // NEW!
            render_pass.draw(0..6 as u32, 0..1);
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

