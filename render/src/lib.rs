pub mod camera;
pub use camera::Camera;
pub mod constants;
pub mod data;
pub mod storage;

const CAMERA_BG_INDEX: u32 = 0;

pub struct Server {
    camera: Camera,
    projection: camera::Projection,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    database: storage::Database,
}

impl std::ops::Deref for Server {
    type Target = storage::Database;

    fn deref(&self) -> &Self::Target {
        &self.database
    }
}

impl std::ops::DerefMut for Server {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.database
    }
}

impl Server {
    /// # Errors
    ///
    /// Errors if the adapter or device cannot be aqcuired.
    pub async fn new(
        window: impl Into<wgpu::SurfaceTarget<'static>>,
        size: common::texture::FixedSize,
    ) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::default(),
            display: None,
        });
        let surface = instance.create_surface(window)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::defaults(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;
        let surface_capabilities = surface.get_capabilities(&adapter);
        let format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.x,
            height: size.y,
            present_mode: surface_capabilities.present_modes[0],
            desired_maximum_frame_latency: 0,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        #[allow(clippy::cast_precision_loss)]
        let aspect = config.width as f32 / config.height as f32;
        let camera = Camera::new(aspect);
        let camera_buffer = {
            use wgpu::util::DeviceExt;
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: &camera.to_gpu(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
        };
        let camera_bind_group_layout =
            device.create_bind_group_layout(&camera::BIND_GROUP_LAYOUT_DESCRIPTOR);
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });
        Ok(Self {
            camera,
            projection: camera::Projection::default(),
            camera_buffer,
            camera_bind_group,
            surface,
            device,
            queue,
            config,
            database: storage::Database::new(),
        })
    }

    pub const fn database_mut(&mut self) -> &mut storage::Database {
        &mut self.database
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn aspect(&self) -> f32 {
        self.config.width as f32 / self.config.height as f32
    }

    pub fn resize(&mut self, size: common::texture::FixedSize) {
        if size.x > 0 && size.y > 0 {
            self.config.width = size.x;
            self.config.height = size.y;
            self.configure_surface();
            self.camera.set_projection(self.projection, self.aspect());
        }
    }

    pub fn camera_set_view(&mut self, view: camera::View) {
        self.camera.set_view(view);
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn camera_set_projection(&mut self, proj: camera::Projection) {
        let aspect = self.config.height as f32 / self.config.width as f32;
        self.camera.set_projection(proj, aspect);
    }

    /// # Errors
    ///
    /// Bails if the device has been lost!
    pub fn render(&mut self) -> anyhow::Result<()> {
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                self.configure_surface();
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost device");
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        // Write to the global uniform buffers with this frame's info
        self.queue
            .write_buffer(&self.camera_buffer, 0, &self.camera.to_gpu());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
            render_pass.set_bind_group(CAMERA_BG_INDEX, &self.camera_bind_group, &[]);
            self.draw_objects(&mut render_pass);
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
        Ok(())
    }

    fn draw_objects(&self, render_pass: &mut wgpu::RenderPass) {
        println!(
            "There are {} objects to be drawn.",
            self.database.instances_iter().count()
        );
        for instance in self.database.instances_iter() {
            println!("{instance:#?}");
            let model = self.database.get_model(instance.model());
            let mesh = self.database.get_mesh(model.mesh());
            let pipeline = self.database.get_pipeline(model.pipeline());
            render_pass.set_pipeline(pipeline);
            let vertex_buffer = {
                use wgpu::util::{BufferInitDescriptor, DeviceExt};
                self.device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(mesh.vertices()),
                    usage: wgpu::BufferUsages::VERTEX,
                })
            };
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            let instance_buffer = {
                use wgpu::util::{BufferInitDescriptor, DeviceExt};
                self.device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: bytemuck::cast_slice(instance.transforms()),
                    usage: wgpu::BufferUsages::VERTEX,
                })
            };
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            match mesh.indices() {
                Some(indices) => {
                    let index_buffer = {
                        use wgpu::util::{BufferInitDescriptor, DeviceExt};
                        self.device.create_buffer_init(&BufferInitDescriptor {
                            label: Some("Index Buffer"),
                            contents: bytemuck::cast_slice(indices),
                            usage: wgpu::BufferUsages::INDEX,
                        })
                    };
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    #[allow(clippy::cast_possible_truncation)]
                    render_pass.draw_indexed(
                        0..indices.len() as u32,
                        0,
                        0..instance.transforms().len() as u32,
                    );
                }
                #[allow(clippy::cast_possible_truncation)]
                None => render_pass.draw(
                    0..mesh.vertices().len() as u32,
                    0..instance.transforms().len() as u32,
                ),
            }
        }
    }

    fn configure_surface(&self) {
        self.surface.configure(&self.device, &self.config);
    }

    pub fn standard_shader_unlit(&mut self) -> storage::Rid<wgpu::RenderPipeline> {
        let shader = self
            .device
            .create_shader_module(wgpu::include_wgsl!("unlit.wgsl"));
        let camera_layout = self
            .device
            .create_bind_group_layout(&camera::BIND_GROUP_LAYOUT_DESCRIPTOR);
        let layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Standard Unlit Shader Pipeline Layout"),
                bind_group_layouts: &[Some(&camera_layout)],
                immediate_size: 0,
            });
        let pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Standard Unlit Shader Pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_unlit"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[data::VERTEX_BUFFER_LAYOUT, data::INSTANCE_BUFFER_LAYOUT],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_unlit"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::all(),
                    })],
                }),
                multiview_mask: None,
                cache: None,
            });
        self.database.create_pipeline(pipeline)
    }
}
