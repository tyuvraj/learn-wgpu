use winit::window::Window;

pub struct IWgpuInit<'a> {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'a>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub sample_count: u32,
}

impl <'a> IWgpuInit <'a> {
    pub async fn new(window: &'a Window, sample_count: u32, limits: Option<wgpu::Limits>) -> IWgpuInit<'a> {
        let limits_device = limits.unwrap_or(wgpu::Limits::default());
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface( window).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to fun an appropriate adapter");
        
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::default() | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            required_limits: limits_device,
            memory_hints: wgpu::MemoryHints::Performance,
        }, None).await.expect("Failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);
        let format = surface_caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);


        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            sample_count,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {

        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

pub async fn get_wgpu_info(window: &Window) {
    let init = IWgpuInit::new(&window, 1, None).await;
    println!("{:?}", init.adapter.get_info());
    println!("{:?}", init.adapter.limits());
    println!("{:?}", init.device.limits());
}

pub struct IRenderPipeline<'a> {
    pub shader: Option<&'a wgpu::ShaderModule>,
    pub vs_shader: Option<&'a wgpu::ShaderModule>,
    pub fs_shader: Option<&'a wgpu::ShaderModule>,
    pub vertex_buffer_layout: &'a[wgpu::VertexBufferLayout<'a>],
    pub pipeline_layout: Option<&'a wgpu::PipelineLayout>,
    pub topology: wgpu::PrimitiveTopology,
    pub strip_index_format: Option<wgpu::IndexFormat>,
    pub cull_mode: Option<wgpu::Face>,
    pub is_depth_stencil: bool,
    pub vs_entry: String,
    pub fs_entry: String,
}

impl Default for IRenderPipeline<'_> {
    fn default() -> Self {
        Self {
            shader: None,
            vs_shader: None,
            fs_shader: None,
            vertex_buffer_layout: &[],
            pipeline_layout: None,
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            cull_mode: None,
            is_depth_stencil: true,
            vs_entry: String::from("vs_main"),
            fs_entry: String::from("fs_main"),
        }
    }
}

impl IRenderPipeline<'_> {
    pub fn new(&mut self, init: &IWgpuInit) -> wgpu::RenderPipeline {
        if self.shader.is_some() {
            self.vs_shader = self.shader;
            self.fs_shader = self.shader;
        }

        let mut depth_stencil: Option<wgpu::DepthStencilState> = None;
        if self.is_depth_stencil {
            depth_stencil = Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            });
        }

        init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&self.pipeline_layout.unwrap()),
            vertex: wgpu::VertexState {
                module: &self.vs_shader.as_ref().unwrap(),
                entry_point: Some(&self.vs_entry),
                buffers: &self.vertex_buffer_layout,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.fs_shader.as_ref().unwrap(),
                entry_point: Some(&self.fs_entry),
                targets: &[Some(init.config.format.into())],
                 compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: self.topology,
                strip_index_format: self.strip_index_format,
                ..Default::default()
            },
            depth_stencil,
            multisample: wgpu::MultisampleState {
                count: init.sample_count,
                ..Default::default()
            },
            multiview: None,
            cache: None,
        })
    }
}

pub fn create_color_attachment<'a>(
    texture_view: &'a wgpu::TextureView,
) -> wgpu::RenderPassColorAttachment<'a>{
    wgpu::RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            store: wgpu::StoreOp::Store,
        },
    }
}

pub fn create_msaa_texture_view(init: &IWgpuInit) -> wgpu::TextureView {
    let msaa_texture = init.device.create_texture(
        &wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: init.config.width,
                height: init.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: init.sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: init.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        }
    );

    msaa_texture.create_view(&wgpu::TextureViewDescriptor::default())

}

pub fn create_msaa_color_attachement<'a>(texture_view: &'a wgpu::TextureView,
mssa_view: &'a wgpu::TextureView) -> wgpu::RenderPassColorAttachment<'a> {
    wgpu::RenderPassColorAttachment {
        view: mssa_view,
        resolve_target: Some(texture_view),
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            store: wgpu::StoreOp::Store,
        },
    }
}

pub fn create_depth_view(init: &IWgpuInit) -> wgpu::TextureView {
    let depth_texture = init.device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: init.config.width,
            height: init.config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count:init.sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth24Plus,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        label: None,
        view_formats: &[],
    });
    depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
}

pub fn create_depth_stencil_attachment<'a>(depth_view: &'a  wgpu::TextureView) -> wgpu::RenderPassDepthStencilAttachment<'a> {
    wgpu::RenderPassDepthStencilAttachment {
        view: depth_view,
        depth_ops: Some(
            wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }
        ),
        stencil_ops: None,
    }
}

