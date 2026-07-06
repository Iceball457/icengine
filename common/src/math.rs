#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix(pub glam::Mat4);

unsafe impl bytemuck::Pod for Matrix {}
unsafe impl bytemuck::Zeroable for Matrix {}
