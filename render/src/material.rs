use wgpu::util::DeviceExt;

use crate::storage::Rid;

pub struct Material {
    pipeline: Rid<wgpu::RenderPipeline>,
    bind_group: wgpu::BindGroup,
}
impl Material {
    #[must_use]
    pub const fn new(pipeline: Rid<wgpu::RenderPipeline>, bind_group: wgpu::BindGroup) -> Self {
        Self {
            pipeline,
            bind_group,
        }
    }
    #[must_use]
    pub const fn pipeline(&self) -> Rid<wgpu::RenderPipeline> {
        self.pipeline
    }
    pub fn setup(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
    }
}

pub const UNLIT_BIND_GROUP_LAYOUT_DESCRIPTOR: wgpu::BindGroupLayoutDescriptor<'_> =
    wgpu::BindGroupLayoutDescriptor {
        label: Some("Unlit Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    };
#[must_use]
pub fn unlit_material(
    render_server: &mut crate::Server,
    albedo: &crate::texture::Texture,
    modulate: [f32; 4],
) -> Rid<Material> {
    let layout = &render_server
        .device
        .create_bind_group_layout(&UNLIT_BIND_GROUP_LAYOUT_DESCRIPTOR);

    let modulate = render_server
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Modulate Buffer"),
            contents: bytemuck::cast_slice(&[modulate]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

    let bind_group = render_server
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Unlit Bind Group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(albedo.sampler()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(albedo.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: modulate.as_entire_binding(),
                },
            ],
        });
    let pipeline = render_server.standard_shader_unlit();
    render_server.create_material(Material {
        pipeline,
        bind_group,
    })
}
