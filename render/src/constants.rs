use super::storage::{Mesh, Vertex};

pub const TEST_QUAD_VERTS: [Vertex; 4] = [
    Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0], [1.0, 1.0, 1.0]),
    Vertex::new([1.0, 0.0, 0.0], [1.0, 0.0], [1.0, 1.0, 1.0]),
    Vertex::new([0.0, 1.0, 0.0], [0.0, 1.0], [1.0, 1.0, 1.0]),
    Vertex::new([1.0, 1.0, 0.0], [1.0, 1.0], [1.0, 1.0, 1.0]),
];

pub const TEST_INDEX: [u16; 6] = [0, 1, 2, 2, 1, 3];

pub static TEST_QUAD: std::sync::LazyLock<Mesh> =
    std::sync::LazyLock::new(|| Mesh::new(TEST_QUAD_VERTS.to_vec(), Some(TEST_INDEX.to_vec())));
