use crate::data::Vertex;
use crate::storage::Mesh;

pub const TEST_QUAD_VERTS: [Vertex; 4] = [
    Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0], [1.0, 1.0, 1.0]),
    Vertex::new([1.0, 0.0, 0.0], [1.0, 0.0], [1.0, 1.0, 1.0]),
    Vertex::new([0.0, 1.0, 0.0], [0.0, 1.0], [1.0, 1.0, 1.0]),
    Vertex::new([1.0, 1.0, 0.0], [1.0, 1.0], [1.0, 1.0, 1.0]),
];

pub const TEST_INDEX: [u16; 6] = [0, 1, 2, 2, 1, 3];

#[must_use]
pub fn test_quad() -> Mesh {
    Mesh::new(TEST_QUAD_VERTS.to_vec(), Some(TEST_INDEX.to_vec()))
}
