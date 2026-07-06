pub static BIND_GROUP_LAYOUT_DESCRIPTOR: wgpu::BindGroupLayoutDescriptor =
    wgpu::BindGroupLayoutDescriptor {
        label: Some("Texture Bind Group Layout Descriptor"),
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
        ],
    };

/// A texture owned by the GPU.
pub struct Texture {
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    pub fn new(
        render_server: &mut crate::Server,
        label: Option<&str>,
        size: (u32, u32),
        data: &[u8],
    ) -> Self {
        use wgpu::util::DeviceExt;
        let texture = render_server.device.create_texture_with_data(
            &render_server.queue,
            &wgpu::TextureDescriptor {
                label,
                size: wgpu::Extent3d {
                    width: size.0,
                    height: size.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::wgt::TextureDataOrder::LayerMajor,
            data,
        );
        let view = texture.create_view(&wgpu::wgt::TextureViewDescriptor::default());
        let sampler = render_server
            .device
            .create_sampler(&wgpu::wgt::SamplerDescriptor {
                label,
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::MipmapFilterMode::Linear,
                lod_min_clamp: 0.0,
                lod_max_clamp: 8.0,
                compare: None,
                anisotropy_clamp: 1,
                border_color: None,
            });
        Self { view, sampler }
    }
    #[must_use]
    pub const fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    #[must_use]
    pub const fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}
