use common::math::Matrix;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    view: glam::Mat4,
    proj: glam::Mat4,
}

impl Camera {
    #[must_use]
    pub fn new(aspect: f32) -> Self {
        Self {
            view: View {
                eye: glam::Vec3::ZERO,
                dir: glam::Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                up: glam::Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            }
            .into(),
            proj: Projection::default().into_mat4(aspect),
        }
    }
    pub fn set_view(&mut self, view: View) {
        self.view = view.into();
    }

    pub fn set_projection(&mut self, proj: Projection, aspect: f32) {
        self.proj = proj.into_mat4(aspect);
    }

    #[must_use]
    pub fn to_gpu(&self) -> [u8; size_of::<Matrix>()] {
        self.into()
    }
}

impl From<&Camera> for [u8; size_of::<Matrix>()] {
    fn from(value: &Camera) -> Self {
        bytemuck::cast(Matrix(value.proj * value.view))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub eye: glam::Vec3,
    pub dir: glam::Vec3,
    pub up: glam::Vec3,
}

impl From<View> for glam::Mat4 {
    fn from(value: View) -> Self {
        let View { eye, dir, up } = value;
        glam::camera::rh::view::look_to_mat4(eye, dir, up)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Projection {
    Perspective(Perspective),
    Orthogonal(Orthogonal),
}

impl Projection {
    fn into_mat4(self, aspect: f32) -> glam::Mat4 {
        match self {
            Self::Perspective(perspective) => perspective.into_mat4(aspect),
            Self::Orthogonal(orthogonal) => orthogonal.into_mat4(aspect),
        }
    }
}

impl Default for Projection {
    fn default() -> Self {
        Self::Perspective(Perspective {
            fovy: 90.0,
            near: 0.01,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Perspective {
    pub fovy: f32,
    pub near: f32,
}

impl Perspective {
    fn into_mat4(self, aspect: f32) -> glam::Mat4 {
        let Self { fovy, near } = self;
        glam::camera::rh::proj::directx::perspective_infinite(fovy, aspect, near)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Orthogonal {
    pub height: f32,
    pub near: f32,
    pub far: f32,
}

impl Orthogonal {
    fn into_mat4(self, aspect: f32) -> glam::Mat4 {
        let Self { height, near, far } = self;
        let width = height * aspect;
        let right = width / 2.0;
        let left = -right;
        let top = height / 2.0;
        let bottom = -top;
        glam::camera::rh::proj::directx::orthographic(left, right, bottom, top, near, far)
    }
}

pub static BIND_GROUP_LAYOUT_DESCRIPTOR: wgpu::BindGroupLayoutDescriptor =
    wgpu::BindGroupLayoutDescriptor {
        label: Some("Camera Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    };
