pub type FixedSize = glam::UVec2;
pub type DynamicSize = glam::Vec2;

pub enum Size {
    Fixed(FixedSize),
    Dynamic(DynamicSize),
}
